// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct ResourceHandleProto {
    // message fields
    pub device: ::std::string::String,
    pub container: ::std::string::String,
    pub name: ::std::string::String,
    pub hash_code: u64,
    pub maybe_type_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResourceHandleProto {}

impl ResourceHandleProto {
    pub fn new() -> ResourceHandleProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResourceHandleProto {
        static mut instance: ::protobuf::lazy::Lazy<ResourceHandleProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResourceHandleProto,
        };
        unsafe {
            instance.get(ResourceHandleProto::new)
        }
    }

    // string device = 1;

    pub fn clear_device(&mut self) {
        self.device.clear();
    }

    // Param is passed by value, moved
    pub fn set_device(&mut self, v: ::std::string::String) {
        self.device = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_device(&mut self) -> &mut ::std::string::String {
        &mut self.device
    }

    // Take field
    pub fn take_device(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.device, ::std::string::String::new())
    }

    pub fn get_device(&self) -> &str {
        &self.device
    }

    fn get_device_for_reflect(&self) -> &::std::string::String {
        &self.device
    }

    fn mut_device_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.device
    }

    // string container = 2;

    pub fn clear_container(&mut self) {
        self.container.clear();
    }

    // Param is passed by value, moved
    pub fn set_container(&mut self, v: ::std::string::String) {
        self.container = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_container(&mut self) -> &mut ::std::string::String {
        &mut self.container
    }

    // Take field
    pub fn take_container(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.container, ::std::string::String::new())
    }

    pub fn get_container(&self) -> &str {
        &self.container
    }

    fn get_container_for_reflect(&self) -> &::std::string::String {
        &self.container
    }

    fn mut_container_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.container
    }

    // string name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // uint64 hash_code = 4;

    pub fn clear_hash_code(&mut self) {
        self.hash_code = 0;
    }

    // Param is passed by value, moved
    pub fn set_hash_code(&mut self, v: u64) {
        self.hash_code = v;
    }

    pub fn get_hash_code(&self) -> u64 {
        self.hash_code
    }

    fn get_hash_code_for_reflect(&self) -> &u64 {
        &self.hash_code
    }

    fn mut_hash_code_for_reflect(&mut self) -> &mut u64 {
        &mut self.hash_code
    }

    // string maybe_type_name = 5;

    pub fn clear_maybe_type_name(&mut self) {
        self.maybe_type_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_maybe_type_name(&mut self, v: ::std::string::String) {
        self.maybe_type_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_maybe_type_name(&mut self) -> &mut ::std::string::String {
        &mut self.maybe_type_name
    }

    // Take field
    pub fn take_maybe_type_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.maybe_type_name, ::std::string::String::new())
    }

    pub fn get_maybe_type_name(&self) -> &str {
        &self.maybe_type_name
    }

    fn get_maybe_type_name_for_reflect(&self) -> &::std::string::String {
        &self.maybe_type_name
    }

    fn mut_maybe_type_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.maybe_type_name
    }
}

