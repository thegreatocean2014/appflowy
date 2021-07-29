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
//! Generated file from `app_query.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct QueryAppRequest {
    // message fields
    pub app_id: ::std::string::String,
    pub read_views: bool,
    pub is_trash: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a QueryAppRequest {
    fn default() -> &'a QueryAppRequest {
        <QueryAppRequest as ::protobuf::Message>::default_instance()
    }
}

impl QueryAppRequest {
    pub fn new() -> QueryAppRequest {
        ::std::default::Default::default()
    }

    // string app_id = 1;


    pub fn get_app_id(&self) -> &str {
        &self.app_id
    }
    pub fn clear_app_id(&mut self) {
        self.app_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_app_id(&mut self, v: ::std::string::String) {
        self.app_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_app_id(&mut self) -> &mut ::std::string::String {
        &mut self.app_id
    }

    // Take field
    pub fn take_app_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.app_id, ::std::string::String::new())
    }

    // bool read_views = 2;


    pub fn get_read_views(&self) -> bool {
        self.read_views
    }
    pub fn clear_read_views(&mut self) {
        self.read_views = false;
    }

    // Param is passed by value, moved
    pub fn set_read_views(&mut self, v: bool) {
        self.read_views = v;
    }

    // bool is_trash = 3;


    pub fn get_is_trash(&self) -> bool {
        self.is_trash
    }
    pub fn clear_is_trash(&mut self) {
        self.is_trash = false;
    }

    // Param is passed by value, moved
    pub fn set_is_trash(&mut self, v: bool) {
        self.is_trash = v;
    }
}

impl ::protobuf::Message for QueryAppRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.app_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.read_views = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_trash = tmp;
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
        if !self.app_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.app_id);
        }
        if self.read_views != false {
            my_size += 2;
        }
        if self.is_trash != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.app_id.is_empty() {
            os.write_string(1, &self.app_id)?;
        }
        if self.read_views != false {
            os.write_bool(2, self.read_views)?;
        }
        if self.is_trash != false {
            os.write_bool(3, self.is_trash)?;
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

    fn new() -> QueryAppRequest {
        QueryAppRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "app_id",
                |m: &QueryAppRequest| { &m.app_id },
                |m: &mut QueryAppRequest| { &mut m.app_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "read_views",
                |m: &QueryAppRequest| { &m.read_views },
                |m: &mut QueryAppRequest| { &mut m.read_views },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "is_trash",
                |m: &QueryAppRequest| { &m.is_trash },
                |m: &mut QueryAppRequest| { &mut m.is_trash },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<QueryAppRequest>(
                "QueryAppRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static QueryAppRequest {
        static instance: ::protobuf::rt::LazyV2<QueryAppRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(QueryAppRequest::new)
    }
}

impl ::protobuf::Clear for QueryAppRequest {
    fn clear(&mut self) {
        self.app_id.clear();
        self.read_views = false;
        self.is_trash = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for QueryAppRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryAppRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fapp_query.proto\"b\n\x0fQueryAppRequest\x12\x15\n\x06app_id\x18\
    \x01\x20\x01(\tR\x05appId\x12\x1d\n\nread_views\x18\x02\x20\x01(\x08R\tr\
    eadViews\x12\x19\n\x08is_trash\x18\x03\x20\x01(\x08R\x07isTrashJ\xcf\x01\
    \n\x06\x12\x04\0\0\x06\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\
    \0\x12\x04\x02\0\x06\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x17\n\x0b\n\
    \x04\x04\0\x02\0\x12\x03\x03\x04\x16\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\
    \x03\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x0b\x11\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x03\x14\x15\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \x04\x04\x18\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\x04\x08\n\x0c\n\
    \x05\x04\0\x02\x01\x01\x12\x03\x04\t\x13\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03\x04\x16\x17\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x05\x04\x16\n\x0c\
    \n\x05\x04\0\x02\x02\x05\x12\x03\x05\x04\x08\n\x0c\n\x05\x04\0\x02\x02\
    \x01\x12\x03\x05\t\x11\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x05\x14\x15\
    b\x06proto3\
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
