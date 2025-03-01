use crate::{
    errors::*,
    module::{WorkspaceDatabase, WorkspaceUser},
    notify::*,
    services::{read_local_workspace_apps, server::Server, AppController, TrashCan, ViewController},
    sql_tables::workspace::{WorkspaceTable, WorkspaceTableChangeset, WorkspaceTableSql},
};
use chrono::Utc;
use flowy_database::SqliteConnection;
use flowy_document_infra::{entities::doc::DocDelta, user_default::initial_read_me};
use flowy_workspace_infra::{
    entities::{app::RepeatedApp, view::View, workspace::*},
    user_default,
};
use lazy_static::lazy_static;
use lib_infra::{entities::network_state::NetworkType, kv::KV};
use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc};

lazy_static! {
    static ref INIT_WORKSPACE: RwLock<HashMap<String, bool>> = RwLock::new(HashMap::new());
}

pub struct WorkspaceController {
    pub user: Arc<dyn WorkspaceUser>,
    pub(crate) workspace_sql: Arc<WorkspaceTableSql>,
    pub(crate) view_controller: Arc<ViewController>,
    pub(crate) database: Arc<dyn WorkspaceDatabase>,
    pub(crate) app_controller: Arc<AppController>,
    pub(crate) trash_can: Arc<TrashCan>,
    server: Server,
}

impl WorkspaceController {
    pub(crate) fn new(
        user: Arc<dyn WorkspaceUser>,
        database: Arc<dyn WorkspaceDatabase>,
        app_controller: Arc<AppController>,
        view_controller: Arc<ViewController>,
        trash_can: Arc<TrashCan>,
        server: Server,
    ) -> Self {
        if let Ok(token) = user.token() {
            INIT_WORKSPACE.write().insert(token, false);
        }

        let workspace_sql = Arc::new(WorkspaceTableSql {});
        Self {
            user,
            workspace_sql,
            view_controller,
            database,
            app_controller,
            trash_can,
            server,
        }
    }

    async fn init(&self, token: &str) -> Result<(), WorkspaceError> {
        log::debug!("Start initializing workspace");
        if let Some(is_init) = INIT_WORKSPACE.read().get(token) {
            if *is_init {
                return Ok(());
            }
        }
        log::debug!("Finish initializing workspace");
        INIT_WORKSPACE.write().insert(token.to_owned(), true);
        let _ = self.server.init();
        let _ = self.trash_can.init()?;
        let _ = self.view_controller.init()?;
        let _ = self.app_controller.init()?;

        Ok(())
    }

    pub fn network_state_changed(&self, new_type: NetworkType) {
        match new_type {
            NetworkType::UnknownNetworkType => {},
            NetworkType::Wifi => {},
            NetworkType::Cell => {},
            NetworkType::Ethernet => {},
        }
    }

    pub async fn user_did_sign_in(&self, token: &str) -> WorkspaceResult<()> {
        // TODO: (nathan) do something here

        log::debug!("workspace initialize after sign in");
        let _ = self.init(token).await?;
        Ok(())
    }

    pub async fn user_did_logout(&self) {
        // TODO: (nathan) do something here
    }

    pub async fn user_session_expired(&self) {
        // TODO: (nathan) do something here
    }

    pub async fn user_did_sign_up(&self, _token: &str) -> WorkspaceResult<()> {
        log::debug!("Create user default workspace");
        let time = Utc::now();
        let mut workspace = user_default::create_default_workspace(time);
        let apps = workspace.take_apps().into_inner();
        let cloned_workspace = workspace.clone();

        let _ = self.create_workspace(workspace).await?;
        for mut app in apps {
            let views = app.take_belongings().into_inner();
            let _ = self.app_controller.create_app(app).await?;
            for (index, view) in views.into_iter().enumerate() {
                if index == 0 {
                    let delta = initial_read_me();
                    let doc_delta = DocDelta {
                        doc_id: view.id.clone(),
                        data: delta.to_json(),
                    };
                    let _ = self.view_controller.apply_doc_delta(doc_delta).await?;

                    self.view_controller.set_latest_view(&view);
                }
                let _ = self.view_controller.create_view(view).await?;
            }
        }

        let token = self.user.token()?;
        let repeated_workspace = RepeatedWorkspace {
            items: vec![cloned_workspace],
        };

        send_dart_notification(&token, WorkspaceNotification::UserCreateWorkspace)
            .payload(repeated_workspace)
            .send();

        log::debug!("workspace initialize after sign up");
        let _ = self.init(&token).await?;
        Ok(())
    }