impl ::protobuf::Message for ResourceHandleProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.device)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.container)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.hash_code = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.maybe_type_name)?;
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
        if !self.device.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.device);
        }
        if !self.container.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.container);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.name);
        }
        if self.hash_code != 0 {
            my_size += ::protobuf::rt::value_size(4, self.hash_code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.maybe_type_name.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.maybe_type_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.device.is_empty() {
            os.write_string(1, &self.device)?;
        }
        if !self.container.is_empty() {
            os.write_string(2, &self.container)?;
        }
        if !self.name.is_empty() {
            os.write_string(3, &self.name)?;
        }
        if self.hash_code != 0 {
            os.write_uint64(4, self.hash_code)?;
        }
        if !self.maybe_type_name.is_empty() {
            os.write_string(5, &self.maybe_type_name)?;
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
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResourceHandleProto {
    fn new() -> ResourceHandleProto {
        ResourceHandleProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResourceHandleProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "device",
                    ResourceHandleProto::get_device_for_reflect,
                    ResourceHandleProto::mut_device_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "container",
                    ResourceHandleProto::get_container_for_reflect,
                    ResourceHandleProto::mut_container_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ResourceHandleProto::get_name_for_reflect,
                    ResourceHandleProto::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "hash_code",
                    ResourceHandleProto::get_hash_code_for_reflect,
                    ResourceHandleProto::mut_hash_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "maybe_type_name",
                    ResourceHandleProto::get_maybe_type_name_for_reflect,
                    ResourceHandleProto::mut_maybe_type_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResourceHandleProto>(
                    "ResourceHandleProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResourceHandleProto {
    fn clear(&mut self) {
        self.clear_device();
        self.clear_container();
        self.clear_name();
        self.clear_hash_code();
        self.clear_maybe_type_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResourceHandleProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResourceHandleProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n/tensorflow/core/framework/resource_handle.proto\x12\ntensorflow\"\xa4\
    \x01\n\x13ResourceHandleProto\x12\x16\n\x06device\x18\x01\x20\x01(\tR\
    \x06device\x12\x1c\n\tcontainer\x18\x02\x20\x01(\tR\tcontainer\x12\x12\n\
    \x04name\x18\x03\x20\x01(\tR\x04name\x12\x1b\n\thash_code\x18\x04\x20\
    \x01(\x04R\x08hashCode\x12&\n\x0fmaybe_type_name\x18\x05\x20\x01(\tR\rma\
    ybeTypeNameB/\n\x18org.tensorflow.frameworkB\x0eResourceHandleP\x01\xf8\
    \x01\x01J\xca\t\n\x06\x12\x04\0\0\x1c\x02\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\n\x08\n\x01\x08\x12\x03\x03\0\
    \x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\n\x0c\n\x05\x08\xe7\x07\
    \0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x03\x07\
    \x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x03\x07\x17\n\x0c\n\x05\
    \x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\x01\x08\x12\x03\x04\0/\n\
    \x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\0/\n\x0c\n\x05\x08\xe7\x07\x01\
    \x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x04\x07\
    \x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x04\x07\x1b\n\x0c\n\
    \x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e.\n\x08\n\x01\x08\x12\x03\x05\0\
    \"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\x0c\n\x05\x08\xe7\x07\
    \x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x05\
    \x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x05\x07\x1a\n\x0c\
    \n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\n\x01\x08\x12\x03\x06\
    \01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\x0c\n\x05\x08\xe7\x07\
    \x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x06\
    \x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x06\x07\x13\n\x0c\
    \n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\xb8\x01\n\x02\x04\0\x12\
    \x04\x0b\0\x1c\x01\x1a\xab\x01\x20Protocol\x20buffer\x20representing\x20\
    a\x20handle\x20to\x20a\x20tensorflow\x20resource.\x20Handles\x20are\n\
    \x20not\x20valid\x20across\x20executions,\x20but\x20can\x20be\x20seriali\
    zed\x20back\x20and\x20forth\x20from\x20within\n\x20a\x20single\x20run.\n\
    \n\n\n\x03\x04\0\x01\x12\x03\x0b\x08\x1b\nB\n\x04\x04\0\x02\0\x12\x03\r\
    \x02\x14\x1a5\x20Unique\x20name\x20for\x20the\x20device\x20containing\
    \x20the\x20resource.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\r\x02\x0b\x1d\n\
    \x0c\n\x05\x04\0\x02\0\x05\x12\x03\r\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\
    \x12\x03\r\t\x0f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\r\x12\x13\n:\n\x04\
    \x04\0\x02\x01\x12\x03\x10\x02\x17\x1a-\x20Container\x20in\x20which\x20t\
    his\x20resource\x20is\x20placed.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\
    \x10\x02\r\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x10\x02\x08\n\x0c\n\
    \x05\x04\0\x02\x01\x01\x12\x03\x10\t\x12\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03\x10\x15\x16\n,\n\x04\x04\0\x02\x02\x12\x03\x13\x02\x12\x1a\x1f\
    \x20Unique\x20name\x20of\x20this\x20resource.\n\n\r\n\x05\x04\0\x02\x02\
    \x04\x12\x04\x13\x02\x10\x17\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x13\
    \x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x13\t\r\n\x0c\n\x05\x04\0\
    \x02\x02\x03\x12\x03\x13\x10\x11\ns\n\x04\x04\0\x02\x03\x12\x03\x17\x02\
    \x17\x1af\x20Hash\x20code\x20for\x20the\x20type\x20of\x20the\x20resource\
    .\x20Is\x20only\x20valid\x20in\x20the\x20same\x20device\n\x20and\x20in\
    \x20the\x20same\x20execution.\n\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\x17\
    \x02\x13\x12\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x17\x02\x08\n\x0c\n\
    \x05\x04\0\x02\x03\x01\x12\x03\x17\t\x12\n\x0c\n\x05\x04\0\x02\x03\x03\
    \x12\x03\x17\x15\x16\n]\n\x04\x04\0\x02\x04\x12\x03\x1b\x02\x1d\x1aP\x20\
    For\x20debug-only,\x20the\x20name\x20of\x20the\x20type\x20pointed\x20to\
    \x20by\x20this\x20handle,\x20if\n\x20available.\n\n\r\n\x05\x04\0\x02\
    \x04\x04\x12\x04\x1b\x02\x17\x17\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\
    \x1b\x02\x08\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x1b\t\x18\n\x0c\n\x05\
    \x04\0\x02\x04\x03\x12\x03\x1b\x1b\x1cb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
