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
//! Generated file from `workspace_query.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct QueryWorkspaceRequest {
    // message fields
    pub workspace_id: ::std::string::String,
    pub read_apps: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a QueryWorkspaceRequest {
    fn default() -> &'a QueryWorkspaceRequest {
        <QueryWorkspaceRequest as ::protobuf::Message>::default_instance()
    }
}

impl QueryWorkspaceRequest {
    pub fn new() -> QueryWorkspaceRequest {
        ::std::default::Default::default()
    }

    // string workspace_id = 1;


    pub fn get_workspace_id(&self) -> &str {
        &self.workspace_id
    }
    pub fn clear_workspace_id(&mut self) {
        self.workspace_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_workspace_id(&mut self, v: ::std::string::String) {
        self.workspace_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_workspace_id(&mut self) -> &mut ::std::string::String {
        &mut self.workspace_id
    }

    // Take field
    pub fn take_workspace_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.workspace_id, ::std::string::String::new())
    }

    // bool read_apps = 2;


    pub fn get_read_apps(&self) -> bool {
        self.read_apps
    }
    pub fn clear_read_apps(&mut self) {
        self.read_apps = false;
    }

    // Param is passed by value, moved
    pub fn set_read_apps(&mut self, v: bool) {
        self.read_apps = v;
    }
}

impl ::protobuf::Message for QueryWorkspaceRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.workspace_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.read_apps = tmp;
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
        if !self.workspace_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.workspace_id);
        }
        if self.read_apps != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.workspace_id.is_empty() {
            os.write_string(1, &self.workspace_id)?;
        }
        if self.read_apps != false {
            os.write_bool(2, self.read_apps)?;
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

    fn new() -> QueryWorkspaceRequest {
        QueryWorkspaceRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "workspace_id",
                |m: &QueryWorkspaceRequest| { &m.workspace_id },
                |m: &mut QueryWorkspaceRequest| { &mut m.workspace_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "read_apps",
                |m: &QueryWorkspaceRequest| { &m.read_apps },
                |m: &mut QueryWorkspaceRequest| { &mut m.read_apps },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<QueryWorkspaceRequest>(
                "QueryWorkspaceRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static QueryWorkspaceRequest {
        static instance: ::protobuf::rt::LazyV2<QueryWorkspaceRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(QueryWorkspaceRequest::new)
    }
}

impl ::protobuf::Clear for QueryWorkspaceRequest {
    fn clear(&mut self) {
        self.workspace_id.clear();
        self.read_apps = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryWorkspaceRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryWorkspaceRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15workspace_query.proto\"W\n\x15QueryWorkspaceRequest\x12!\n\x0cwork\
    space_id\x18\x01\x20\x01(\tR\x0bworkspaceId\x12\x1b\n\tread_apps\x18\x02\
    \x20\x01(\x08R\x08readAppsJ\x98\x01\n\x06\x12\x04\0\0\x05\x01\n\x08\n\
    \x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x05\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x02\x08\x1d\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x04\
    \x1c\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\x04\n\n\x0c\n\x05\x04\0\x02\
    \0\x01\x12\x03\x03\x0b\x17\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x1a\
    \x1b\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x04\x17\n\x0c\n\x05\x04\0\x02\
    \x01\x05\x12\x03\x04\x04\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\t\
    \x12\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04\x15\x16b\x06proto3\
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
