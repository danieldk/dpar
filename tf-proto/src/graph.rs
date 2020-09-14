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
pub struct GraphDef {
    // message fields
    pub node: ::protobuf::RepeatedField<super::node_def::NodeDef>,
    pub versions: ::protobuf::SingularPtrField<super::versions::VersionDef>,
    pub version: i32,
    pub library: ::protobuf::SingularPtrField<super::function::FunctionDefLibrary>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GraphDef {}

impl GraphDef {
    pub fn new() -> GraphDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GraphDef {
        static mut instance: ::protobuf::lazy::Lazy<GraphDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GraphDef,
        };
        unsafe {
            instance.get(GraphDef::new)
        }
    }

    // repeated .tensorflow.NodeDef node = 1;

    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    // Param is passed by value, moved
    pub fn set_node(&mut self, v: ::protobuf::RepeatedField<super::node_def::NodeDef>) {
        self.node = v;
    }

    // Mutable pointer to the field.
    pub fn mut_node(&mut self) -> &mut ::protobuf::RepeatedField<super::node_def::NodeDef> {
        &mut self.node
    }

    // Take field
    pub fn take_node(&mut self) -> ::protobuf::RepeatedField<super::node_def::NodeDef> {
        ::std::mem::replace(&mut self.node, ::protobuf::RepeatedField::new())
    }

    pub fn get_node(&self) -> &[super::node_def::NodeDef] {
        &self.node
    }

    fn get_node_for_reflect(&self) -> &::protobuf::RepeatedField<super::node_def::NodeDef> {
        &self.node
    }

    fn mut_node_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::node_def::NodeDef> {
        &mut self.node
    }

    // .tensorflow.VersionDef versions = 4;

    pub fn clear_versions(&mut self) {
        self.versions.clear();
    }

