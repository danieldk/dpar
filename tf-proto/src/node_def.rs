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
pub struct NodeDef {
    // message fields
    pub name: ::std::string::String,
    pub op: ::std::string::String,
    pub input: ::protobuf::RepeatedField<::std::string::String>,
    pub device: ::std::string::String,
    pub attr: ::std::collections::HashMap<::std::string::String, super::attr_value::AttrValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NodeDef {}

impl NodeDef {
    pub fn new() -> NodeDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NodeDef {
        static mut instance: ::protobuf::lazy::Lazy<NodeDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NodeDef,
        };
        unsafe {
            instance.get(NodeDef::new)
        }
    }

    // string name = 1;

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

    // string op = 2;

    pub fn clear_op(&mut self) {
        self.op.clear();
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: ::std::string::String) {
        self.op = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_op(&mut self) -> &mut ::std::string::String {
        &mut self.op
    }

    // Take field
    pub fn take_op(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.op, ::std::string::String::new())
    }

    pub fn get_op(&self) -> &str {
        &self.op
    }

    fn get_op_for_reflect(&self) -> &::std::string::String {
        &self.op
    }

    fn mut_op_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.op
    }

    // repeated string input = 3;

    pub fn clear_input(&mut self) {
        self.input.clear();
    }

    // Param is passed by value, moved
    pub fn set_input(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.input = v;
    }

    // Mutable pointer to the field.
    pub fn mut_input(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.input
    }

    // Take field
    pub fn take_input(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.input, ::protobuf::RepeatedField::new())
    }

    pub fn get_input(&self) -> &[::std::string::String] {
        &self.input
    }

    fn get_input_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.input
    }

    fn mut_input_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.input
    }

    // string device = 4;

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

    // repeated .tensorflow.NodeDef.AttrEntry attr = 5;

    pub fn clear_attr(&mut self) {
        self.attr.clear();
    }

    // Param is passed by value, moved
    pub fn set_attr(&mut self, v: ::std::collections::HashMap<::std::string::String, super::attr_value::AttrValue>) {
        self.attr = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attr(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, super::attr_value::AttrValue> {
        &mut self.attr
    }

    // Take field
    pub fn take_attr(&mut self) -> ::std::collections::HashMap<::std::string::String, super::attr_value::AttrValue> {
        ::std::mem::replace(&mut self.attr, ::std::collections::HashMap::new())
    }

    pub fn get_attr(&self) -> &::std::collections::HashMap<::std::string::String, super::attr_value::AttrValue> {
        &self.attr
    }

    fn get_attr_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, super::attr_value::AttrValue> {
        &self.attr
    }

    fn mut_attr_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, super::attr_value::AttrValue> {
        &mut self.attr
    }
}

impl ::protobuf::Message for NodeDef {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.op)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.input)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.device)?;
                },
                5 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<super::attr_value::AttrValue>>(wire_type, is, &mut self.attr)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.op.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.op);
        }
        for value in &self.input {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if !self.device.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.device);
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<super::attr_value::AttrValue>>(5, &self.attr);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.op.is_empty() {
            os.write_string(2, &self.op)?;
        }
        for v in &self.input {
            os.write_string(3, &v)?;
        };
        if !self.device.is_empty() {
            os.write_string(4, &self.device)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<super::attr_value::AttrValue>>(5, &self.attr, os)?;
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

impl ::protobuf::MessageStatic for NodeDef {
    fn new() -> NodeDef {
        NodeDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<NodeDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    NodeDef::get_name_for_reflect,
                    NodeDef::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "op",
                    NodeDef::get_op_for_reflect,
                    NodeDef::mut_op_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "input",
                    NodeDef::get_input_for_reflect,
                    NodeDef::mut_input_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "device",
                    NodeDef::get_device_for_reflect,
                    NodeDef::mut_device_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<super::attr_value::AttrValue>>(
                    "attr",
                    NodeDef::get_attr_for_reflect,
                    NodeDef::mut_attr_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NodeDef>(
                    "NodeDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NodeDef {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_op();
        self.clear_input();
        self.clear_device();
        self.clear_attr();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NodeDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NodeDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(tensorflow/core/framework/node_def.proto\x12\ntensorflow\x1a*tensorfl\
    ow/core/framework/attr_value.proto\"\xde\x01\n\x07NodeDef\x12\x12\n\x04n\
    ame\x18\x01\x20\x01(\tR\x04name\x12\x0e\n\x02op\x18\x02\x20\x01(\tR\x02o\
    p\x12\x14\n\x05input\x18\x03\x20\x03(\tR\x05input\x12\x16\n\x06device\
    \x18\x04\x20\x01(\tR\x06device\x121\n\x04attr\x18\x05\x20\x03(\x0b2\x1d.\
    tensorflow.NodeDef.AttrEntryR\x04attr\x1aN\n\tAttrEntry\x12\x10\n\x03key\
    \x18\x01\x20\x01(\tR\x03key\x12+\n\x05value\x18\x02\x20\x01(\x0b2\x15.te\
    nsorflow.AttrValueR\x05value:\x028\x01B*\n\x18org.tensorflow.frameworkB\
    \tNodeProtoP\x01\xf8\x01\x01J\xdb\x15\n\x06\x12\x04\0\0>\x02\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\n\x08\n\x01\x08\
    \x12\x03\x03\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\n\x0c\n\
    \x05\x08\xe7\x07\0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\
    \x12\x03\x03\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x03\x07\
    \x17\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\x01\x08\
    \x12\x03\x04\0*\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\0*\n\x0c\n\x05\
    \x08\xe7\x07\x01\x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\
    \x12\x03\x04\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x04\
    \x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e)\n\x08\n\x01\
    \x08\x12\x03\x05\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\x0c\n\
    \x05\x08\xe7\x07\x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\
    \x02\0\x12\x03\x05\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\
    \x05\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\n\
    \x01\x08\x12\x03\x06\01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\x0c\
    \n\x05\x08\xe7\x07\x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\x03\
    \x02\0\x12\x03\x06\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\
    \x06\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\t\n\x02\
    \x03\0\x12\x03\x08\x073\n\n\n\x02\x04\0\x12\x04\n\0>\x01\n\n\n\x03\x04\0\
    \x01\x12\x03\n\x08\x0f\n\xbe\x01\n\x04\x04\0\x02\0\x12\x03\x0e\x02\x12\
    \x1a\xb0\x01\x20The\x20name\x20given\x20to\x20this\x20operator.\x20Used\
    \x20for\x20naming\x20inputs,\n\x20logging,\x20visualization,\x20etc.\x20\
    \x20Unique\x20within\x20a\x20single\x20GraphDef.\n\x20Must\x20match\x20t\
    he\x20regexp\x20\"[A-Za-z0-9.][A-Za-z0-9_./]*\".\n\n\r\n\x05\x04\0\x02\0\
    \x04\x12\x04\x0e\x02\n\x11\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x0e\x02\
    \x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0e\t\r\n\x0c\n\x05\x04\0\x02\0\
    \x03\x12\x03\x0e\x10\x11\n\x92\x01\n\x04\x04\0\x02\x01\x12\x03\x12\x02\
    \x10\x1a\x84\x01\x20The\x20operation\x20name.\x20\x20There\x20may\x20be\
    \x20custom\x20parameters\x20in\x20attrs.\n\x20Op\x20names\x20starting\
    \x20with\x20an\x20underscore\x20are\x20reserved\x20for\x20internal\x20us\
    e.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x12\x02\x0e\x12\n\x0c\n\x05\x04\
    \0\x02\x01\x05\x12\x03\x12\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\
    \x12\t\x0b\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x12\x0e\x0f\n\xa7\x02\n\
    \x04\x04\0\x02\x02\x12\x03\x19\x02\x1c\x1a\x99\x02\x20Each\x20input\x20i\
    s\x20\"node:src_output\"\x20with\x20\"node\"\x20being\x20a\x20string\x20\
    name\x20and\n\x20\"src_output\"\x20indicating\x20which\x20output\x20tens\
    or\x20to\x20use\x20from\x20\"node\".\x20If\n\x20\"src_output\"\x20is\x20\
    0\x20the\x20\":0\"\x20suffix\x20can\x20be\x20omitted.\x20\x20Regular\x20\
    inputs\n\x20may\x20optionally\x20be\x20followed\x20by\x20control\x20inpu\
    ts\x20that\x20have\x20the\x20format\n\x20\"^node\".\n\n\x0c\n\x05\x04\0\
    \x02\x02\x04\x12\x03\x19\x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x19\
    \x0b\x11\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x19\x12\x17\n\x0c\n\x05\
    \x04\0\x02\x02\x03\x12\x03\x19\x1a\x1b\n\x9e\x06\n\x04\x04\0\x02\x03\x12\
    \x03/\x02\x14\x1a\x90\x06\x20A\x20(possibly\x20partial)\x20specification\
    \x20for\x20the\x20device\x20on\x20which\x20this\n\x20node\x20should\x20b\
    e\x20placed.\n\x20The\x20expected\x20syntax\x20for\x20this\x20string\x20\
    is\x20as\x20follows:\n\n\x20DEVICE_SPEC\x20::=\x20PARTIAL_SPEC\n\n\x20PA\
    RTIAL_SPEC\x20::=\x20(\"/\"\x20CONSTRAINT)\x20*\n\x20CONSTRAINT\x20::=\
    \x20(\"job:\"\x20JOB_NAME)\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20|\x20(\"replica:\"\x20[1-9][0-9]*)\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20|\x20(\"task:\"\x20[1-9][0-9]*)\n\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20(\x20(\"gpu\"\
    \x20|\x20\"cpu\")\x20\":\"\x20([1-9][0-9]*\x20|\x20\"*\")\x20)\n\n\x20Va\
    lid\x20values\x20for\x20this\x20string\x20include:\n\x20*\x20\"/job:work\
    er/replica:0/task:1/device:GPU:3\"\x20\x20(full\x20specification)\n\x20*\
    \x20\"/job:worker/device:GPU:3\"\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20(partial\x20specification)\n\x20*\
    \x20\"\"\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20(no\x20specification)\n\n\x20If\x20the\x20constraints\x20do\x20n\
    ot\x20resolve\x20to\x20a\x20single\x20device\x20(or\x20if\x20this\n\x20f\
    ield\x20is\x20empty\x20or\x20not\x20present),\x20the\x20runtime\x20will\
    \x20attempt\x20to\n\x20choose\x20a\x20device\x20automatically.\n\n\r\n\
    \x05\x04\0\x02\x03\x04\x12\x04/\x02\x19\x1c\n\x0c\n\x05\x04\0\x02\x03\
    \x05\x12\x03/\x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03/\t\x0f\n\x0c\
    \n\x05\x04\0\x02\x03\x03\x12\x03/\x12\x13\n\x94\x05\n\x04\x04\0\x02\x04\
    \x12\x03=\x02\"\x1a\x86\x05\x20Operation-specific\x20graph-construction-\
    time\x20configuration.\n\x20Note\x20that\x20this\x20should\x20include\
    \x20all\x20attrs\x20defined\x20in\x20the\n\x20corresponding\x20OpDef,\
    \x20including\x20those\x20with\x20a\x20value\x20matching\n\x20the\x20def\
    ault\x20--\x20this\x20allows\x20the\x20default\x20to\x20change\x20and\
    \x20makes\n\x20NodeDefs\x20easier\x20to\x20interpret\x20on\x20their\x20o\
    wn.\x20\x20However,\x20if\n\x20an\x20attr\x20with\x20a\x20default\x20is\
    \x20not\x20specified\x20in\x20this\x20list,\x20the\n\x20default\x20will\
    \x20be\x20used.\n\x20The\x20\"names\"\x20(keys)\x20must\x20match\x20the\
    \x20regexp\x20\"[a-z][a-z0-9_]+\"\x20(and\n\x20one\x20of\x20the\x20names\
    \x20from\x20the\x20corresponding\x20OpDef's\x20attr\x20field).\n\x20The\
    \x20values\x20must\x20have\x20a\x20type\x20matching\x20the\x20correspond\
    ing\x20OpDef\n\x20attr's\x20type\x20field.\n\x20TODO(josh11b):\x20Add\
    \x20some\x20examples\x20here\x20showing\x20best\x20practices.\n\n\r\n\
    \x05\x04\0\x02\x04\x04\x12\x04=\x02/\x14\n\x0c\n\x05\x04\0\x02\x04\x06\
    \x12\x03=\x02\x18\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03=\x19\x1d\n\x0c\n\
    \x05\x04\0\x02\x04\x03\x12\x03=\x20!b\x06proto3\
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
