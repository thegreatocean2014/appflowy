use crate::{entities::workspace::ViewTable, sqlx_ext::SqlBuilder};
use chrono::Utc;
use flowy_net::errors::{invalid_params, ServerError};
use flowy_workspace::{
    entities::view::parser::ViewId,
    protobuf::{RepeatedView, View, ViewType},
};
use protobuf::ProtobufEnum;
use sqlx::postgres::PgArguments;
use uuid::Uuid;

pub struct Builder {
    table: ViewTable,
}

impl Builder {
    pub fn new(belong_to_id: &str) -> Self {
        let uuid = uuid::Uuid::new_v4();
        let time = Utc::now();

        let table = ViewTable {
            id: uuid,
            belong_to_id: belong_to_id.to_string(),
            name: "".to_string(),
            description: "".to_string(),
            modified_time: time,
            create_time: time,
            thumbnail: "".to_string(),
            view_type: ViewType::Doc.value(),
            is_trash: false,
        };

        Self { table }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.table.name = name.to_string();
        self
    }

    pub fn desc(mut self, desc: &str) -> Self {
        self.table.description = desc.to_owned();
        self
    }

    pub fn thumbnail(mut self, thumbnail: &str) -> Self {
        self.table.thumbnail = thumbnail.to_owned();
        self
    }

    pub fn view_type(mut self, view_type: ViewType) -> Self {
        self.table.view_type = view_type.value();
        self
    }

    pub fn build(self) -> Result<(String, PgArguments, View), ServerError> {
        let view = make_view_from_table(self.table.clone(), RepeatedView::default());

        let (sql, args) = SqlBuilder::create("view_table")
            .add_arg("id", self.table.id)
            .add_arg("belong_to_id", self.table.belong_to_id)
            .add_arg("name", self.table.name)
            .add_arg("description", self.table.description)
            .add_arg("modified_time", self.table.modified_time)
            .add_arg("create_time", self.table.create_time)
            .add_arg("thumbnail", self.table.thumbnail)
            .add_arg("view_type", self.table.view_type)
            .build()?;

        Ok((sql, args, view))
    }
}

pub(crate) fn make_view_from_table(table: ViewTable, views: RepeatedView) -> View {
    let view_type = ViewType::from_i32(table.view_type).unwrap_or(ViewType::Doc);

    let mut view = View::default();
    view.set_id(table.id.to_string());
    view.set_belong_to_id(table.belong_to_id);
    view.set_name(table.name);
    view.set_desc(table.description);
    view.set_view_type(view_type);
    view.set_belongings(views);
    view.set_create_time(table.create_time.timestamp());
    view.set_modified_time(table.modified_time.timestamp());

    view
}

pub(crate) fn check_view_id(id: String) -> Result<Uuid, ServerError> {
    let view_id = ViewId::parse(id).map_err(invalid_params)?;
    let view_id = Uuid::parse_str(view_id.as_ref())?;
    Ok(view_id)
}