    pub fn has_versions(&self) -> bool {
        self.versions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_versions(&mut self, v: super::versions::VersionDef) {
        self.versions = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_versions(&mut self) -> &mut super::versions::VersionDef {
        if self.versions.is_none() {
            self.versions.set_default();
        }
        self.versions.as_mut().unwrap()
    }

    // Take field
    pub fn take_versions(&mut self) -> super::versions::VersionDef {
        self.versions.take().unwrap_or_else(|| super::versions::VersionDef::new())
    }

    pub fn get_versions(&self) -> &super::versions::VersionDef {
        self.versions.as_ref().unwrap_or_else(|| super::versions::VersionDef::default_instance())
    }

    fn get_versions_for_reflect(&self) -> &::protobuf::SingularPtrField<super::versions::VersionDef> {
        &self.versions
    }

    fn mut_versions_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::versions::VersionDef> {
        &mut self.versions
    }

    // int32 version = 3;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i32) {
        self.version = v;
    }

    pub fn get_version(&self) -> i32 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &i32 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut i32 {
        &mut self.version
    }

    // .tensorflow.FunctionDefLibrary library = 2;

    pub fn clear_library(&mut self) {
        self.library.clear();
    }

    pub fn has_library(&self) -> bool {
        self.library.is_some()
    }

    // Param is passed by value, moved
    pub fn set_library(&mut self, v: super::function::FunctionDefLibrary) {
        self.library = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_library(&mut self) -> &mut super::function::FunctionDefLibrary {
        if self.library.is_none() {
            self.library.set_default();
        }
        self.library.as_mut().unwrap()
    }

    // Take field
    pub fn take_library(&mut self) -> super::function::FunctionDefLibrary {
        self.library.take().unwrap_or_else(|| super::function::FunctionDefLibrary::new())
    }

    pub fn get_library(&self) -> &super::function::FunctionDefLibrary {
        self.library.as_ref().unwrap_or_else(|| super::function::FunctionDefLibrary::default_instance())
    }

    fn get_library_for_reflect(&self) -> &::protobuf::SingularPtrField<super::function::FunctionDefLibrary> {
        &self.library
    }

    fn mut_library_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::function::FunctionDefLibrary> {
        &mut self.library
    }
}

impl ::protobuf::Message for GraphDef {
    fn is_initialized(&self) -> bool {
        for v in &self.node {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.versions {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.library {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.node)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.versions)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.version = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.library)?;
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
        for value in &self.node {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.versions.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(3, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.library.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.node {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.versions.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.version != 0 {
            os.write_int32(3, self.version)?;
        }
        if let Some(ref v) = self.library.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for GraphDef {
    fn new() -> GraphDef {
        GraphDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<GraphDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::node_def::NodeDef>>(
                    "node",
                    GraphDef::get_node_for_reflect,
                    GraphDef::mut_node_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::versions::VersionDef>>(
                    "versions",
                    GraphDef::get_versions_for_reflect,
                    GraphDef::mut_versions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "version",
                    GraphDef::get_version_for_reflect,
                    GraphDef::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::function::FunctionDefLibrary>>(
                    "library",
                    GraphDef::get_library_for_reflect,
                    GraphDef::mut_library_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GraphDef>(
                    "GraphDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GraphDef {
    fn clear(&mut self) {
        self.clear_node();
        self.clear_versions();
        self.clear_version();
        self.clear_library();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GraphDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GraphDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%tensorflow/core/framework/graph.proto\x12\ntensorflow\x1a(tensorflow/\
    core/framework/node_def.proto\x1a(tensorflow/core/framework/function.pro\
    to\x1a(tensorflow/core/framework/versions.proto\"\xbf\x01\n\x08GraphDef\
    \x12'\n\x04node\x18\x01\x20\x03(\x0b2\x13.tensorflow.NodeDefR\x04node\
    \x122\n\x08versions\x18\x04\x20\x01(\x0b2\x16.tensorflow.VersionDefR\x08\
    versions\x12\x1c\n\x07version\x18\x03\x20\x01(\x05R\x07versionB\x02\x18\
    \x01\x128\n\x07library\x18\x02\x20\x01(\x0b2\x1e.tensorflow.FunctionDefL\
    ibraryR\x07libraryB,\n\x18org.tensorflow.frameworkB\x0bGraphProtosP\x01\
    \xf8\x01\x01J\x9d\x12\n\x06\x12\x04\0\07\x02\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\n\x08\n\x01\x08\x12\x03\x03\0\
    \x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\n\x0c\n\x05\x08\xe7\x07\
    \0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x03\x07\
    \x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x03\x07\x17\n\x0c\n\x05\
    \x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\x01\x08\x12\x03\x04\0,\n\
    \x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\0,\n\x0c\n\x05\x08\xe7\x07\x01\
    \x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x04\x07\
    \x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x04\x07\x1b\n\x0c\n\
    \x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e+\n\x08\n\x01\x08\x12\x03\x05\0\
    \"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\x0c\n\x05\x08\xe7\x07\
    \x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x05\
    \x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x05\x07\x1a\n\x0c\
    \n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\n\x01\x08\x12\x03\x06\
    \01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\x0c\n\x05\x08\xe7\x07\
    \x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x06\
    \x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x06\x07\x13\n\x0c\
    \n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\t\n\x02\x03\0\x12\x03\x08\
    \x071\n\t\n\x02\x03\x01\x12\x03\t\x071\n\t\n\x02\x03\x02\x12\x03\n\x071\
    \n0\n\x02\x04\0\x12\x04\r\07\x01\x1a$\x20Represents\x20the\x20graph\x20o\
    f\x20operations\n\n\n\n\x03\x04\0\x01\x12\x03\r\x08\x10\n\x0b\n\x04\x04\
    \0\x02\0\x12\x03\x0e\x02\x1c\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x0e\x02\
    \n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x0e\x0b\x12\n\x0c\n\x05\x04\0\x02\
    \0\x01\x12\x03\x0e\x13\x17\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0e\x1a\
    \x1b\n\xef\x01\n\x04\x04\0\x02\x01\x12\x03\x13\x02\x1a\x1a\xe1\x01\x20Co\
    mpatibility\x20versions\x20of\x20the\x20graph.\x20\x20See\x20core/public\
    /version.h\x20for\x20version\n\x20history.\x20\x20The\x20GraphDef\x20ver\
    sion\x20is\x20distinct\x20from\x20the\x20TensorFlow\x20version,\x20and\n\
    \x20each\x20release\x20of\x20TensorFlow\x20will\x20support\x20a\x20range\
    \x20of\x20GraphDef\x20versions.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\
    \x13\x02\x0e\x1c\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x13\x02\x0c\n\x0c\
    \n\x05\x04\0\x02\x01\x01\x12\x03\x13\r\x15\n\x0c\n\x05\x04\0\x02\x01\x03\
    \x12\x03\x13\x18\x19\n\xc4\x01\n\x04\x04\0\x02\x02\x12\x03\x18\x02(\x1a\
    \xb6\x01\x20Deprecated\x20single\x20version\x20field;\x20use\x20versions\
    \x20above\x20instead.\x20\x20Since\x20all\n\x20GraphDef\x20changes\x20be\
    fore\x20\"versions\"\x20was\x20introduced\x20were\x20forward\n\x20compat\
    ible,\x20this\x20field\x20is\x20entirely\x20ignored.\n\n\r\n\x05\x04\0\
    \x02\x02\x04\x12\x04\x18\x02\x13\x1a\n\x0c\n\x05\x04\0\x02\x02\x05\x12\
    \x03\x18\x02\x07\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x18\x08\x0f\n\x0c\
    \n\x05\x04\0\x02\x02\x03\x12\x03\x18\x12\x13\n\x0c\n\x05\x04\0\x02\x02\
    \x08\x12\x03\x18\x14'\n\x0f\n\x08\x04\0\x02\x02\x08\xe7\x07\0\x12\x03\
    \x18\x15&\n\x10\n\t\x04\0\x02\x02\x08\xe7\x07\0\x02\x12\x03\x18\x15\x1f\
    \n\x11\n\n\x04\0\x02\x02\x08\xe7\x07\0\x02\0\x12\x03\x18\x15\x1f\n\x12\n\
    \x0b\x04\0\x02\x02\x08\xe7\x07\0\x02\0\x01\x12\x03\x18\x15\x1f\n\x10\n\t\
    \x04\0\x02\x02\x08\xe7\x07\0\x03\x12\x03\x18\"&\n\xc4\x08\n\x04\x04\0\
    \x02\x03\x12\x036\x02!\x1a\xb6\x08\x20EXPERIMENTAL.\x20DO\x20NOT\x20USE\
    \x20OR\x20DEPEND\x20ON\x20THIS\x20YET.\n\n\x20\"library\"\x20provides\
    \x20user-defined\x20functions.\n\n\x20Naming:\n\x20\x20\x20*\x20library.\
    function.name\x20are\x20in\x20a\x20flat\x20namespace.\n\x20\x20\x20\x20\
    \x20NOTE:\x20We\x20may\x20need\x20to\x20change\x20it\x20to\x20be\x20hier\
    archical\x20to\x20support\n\x20\x20\x20\x20\x20different\x20orgs.\x20E.g\
    .,\n\x20\x20\x20\x20\x20{\x20\"/google/nn\",\x20{\x20...\x20}},\n\x20\
    \x20\x20\x20\x20{\x20\"/google/vision\",\x20{\x20...\x20}}\n\x20\x20\x20\
    \x20\x20{\x20\"/org_foo/module_bar\",\x20{\x20...\x20}}\n\x20\x20\x20\
    \x20\x20map<string,\x20FunctionDefLib>\x20named_lib;\n\x20\x20\x20*\x20I\
    f\x20node[i].op\x20is\x20the\x20name\x20of\x20one\x20function\x20in\x20\
    \"library\",\n\x20\x20\x20\x20\x20node[i]\x20is\x20deemed\x20as\x20a\x20\
    function\x20call.\x20Otherwise,\x20node[i].op\n\x20\x20\x20\x20\x20must\
    \x20be\x20a\x20primitive\x20operation\x20supported\x20by\x20the\x20runti\
    me.\n\n\n\x20Function\x20call\x20semantics:\n\n\x20\x20\x20*\x20The\x20c\
    allee\x20may\x20start\x20execution\x20as\x20soon\x20as\x20some\x20of\x20\
    its\x20inputs\n\x20\x20\x20\x20\x20are\x20ready.\x20The\x20caller\x20may\
    \x20want\x20to\x20use\x20Tuple()\x20mechanism\x20to\n\x20\x20\x20\x20\
    \x20ensure\x20all\x20inputs\x20are\x20ready\x20in\x20the\x20same\x20time\
    .\n\n\x20\x20\x20*\x20The\x20consumer\x20of\x20return\x20values\x20may\
    \x20start\x20executing\x20as\x20soon\x20as\n\x20\x20\x20\x20\x20the\x20r\
    eturn\x20values\x20the\x20consumer\x20depends\x20on\x20are\x20ready.\x20\
    \x20The\n\x20\x20\x20\x20\x20consumer\x20may\x20want\x20to\x20use\x20Tup\
    le()\x20mechanism\x20to\x20ensure\x20the\n\x20\x20\x20\x20\x20consumer\
    \x20does\x20not\x20start\x20until\x20all\x20return\x20values\x20of\x20th\
    e\x20callee\n\x20\x20\x20\x20\x20function\x20are\x20ready.\n\n\r\n\x05\
    \x04\0\x02\x03\x04\x12\x046\x02\x18(\n\x0c\n\x05\x04\0\x02\x03\x06\x12\
    \x036\x02\x14\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x036\x15\x1c\n\x0c\n\x05\
    \x04\0\x02\x03\x03\x12\x036\x1f\x20b\x06proto3\
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
