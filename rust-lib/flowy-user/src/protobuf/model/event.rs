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
//! Generated file from `event.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum UserEvent {
    InitUser = 0,
    SignIn = 1,
    SignUp = 2,
    SignOut = 3,
    UpdateUser = 4,
    GetUserProfile = 5,
}

impl ::protobuf::ProtobufEnum for UserEvent {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<UserEvent> {
        match value {
            0 => ::std::option::Option::Some(UserEvent::InitUser),
            1 => ::std::option::Option::Some(UserEvent::SignIn),
            2 => ::std::option::Option::Some(UserEvent::SignUp),
            3 => ::std::option::Option::Some(UserEvent::SignOut),
            4 => ::std::option::Option::Some(UserEvent::UpdateUser),
            5 => ::std::option::Option::Some(UserEvent::GetUserProfile),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [UserEvent] = &[
            UserEvent::InitUser,
            UserEvent::SignIn,
            UserEvent::SignUp,
            UserEvent::SignOut,
            UserEvent::UpdateUser,
            UserEvent::GetUserProfile,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<UserEvent>("UserEvent", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for UserEvent {
}

impl ::std::default::Default for UserEvent {
    fn default() -> Self {
        UserEvent::InitUser
    }
}

impl ::protobuf::reflect::ProtobufValue for UserEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bevent.proto*b\n\tUserEvent\x12\x0c\n\x08InitUser\x10\0\x12\n\n\x06\
    SignIn\x10\x01\x12\n\n\x06SignUp\x10\x02\x12\x0b\n\x07SignOut\x10\x03\
    \x12\x0e\n\nUpdateUser\x10\x04\x12\x12\n\x0eGetUserProfile\x10\x05J\xa0\
    \x02\n\x06\x12\x04\0\0\t\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\
    \x05\0\x12\x04\x02\0\t\x01\n\n\n\x03\x05\0\x01\x12\x03\x02\x05\x0e\n\x0b\
    \n\x04\x05\0\x02\0\x12\x03\x03\x04\x11\n\x0c\n\x05\x05\0\x02\0\x01\x12\
    \x03\x03\x04\x0c\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x03\x0f\x10\n\x0b\n\
    \x04\x05\0\x02\x01\x12\x03\x04\x04\x0f\n\x0c\n\x05\x05\0\x02\x01\x01\x12\
    \x03\x04\x04\n\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x04\r\x0e\n\x0b\n\
    \x04\x05\0\x02\x02\x12\x03\x05\x04\x0f\n\x0c\n\x05\x05\0\x02\x02\x01\x12\
    \x03\x05\x04\n\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x05\r\x0e\n\x0b\n\
    \x04\x05\0\x02\x03\x12\x03\x06\x04\x10\n\x0c\n\x05\x05\0\x02\x03\x01\x12\
    \x03\x06\x04\x0b\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\x06\x0e\x0f\n\x0b\
    \n\x04\x05\0\x02\x04\x12\x03\x07\x04\x13\n\x0c\n\x05\x05\0\x02\x04\x01\
    \x12\x03\x07\x04\x0e\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x07\x11\x12\n\
    \x0b\n\x04\x05\0\x02\x05\x12\x03\x08\x04\x17\n\x0c\n\x05\x05\0\x02\x05\
    \x01\x12\x03\x08\x04\x12\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03\x08\x15\
    \x16b\x06proto3\
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
