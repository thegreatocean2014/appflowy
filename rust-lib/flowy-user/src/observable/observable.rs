use bytes::Bytes;
use flowy_derive::ProtoBuf_Enum;
use flowy_dispatch::prelude::ToBytes;
use flowy_observable::{dart::RustStreamSender, entities::ObservableSubject, ObservableBuilder};

const OBSERVABLE_CATEGORY: &'static str = "User";

#[derive(ProtoBuf_Enum, Debug)]
pub(crate) enum UserObservable {
    Unknown            = 0,
    UserAuthChanged    = 1,
    UserProfileUpdated = 2,
}

impl std::default::Default for UserObservable {
    fn default() -> Self { UserObservable::Unknown }
}

impl std::convert::Into<i32> for UserObservable {
    fn into(self) -> i32 { self as i32 }
}

pub(crate) fn observable(id: &str, ty: UserObservable) -> ObservableBuilder { ObservableBuilder::new(id, ty, OBSERVABLE_CATEGORY) }
