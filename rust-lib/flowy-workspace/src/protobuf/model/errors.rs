// This file is generated by rust-protobuf 2.22.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `errors.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct WorkspaceError {
    // message fields
    pub code: ErrorCode,
    pub msg: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a WorkspaceError {
    fn default() -> &'a WorkspaceError {
        <WorkspaceError as ::protobuf::Message>::default_instance()
    }
}

impl WorkspaceError {
    pub fn new() -> WorkspaceError {
        ::std::default::Default::default()
    }

    // .ErrorCode code = 1;


    pub fn get_code(&self) -> ErrorCode {
        self.code
    }
    pub fn clear_code(&mut self) {
        self.code = ErrorCode::Unknown;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: ErrorCode) {
        self.code = v;
    }

    // string msg = 2;


    pub fn get_msg(&self) -> &str {
        &self.msg
    }
    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }
}

impl ::protobuf::Message for WorkspaceError {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.code, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.code != ErrorCode::Unknown {
            my_size += ::protobuf::rt::enum_size(1, self.code);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.msg);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.code != ErrorCode::Unknown {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.code))?;
        }
        if !self.msg.is_empty() {
            os.write_string(2, &self.msg)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> WorkspaceError {
        WorkspaceError::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ErrorCode>>(
                "code",
                |m: &WorkspaceError| { &m.code },
                |m: &mut WorkspaceError| { &mut m.code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "msg",
                |m: &WorkspaceError| { &m.msg },
                |m: &mut WorkspaceError| { &mut m.msg },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<WorkspaceError>(
                "WorkspaceError",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static WorkspaceError {
        static instance: ::protobuf::rt::LazyV2<WorkspaceError> = ::protobuf::rt::LazyV2::INIT;
        instance.get(WorkspaceError::new)
    }
}

impl ::protobuf::Clear for WorkspaceError {
    fn clear(&mut self) {
        self.code = ErrorCode::Unknown;
        self.msg.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WorkspaceError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WorkspaceError {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ErrorCode {
    Unknown = 0,
    WorkspaceNameInvalid = 1,
    WorkspaceIdInvalid = 2,
    AppColorStyleInvalid = 3,
    WorkspaceDescInvalid = 4,
    CurrentWorkspaceNotFound = 5,
    AppIdInvalid = 10,
    AppNameInvalid = 11,
    ViewNameInvalid = 20,
    ViewThumbnailInvalid = 21,
    ViewIdInvalid = 22,
    ViewDescInvalid = 23,
    DatabaseConnectionFail = 100,
    WorkspaceDatabaseError = 101,
    UserIdIsEmpty = 102,
    UserUnauthorized = 103,
    InternalError = 1000,
    RecordNotFound = 1001,
}

impl ::protobuf::ProtobufEnum for ErrorCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ErrorCode> {
        match value {
            0 => ::std::option::Option::Some(ErrorCode::Unknown),
            1 => ::std::option::Option::Some(ErrorCode::WorkspaceNameInvalid),
            2 => ::std::option::Option::Some(ErrorCode::WorkspaceIdInvalid),
            3 => ::std::option::Option::Some(ErrorCode::AppColorStyleInvalid),
            4 => ::std::option::Option::Some(ErrorCode::WorkspaceDescInvalid),
            5 => ::std::option::Option::Some(ErrorCode::CurrentWorkspaceNotFound),
            10 => ::std::option::Option::Some(ErrorCode::AppIdInvalid),
            11 => ::std::option::Option::Some(ErrorCode::AppNameInvalid),
            20 => ::std::option::Option::Some(ErrorCode::ViewNameInvalid),
            21 => ::std::option::Option::Some(ErrorCode::ViewThumbnailInvalid),
            22 => ::std::option::Option::Some(ErrorCode::ViewIdInvalid),
            23 => ::std::option::Option::Some(ErrorCode::ViewDescInvalid),
            100 => ::std::option::Option::Some(ErrorCode::DatabaseConnectionFail),
            101 => ::std::option::Option::Some(ErrorCode::WorkspaceDatabaseError),
            102 => ::std::option::Option::Some(ErrorCode::UserIdIsEmpty),
            103 => ::std::option::Option::Some(ErrorCode::UserUnauthorized),
            1000 => ::std::option::Option::Some(ErrorCode::InternalError),
            1001 => ::std::option::Option::Some(ErrorCode::RecordNotFound),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ErrorCode] = &[
            ErrorCode::Unknown,
            ErrorCode::WorkspaceNameInvalid,
            ErrorCode::WorkspaceIdInvalid,
            ErrorCode::AppColorStyleInvalid,
            ErrorCode::WorkspaceDescInvalid,
            ErrorCode::CurrentWorkspaceNotFound,
            ErrorCode::AppIdInvalid,
            ErrorCode::AppNameInvalid,
            ErrorCode::ViewNameInvalid,
            ErrorCode::ViewThumbnailInvalid,
            ErrorCode::ViewIdInvalid,
            ErrorCode::ViewDescInvalid,
            ErrorCode::DatabaseConnectionFail,
            ErrorCode::WorkspaceDatabaseError,
            ErrorCode::UserIdIsEmpty,
            ErrorCode::UserUnauthorized,
            ErrorCode::InternalError,
            ErrorCode::RecordNotFound,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<ErrorCode>("ErrorCode", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for ErrorCode {
}

impl ::std::default::Default for ErrorCode {
    fn default() -> Self {
        ErrorCode::Unknown
    }
}

impl ::protobuf::reflect::ProtobufValue for ErrorCode {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cerrors.proto\"B\n\x0eWorkspaceError\x12\x1e\n\x04code\x18\x01\x20\
    \x01(\x0e2\n.ErrorCodeR\x04code\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\x03\
    msg*\xa3\x03\n\tErrorCode\x12\x0b\n\x07Unknown\x10\0\x12\x18\n\x14Worksp\
    aceNameInvalid\x10\x01\x12\x16\n\x12WorkspaceIdInvalid\x10\x02\x12\x18\n\
    \x14AppColorStyleInvalid\x10\x03\x12\x18\n\x14WorkspaceDescInvalid\x10\
    \x04\x12\x1c\n\x18CurrentWorkspaceNotFound\x10\x05\x12\x10\n\x0cAppIdInv\
    alid\x10\n\x12\x12\n\x0eAppNameInvalid\x10\x0b\x12\x13\n\x0fViewNameInva\
    lid\x10\x14\x12\x18\n\x14ViewThumbnailInvalid\x10\x15\x12\x11\n\rViewIdI\
    nvalid\x10\x16\x12\x13\n\x0fViewDescInvalid\x10\x17\x12\x1a\n\x16Databas\
    eConnectionFail\x10d\x12\x1a\n\x16WorkspaceDatabaseError\x10e\x12\x11\n\
    \rUserIdIsEmpty\x10f\x12\x14\n\x10UserUnauthorized\x10g\x12\x12\n\rInter\
    nalError\x10\xe8\x07\x12\x13\n\x0eRecordNotFound\x10\xe9\x07J\x92\x07\n\
    \x06\x12\x04\0\0\x19\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\
    \x12\x04\x02\0\x05\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x16\n\x0b\n\
    \x04\x04\0\x02\0\x12\x03\x03\x04\x17\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\
    \x03\x04\r\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x0e\x12\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x03\x15\x16\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \x04\x04\x13\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\x04\n\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\x04\x0b\x0e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\x04\x11\x12\n\n\n\x02\x05\0\x12\x04\x06\0\x19\x01\n\n\n\x03\x05\0\
    \x01\x12\x03\x06\x05\x0e\n\x0b\n\x04\x05\0\x02\0\x12\x03\x07\x04\x10\n\
    \x0c\n\x05\x05\0\x02\0\x01\x12\x03\x07\x04\x0b\n\x0c\n\x05\x05\0\x02\0\
    \x02\x12\x03\x07\x0e\x0f\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x08\x04\x1d\n\
    \x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x08\x04\x18\n\x0c\n\x05\x05\0\x02\
    \x01\x02\x12\x03\x08\x1b\x1c\n\x0b\n\x04\x05\0\x02\x02\x12\x03\t\x04\x1b\
    \n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\t\x04\x16\n\x0c\n\x05\x05\0\x02\
    \x02\x02\x12\x03\t\x19\x1a\n\x0b\n\x04\x05\0\x02\x03\x12\x03\n\x04\x1d\n\
    \x0c\n\x05\x05\0\x02\x03\x01\x12\x03\n\x04\x18\n\x0c\n\x05\x05\0\x02\x03\
    \x02\x12\x03\n\x1b\x1c\n\x0b\n\x04\x05\0\x02\x04\x12\x03\x0b\x04\x1d\n\
    \x0c\n\x05\x05\0\x02\x04\x01\x12\x03\x0b\x04\x18\n\x0c\n\x05\x05\0\x02\
    \x04\x02\x12\x03\x0b\x1b\x1c\n\x0b\n\x04\x05\0\x02\x05\x12\x03\x0c\x04!\
    \n\x0c\n\x05\x05\0\x02\x05\x01\x12\x03\x0c\x04\x1c\n\x0c\n\x05\x05\0\x02\
    \x05\x02\x12\x03\x0c\x1f\x20\n\x0b\n\x04\x05\0\x02\x06\x12\x03\r\x04\x16\
    \n\x0c\n\x05\x05\0\x02\x06\x01\x12\x03\r\x04\x10\n\x0c\n\x05\x05\0\x02\
    \x06\x02\x12\x03\r\x13\x15\n\x0b\n\x04\x05\0\x02\x07\x12\x03\x0e\x04\x18\
    \n\x0c\n\x05\x05\0\x02\x07\x01\x12\x03\x0e\x04\x12\n\x0c\n\x05\x05\0\x02\
    \x07\x02\x12\x03\x0e\x15\x17\n\x0b\n\x04\x05\0\x02\x08\x12\x03\x0f\x04\
    \x19\n\x0c\n\x05\x05\0\x02\x08\x01\x12\x03\x0f\x04\x13\n\x0c\n\x05\x05\0\
    \x02\x08\x02\x12\x03\x0f\x16\x18\n\x0b\n\x04\x05\0\x02\t\x12\x03\x10\x04\
    \x1e\n\x0c\n\x05\x05\0\x02\t\x01\x12\x03\x10\x04\x18\n\x0c\n\x05\x05\0\
    \x02\t\x02\x12\x03\x10\x1b\x1d\n\x0b\n\x04\x05\0\x02\n\x12\x03\x11\x04\
    \x17\n\x0c\n\x05\x05\0\x02\n\x01\x12\x03\x11\x04\x11\n\x0c\n\x05\x05\0\
    \x02\n\x02\x12\x03\x11\x14\x16\n\x0b\n\x04\x05\0\x02\x0b\x12\x03\x12\x04\
    \x19\n\x0c\n\x05\x05\0\x02\x0b\x01\x12\x03\x12\x04\x13\n\x0c\n\x05\x05\0\
    \x02\x0b\x02\x12\x03\x12\x16\x18\n\x0b\n\x04\x05\0\x02\x0c\x12\x03\x13\
    \x04!\n\x0c\n\x05\x05\0\x02\x0c\x01\x12\x03\x13\x04\x1a\n\x0c\n\x05\x05\
    \0\x02\x0c\x02\x12\x03\x13\x1d\x20\n\x0b\n\x04\x05\0\x02\r\x12\x03\x14\
    \x04!\n\x0c\n\x05\x05\0\x02\r\x01\x12\x03\x14\x04\x1a\n\x0c\n\x05\x05\0\
    \x02\r\x02\x12\x03\x14\x1d\x20\n\x0b\n\x04\x05\0\x02\x0e\x12\x03\x15\x04\
    \x18\n\x0c\n\x05\x05\0\x02\x0e\x01\x12\x03\x15\x04\x11\n\x0c\n\x05\x05\0\
    \x02\x0e\x02\x12\x03\x15\x14\x17\n\x0b\n\x04\x05\0\x02\x0f\x12\x03\x16\
    \x04\x1b\n\x0c\n\x05\x05\0\x02\x0f\x01\x12\x03\x16\x04\x14\n\x0c\n\x05\
    \x05\0\x02\x0f\x02\x12\x03\x16\x17\x1a\n\x0b\n\x04\x05\0\x02\x10\x12\x03\
    \x17\x04\x19\n\x0c\n\x05\x05\0\x02\x10\x01\x12\x03\x17\x04\x11\n\x0c\n\
    \x05\x05\0\x02\x10\x02\x12\x03\x17\x14\x18\n\x0b\n\x04\x05\0\x02\x11\x12\
    \x03\x18\x04\x1a\n\x0c\n\x05\x05\0\x02\x11\x01\x12\x03\x18\x04\x12\n\x0c\
    \n\x05\x05\0\x02\x11\x02\x12\x03\x18\x15\x19b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