    pub(crate) async fn create_workspace_from_params(
        &self,
        params: CreateWorkspaceParams,
    ) -> Result<Workspace, WorkspaceError> {
        let workspace = self.create_workspace_on_server(params.clone()).await?;
        self.create_workspace(workspace).await
    }

    pub(crate) async fn create_workspace(&self, workspace: Workspace) -> Result<Workspace, WorkspaceError> {
        let user_id = self.user.user_id()?;
        let token = self.user.token()?;
        let workspace_table = WorkspaceTable::new(workspace.clone(), &user_id);
        let conn = &*self.database.db_connection()?;
        //[[immediate_transaction]]
        // https://sqlite.org/lang_transaction.html
        // IMMEDIATE cause the database connection to start a new write immediately,
        // without waiting for a write statement. The BEGIN IMMEDIATE might fail
        // with SQLITE_BUSY if another write transaction is already active on another
        // database connection.
        //
        // EXCLUSIVE is similar to IMMEDIATE in that a write transaction is started
        // immediately. EXCLUSIVE and IMMEDIATE are the same in WAL mode, but in
        // other journaling modes, EXCLUSIVE prevents other database connections from
        // reading the database while the transaction is underway.
        conn.immediate_transaction::<_, WorkspaceError, _>(|| {
            self.workspace_sql.create_workspace(workspace_table, conn)?;
            let repeated_workspace = self.read_local_workspaces(None, &user_id, conn)?;
            send_dart_notification(&token, WorkspaceNotification::UserCreateWorkspace)
                .payload(repeated_workspace)
                .send();

            Ok(())
        })?;

        set_current_workspace(&workspace.id);

        Ok(workspace)
    }

    #[allow(dead_code)]
    pub(crate) async fn update_workspace(&self, params: UpdateWorkspaceParams) -> Result<(), WorkspaceError> {
        let changeset = WorkspaceTableChangeset::new(params.clone());
        let workspace_id = changeset.id.clone();
        let conn = &*self.database.db_connection()?;
        conn.immediate_transaction::<_, WorkspaceError, _>(|| {
            let _ = self.workspace_sql.update_workspace(changeset, conn)?;
            let user_id = self.user.user_id()?;
            let workspace = self.read_local_workspace(workspace_id.clone(), &user_id, conn)?;
            send_dart_notification(&workspace_id, WorkspaceNotification::WorkspaceUpdated)
                .payload(workspace)
                .send();

            Ok(())
        })?;

        let _ = self.update_workspace_on_server(params)?;

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) async fn delete_workspace(&self, workspace_id: &str) -> Result<(), WorkspaceError> {
        let user_id = self.user.user_id()?;
        let token = self.user.token()?;
        let conn = &*self.database.db_connection()?;
        conn.immediate_transaction::<_, WorkspaceError, _>(|| {
            let _ = self.workspace_sql.delete_workspace(workspace_id, conn)?;
            let repeated_workspace = self.read_local_workspaces(None, &user_id, conn)?;
            send_dart_notification(&token, WorkspaceNotification::UserDeleteWorkspace)
                .payload(repeated_workspace)
                .send();

            Ok(())
        })?;

        let _ = self.delete_workspace_on_server(workspace_id)?;
        Ok(())
    }

