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
pub struct UserError {
    // message fields
    pub code: UserErrCode,
    pub msg: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a UserError {
    fn default() -> &'a UserError {
        <UserError as ::protobuf::Message>::default_instance()
    }
}

impl UserError {
    pub fn new() -> UserError {
        ::std::default::Default::default()
    }

    // .UserErrCode code = 1;


    pub fn get_code(&self) -> UserErrCode {
        self.code
    }
    pub fn clear_code(&mut self) {
        self.code = UserErrCode::Unknown;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: UserErrCode) {
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

impl ::protobuf::Message for UserError {
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
        if self.code != UserErrCode::Unknown {
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
        if self.code != UserErrCode::Unknown {
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

    fn new() -> UserError {
        UserError::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<UserErrCode>>(
                "code",
                |m: &UserError| { &m.code },
                |m: &mut UserError| { &mut m.code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "msg",
                |m: &UserError| { &m.msg },
                |m: &mut UserError| { &mut m.msg },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<UserError>(
                "UserError",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static UserError {
        static instance: ::protobuf::rt::LazyV2<UserError> = ::protobuf::rt::LazyV2::INIT;
        instance.get(UserError::new)
    }
}

impl ::protobuf::Clear for UserError {
    fn clear(&mut self) {
        self.code = UserErrCode::Unknown;
        self.msg.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserError {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum UserErrCode {
    Unknown = 0,
    UserDatabaseInitFailed = 1,
    UserDatabaseWriteLocked = 2,
    UserDatabaseReadLocked = 3,
    UserDatabaseDidNotMatch = 4,
    UserDatabaseInternalError = 5,
    SqlInternalError = 6,
    UserNotLoginYet = 10,
    ReadCurrentIdFailed = 11,
    WriteCurrentIdFailed = 12,
    EmailIsEmpty = 20,
    EmailFormatInvalid = 21,
    EmailAlreadyExists = 22,
    PasswordIsEmpty = 30,
    PasswordTooLong = 31,
    PasswordContainsForbidCharacters = 32,
    PasswordFormatInvalid = 33,
    UserNameTooLong = 40,
    UserNameContainsForbiddenCharacters = 41,
    UserNameIsEmpty = 42,
    UserWorkspaceInvalid = 50,
    UserIdInvalid = 51,
    CreateDefaultWorkspaceFailed = 52,
    DefaultWorkspaceAlreadyExist = 53,
    ServerError = 100,
}

impl ::protobuf::ProtobufEnum for UserErrCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UserErrCode> {
        match value {
            0 => ::std::option::Option::Some(UserErrCode::Unknown),
            1 => ::std::option::Option::Some(UserErrCode::UserDatabaseInitFailed),
            2 => ::std::option::Option::Some(UserErrCode::UserDatabaseWriteLocked),
            3 => ::std::option::Option::Some(UserErrCode::UserDatabaseReadLocked),
            4 => ::std::option::Option::Some(UserErrCode::UserDatabaseDidNotMatch),
            5 => ::std::option::Option::Some(UserErrCode::UserDatabaseInternalError),
            6 => ::std::option::Option::Some(UserErrCode::SqlInternalError),
            10 => ::std::option::Option::Some(UserErrCode::UserNotLoginYet),
            11 => ::std::option::Option::Some(UserErrCode::ReadCurrentIdFailed),
            12 => ::std::option::Option::Some(UserErrCode::WriteCurrentIdFailed),
            20 => ::std::option::Option::Some(UserErrCode::EmailIsEmpty),
            21 => ::std::option::Option::Some(UserErrCode::EmailFormatInvalid),
            22 => ::std::option::Option::Some(UserErrCode::EmailAlreadyExists),
            30 => ::std::option::Option::Some(UserErrCode::PasswordIsEmpty),
            31 => ::std::option::Option::Some(UserErrCode::PasswordTooLong),
            32 => ::std::option::Option::Some(UserErrCode::PasswordContainsForbidCharacters),
            33 => ::std::option::Option::Some(UserErrCode::PasswordFormatInvalid),
            40 => ::std::option::Option::Some(UserErrCode::UserNameTooLong),
            41 => ::std::option::Option::Some(UserErrCode::UserNameContainsForbiddenCharacters),
            42 => ::std::option::Option::Some(UserErrCode::UserNameIsEmpty),
            50 => ::std::option::Option::Some(UserErrCode::UserWorkspaceInvalid),
            51 => ::std::option::Option::Some(UserErrCode::UserIdInvalid),
            52 => ::std::option::Option::Some(UserErrCode::CreateDefaultWorkspaceFailed),
            53 => ::std::option::Option::Some(UserErrCode::DefaultWorkspaceAlreadyExist),
            100 => ::std::option::Option::Some(UserErrCode::ServerError),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [UserErrCode] = &[
            UserErrCode::Unknown,
            UserErrCode::UserDatabaseInitFailed,
            UserErrCode::UserDatabaseWriteLocked,
            UserErrCode::UserDatabaseReadLocked,
            UserErrCode::UserDatabaseDidNotMatch,
            UserErrCode::UserDatabaseInternalError,
            UserErrCode::SqlInternalError,
            UserErrCode::UserNotLoginYet,
            UserErrCode::ReadCurrentIdFailed,
            UserErrCode::WriteCurrentIdFailed,
            UserErrCode::EmailIsEmpty,
            UserErrCode::EmailFormatInvalid,
            UserErrCode::EmailAlreadyExists,
            UserErrCode::PasswordIsEmpty,
            UserErrCode::PasswordTooLong,
            UserErrCode::PasswordContainsForbidCharacters,
            UserErrCode::PasswordFormatInvalid,
            UserErrCode::UserNameTooLong,
            UserErrCode::UserNameContainsForbiddenCharacters,
            UserErrCode::UserNameIsEmpty,
            UserErrCode::UserWorkspaceInvalid,
            UserErrCode::UserIdInvalid,
            UserErrCode::CreateDefaultWorkspaceFailed,
            UserErrCode::DefaultWorkspaceAlreadyExist,
            UserErrCode::ServerError,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<UserErrCode>("UserErrCode", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for UserErrCode {
}

impl ::std::default::Default for UserErrCode {
    fn default() -> Self {
        UserErrCode::Unknown
    }
}

impl ::protobuf::reflect::ProtobufValue for UserErrCode {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cerrors.proto\"?\n\tUserError\x12\x20\n\x04code\x18\x01\x20\x01(\
    \x0e2\x0c.UserErrCodeR\x04code\x12\x10\n\x03msg\x18\x02\x20\x01(\tR\x03m\
    sg*\x8b\x05\n\x0bUserErrCode\x12\x0b\n\x07Unknown\x10\0\x12\x1a\n\x16Use\
    rDatabaseInitFailed\x10\x01\x12\x1b\n\x17UserDatabaseWriteLocked\x10\x02\
    \x12\x1a\n\x16UserDatabaseReadLocked\x10\x03\x12\x1b\n\x17UserDatabaseDi\
    dNotMatch\x10\x04\x12\x1d\n\x19UserDatabaseInternalError\x10\x05\x12\x14\
    \n\x10SqlInternalError\x10\x06\x12\x13\n\x0fUserNotLoginYet\x10\n\x12\
    \x17\n\x13ReadCurrentIdFailed\x10\x0b\x12\x18\n\x14WriteCurrentIdFailed\
    \x10\x0c\x12\x10\n\x0cEmailIsEmpty\x10\x14\x12\x16\n\x12EmailFormatInval\
    id\x10\x15\x12\x16\n\x12EmailAlreadyExists\x10\x16\x12\x13\n\x0fPassword\
    IsEmpty\x10\x1e\x12\x13\n\x0fPasswordTooLong\x10\x1f\x12$\n\x20PasswordC\
    ontainsForbidCharacters\x10\x20\x12\x19\n\x15PasswordFormatInvalid\x10!\
    \x12\x13\n\x0fUserNameTooLong\x10(\x12'\n#UserNameContainsForbiddenChara\
    cters\x10)\x12\x13\n\x0fUserNameIsEmpty\x10*\x12\x18\n\x14UserWorkspaceI\
    nvalid\x102\x12\x11\n\rUserIdInvalid\x103\x12\x20\n\x1cCreateDefaultWork\
    spaceFailed\x104\x12\x20\n\x1cDefaultWorkspaceAlreadyExist\x105\x12\x0f\
    \n\x0bServerError\x10dJ\xb1\t\n\x06\x12\x04\0\0\x20\x01\n\x08\n\x01\x0c\
    \x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x05\x01\n\n\n\x03\x04\0\
    \x01\x12\x03\x02\x08\x11\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x04\x19\n\
    \x0c\n\x05\x04\0\x02\0\x06\x12\x03\x03\x04\x0f\n\x0c\n\x05\x04\0\x02\0\
    \x01\x12\x03\x03\x10\x14\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x17\x18\
    \n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x04\x13\n\x0c\n\x05\x04\0\x02\x01\
    \x05\x12\x03\x04\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\x0b\x0e\
    \n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04\x11\x12\n\n\n\x02\x05\0\x12\
    \x04\x06\0\x20\x01\n\n\n\x03\x05\0\x01\x12\x03\x06\x05\x10\n\x0b\n\x04\
    \x05\0\x02\0\x12\x03\x07\x04\x10\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x07\
    \x04\x0b\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x07\x0e\x0f\n\x0b\n\x04\x05\
    \0\x02\x01\x12\x03\x08\x04\x1f\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x08\
    \x04\x1a\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x08\x1d\x1e\n\x0b\n\x04\
    \x05\0\x02\x02\x12\x03\t\x04\x20\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\t\
    \x04\x1b\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\t\x1e\x1f\n\x0b\n\x04\x05\
    \0\x02\x03\x12\x03\n\x04\x1f\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03\n\x04\
    \x1a\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\n\x1d\x1e\n\x0b\n\x04\x05\0\
    \x02\x04\x12\x03\x0b\x04\x20\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03\x0b\
    \x04\x1b\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x0b\x1e\x1f\n\x0b\n\x04\
    \x05\0\x02\x05\x12\x03\x0c\x04\"\n\x0c\n\x05\x05\0\x02\x05\x01\x12\x03\
    \x0c\x04\x1d\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03\x0c\x20!\n\x0b\n\x04\
    \x05\0\x02\x06\x12\x03\r\x04\x19\n\x0c\n\x05\x05\0\x02\x06\x01\x12\x03\r\
    \x04\x14\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03\r\x17\x18\n\x0b\n\x04\x05\
    \0\x02\x07\x12\x03\x0e\x04\x19\n\x0c\n\x05\x05\0\x02\x07\x01\x12\x03\x0e\
    \x04\x13\n\x0c\n\x05\x05\0\x02\x07\x02\x12\x03\x0e\x16\x18\n\x0b\n\x04\
    \x05\0\x02\x08\x12\x03\x0f\x04\x1d\n\x0c\n\x05\x05\0\x02\x08\x01\x12\x03\
    \x0f\x04\x17\n\x0c\n\x05\x05\0\x02\x08\x02\x12\x03\x0f\x1a\x1c\n\x0b\n\
    \x04\x05\0\x02\t\x12\x03\x10\x04\x1e\n\x0c\n\x05\x05\0\x02\t\x01\x12\x03\
    \x10\x04\x18\n\x0c\n\x05\x05\0\x02\t\x02\x12\x03\x10\x1b\x1d\n\x0b\n\x04\
    \x05\0\x02\n\x12\x03\x11\x04\x16\n\x0c\n\x05\x05\0\x02\n\x01\x12\x03\x11\
    \x04\x10\n\x0c\n\x05\x05\0\x02\n\x02\x12\x03\x11\x13\x15\n\x0b\n\x04\x05\
    \0\x02\x0b\x12\x03\x12\x04\x1c\n\x0c\n\x05\x05\0\x02\x0b\x01\x12\x03\x12\
    \x04\x16\n\x0c\n\x05\x05\0\x02\x0b\x02\x12\x03\x12\x19\x1b\n\x0b\n\x04\
    \x05\0\x02\x0c\x12\x03\x13\x04\x1c\n\x0c\n\x05\x05\0\x02\x0c\x01\x12\x03\
    \x13\x04\x16\n\x0c\n\x05\x05\0\x02\x0c\x02\x12\x03\x13\x19\x1b\n\x0b\n\
    \x04\x05\0\x02\r\x12\x03\x14\x04\x19\n\x0c\n\x05\x05\0\x02\r\x01\x12\x03\
    \x14\x04\x13\n\x0c\n\x05\x05\0\x02\r\x02\x12\x03\x14\x16\x18\n\x0b\n\x04\
    \x05\0\x02\x0e\x12\x03\x15\x04\x19\n\x0c\n\x05\x05\0\x02\x0e\x01\x12\x03\
    \x15\x04\x13\n\x0c\n\x05\x05\0\x02\x0e\x02\x12\x03\x15\x16\x18\n\x0b\n\
    \x04\x05\0\x02\x0f\x12\x03\x16\x04*\n\x0c\n\x05\x05\0\x02\x0f\x01\x12\
    \x03\x16\x04$\n\x0c\n\x05\x05\0\x02\x0f\x02\x12\x03\x16')\n\x0b\n\x04\
    \x05\0\x02\x10\x12\x03\x17\x04\x1f\n\x0c\n\x05\x05\0\x02\x10\x01\x12\x03\
    \x17\x04\x19\n\x0c\n\x05\x05\0\x02\x10\x02\x12\x03\x17\x1c\x1e\n\x0b\n\
    \x04\x05\0\x02\x11\x12\x03\x18\x04\x19\n\x0c\n\x05\x05\0\x02\x11\x01\x12\
    \x03\x18\x04\x13\n\x0c\n\x05\x05\0\x02\x11\x02\x12\x03\x18\x16\x18\n\x0b\
    \n\x04\x05\0\x02\x12\x12\x03\x19\x04-\n\x0c\n\x05\x05\0\x02\x12\x01\x12\
    \x03\x19\x04'\n\x0c\n\x05\x05\0\x02\x12\x02\x12\x03\x19*,\n\x0b\n\x04\
    \x05\0\x02\x13\x12\x03\x1a\x04\x19\n\x0c\n\x05\x05\0\x02\x13\x01\x12\x03\
    \x1a\x04\x13\n\x0c\n\x05\x05\0\x02\x13\x02\x12\x03\x1a\x16\x18\n\x0b\n\
    \x04\x05\0\x02\x14\x12\x03\x1b\x04\x1e\n\x0c\n\x05\x05\0\x02\x14\x01\x12\
    \x03\x1b\x04\x18\n\x0c\n\x05\x05\0\x02\x14\x02\x12\x03\x1b\x1b\x1d\n\x0b\
    \n\x04\x05\0\x02\x15\x12\x03\x1c\x04\x17\n\x0c\n\x05\x05\0\x02\x15\x01\
    \x12\x03\x1c\x04\x11\n\x0c\n\x05\x05\0\x02\x15\x02\x12\x03\x1c\x14\x16\n\
    \x0b\n\x04\x05\0\x02\x16\x12\x03\x1d\x04&\n\x0c\n\x05\x05\0\x02\x16\x01\
    \x12\x03\x1d\x04\x20\n\x0c\n\x05\x05\0\x02\x16\x02\x12\x03\x1d#%\n\x0b\n\
    \x04\x05\0\x02\x17\x12\x03\x1e\x04&\n\x0c\n\x05\x05\0\x02\x17\x01\x12\
    \x03\x1e\x04\x20\n\x0c\n\x05\x05\0\x02\x17\x02\x12\x03\x1e#%\n\x0b\n\x04\
    \x05\0\x02\x18\x12\x03\x1f\x04\x16\n\x0c\n\x05\x05\0\x02\x18\x01\x12\x03\
    \x1f\x04\x0f\n\x0c\n\x05\x05\0\x02\x18\x02\x12\x03\x1f\x12\x15b\x06proto\
    3\
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