    pub(crate) async fn open_workspace(&self, params: WorkspaceIdentifier) -> Result<Workspace, WorkspaceError> {
        let user_id = self.user.user_id()?;
        let conn = self.database.db_connection()?;
        if let Some(workspace_id) = params.workspace_id {
            let workspace = self.read_local_workspace(workspace_id, &user_id, &*conn)?;
            set_current_workspace(&workspace.id);
            Ok(workspace)
        } else {
            return Err(WorkspaceError::workspace_id().context("Opened workspace id should not be empty"));
        }
    }

    pub(crate) async fn read_workspaces(
        &self,
        params: WorkspaceIdentifier,
    ) -> Result<RepeatedWorkspace, WorkspaceError> {
        let user_id = self.user.user_id()?;
        let workspaces =
            self.read_local_workspaces(params.workspace_id.clone(), &user_id, &*self.database.db_connection()?)?;
        let _ = self.read_workspaces_on_server(user_id, params);
        Ok(workspaces)
    }

    pub(crate) async fn read_current_workspace(&self) -> Result<CurrentWorkspaceSetting, WorkspaceError> {
        let workspace_id = get_current_workspace()?;
        let user_id = self.user.user_id()?;
        let params = WorkspaceIdentifier {
            workspace_id: Some(workspace_id.clone()),
        };
        let workspace = self.read_local_workspace(workspace_id, &user_id, &*self.database.db_connection()?)?;

        let latest_view: Option<View> = self.view_controller.latest_visit_view().unwrap_or(None);
        let setting = CurrentWorkspaceSetting { workspace, latest_view };
        let _ = self.read_workspaces_on_server(user_id, params)?;
        Ok(setting)
    }

    pub(crate) async fn read_current_workspace_apps(&self) -> Result<RepeatedApp, WorkspaceError> {
        let workspace_id = get_current_workspace()?;
        let conn = self.database.db_connection()?;
        let repeated_app = self.read_local_apps(&workspace_id, &*conn)?;
        // TODO: read from server
        Ok(repeated_app)
    }

    #[tracing::instrument(level = "debug", skip(self, conn), err)]
    fn read_local_workspaces(
        &self,
        workspace_id: Option<String>,
        user_id: &str,
        conn: &SqliteConnection,
    ) -> Result<RepeatedWorkspace, WorkspaceError> {
        let workspace_id = workspace_id.to_owned();
        let workspace_tables = self.workspace_sql.read_workspaces(workspace_id, user_id, conn)?;

        let mut workspaces = vec![];
        for table in workspace_tables {
            let apps = self.read_local_apps(&table.id, conn)?.into_inner();
            let mut workspace: Workspace = table.into();
            workspace.apps.items = apps;
            workspaces.push(workspace);
        }
        Ok(RepeatedWorkspace { items: workspaces })
    }

    fn read_local_workspace(
        &self,
        workspace_id: String,
        user_id: &str,
        conn: &SqliteConnection,
    ) -> Result<Workspace, WorkspaceError> {
        // Opti: fetch single workspace from local db
        let mut repeated_workspace = self.read_local_workspaces(Some(workspace_id.clone()), user_id, conn)?;
        if repeated_workspace.is_empty() {
            return Err(WorkspaceError::record_not_found().context(format!("{} workspace not found", workspace_id)));
        }

        debug_assert_eq!(repeated_workspace.len(), 1);
        let workspace = repeated_workspace.drain(..1).collect::<Vec<Workspace>>().pop().unwrap();
        Ok(workspace)
    }

    #[tracing::instrument(level = "debug", skip(self, conn), err)]
    fn read_local_apps(&self, workspace_id: &str, conn: &SqliteConnection) -> Result<RepeatedApp, WorkspaceError> {
        let repeated_app = read_local_workspace_apps(workspace_id, self.trash_can.clone(), conn)?;
        Ok(repeated_app)
    }
}

impl WorkspaceController {
    fn token_with_server(&self) -> Result<(String, Server), WorkspaceError> {
        let token = self.user.token()?;
        let server = self.server.clone();
        Ok((token, server))
    }

    #[tracing::instrument(level = "debug", skip(self), err)]
    async fn create_workspace_on_server(&self, params: CreateWorkspaceParams) -> Result<Workspace, WorkspaceError> {
        let token = self.user.token()?;
        let workspace = self.server.create_workspace(&token, params).await?;
        Ok(workspace)
    }

    #[tracing::instrument(level = "debug", skip(self), err)]
    fn update_workspace_on_server(&self, params: UpdateWorkspaceParams) -> Result<(), WorkspaceError> {
        let (token, server) = self.token_with_server()?;
        tokio::spawn(async move {
            match server.update_workspace(&token, params).await {
                Ok(_) => {},
                Err(e) => {
                    // TODO: retry?
                    log::error!("Update workspace failed: {:?}", e);
                },
            }
        });
        Ok(())
    }

    #[tracing::instrument(level = "debug", skip(self), err)]
    fn delete_workspace_on_server(&self, workspace_id: &str) -> Result<(), WorkspaceError> {
        let params = WorkspaceIdentifier {
            workspace_id: Some(workspace_id.to_string()),
        };
        let (token, server) = self.token_with_server()?;
        tokio::spawn(async move {
            match server.delete_workspace(&token, params).await {
                Ok(_) => {},
                Err(e) => {
                    // TODO: retry?
                    log::error!("Delete workspace failed: {:?}", e);
                },
            }
        });
        Ok(())
    }

    #[tracing::instrument(level = "debug", skip(self), err)]
    fn read_workspaces_on_server(&self, user_id: String, params: WorkspaceIdentifier) -> Result<(), WorkspaceError> {
        let (token, server) = self.token_with_server()?;
        let workspace_sql = self.workspace_sql.clone();
        let app_ctrl = self.app_controller.clone();
        let view_ctrl = self.view_controller.clone();
        let conn = self.database.db_connection()?;
        tokio::spawn(async move {
            // Opti: handle the error and retry?
            let workspaces = server.read_workspace(&token, params).await?;
            let _ = (&*conn).immediate_transaction::<_, WorkspaceError, _>(|| {
                tracing::debug!("Save {} workspace", workspaces.len());
                for workspace in &workspaces.items {
                    let m_workspace = workspace.clone();
                    let apps = m_workspace.apps.clone().into_inner();
                    let workspace_table = WorkspaceTable::new(m_workspace, &user_id);

                    let _ = workspace_sql.create_workspace(workspace_table, &*conn)?;
                    tracing::debug!("Save {} apps", apps.len());
                    for app in apps {
                        let views = app.belongings.clone().into_inner();
                        match app_ctrl.save_app(app, &*conn) {
                            Ok(_) => {},
                            Err(e) => log::error!("create app failed: {:?}", e),
                        }

                        tracing::debug!("Save {} views", views.len());
                        for view in views {
                            match view_ctrl.save_view(view, &*conn) {
                                Ok(_) => {},
                                Err(e) => log::error!("create view failed: {:?}", e),
                            }
                        }
                    }
                }
                Ok(())
            })?;

            send_dart_notification(&token, WorkspaceNotification::WorkspaceListUpdated)
                .payload(workspaces)
                .send();
            Result::<(), WorkspaceError>::Ok(())
        });

        Ok(())
    }
}

const CURRENT_WORKSPACE_ID: &str = "current_workspace_id";

fn set_current_workspace(workspace_id: &str) { KV::set_str(CURRENT_WORKSPACE_ID, workspace_id.to_owned()); }

fn get_current_workspace() -> Result<String, WorkspaceError> {
    match KV::get_str(CURRENT_WORKSPACE_ID) {
        None => Err(WorkspaceError::record_not_found()
            .context("Current workspace not found or should call open workspace first")),
        Some(workspace_id) => Ok(workspace_id),
    }
}
