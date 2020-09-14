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
pub struct DebugTensorWatch {
    // message fields
    pub node_name: ::std::string::String,
    pub output_slot: i32,
    pub debug_ops: ::protobuf::RepeatedField<::std::string::String>,
    pub debug_urls: ::protobuf::RepeatedField<::std::string::String>,
    pub tolerate_debug_op_creation_failures: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugTensorWatch {}

impl DebugTensorWatch {
    pub fn new() -> DebugTensorWatch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugTensorWatch {
        static mut instance: ::protobuf::lazy::Lazy<DebugTensorWatch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugTensorWatch,
        };
        unsafe {
            instance.get(DebugTensorWatch::new)
        }
    }

    // string node_name = 1;

    pub fn clear_node_name(&mut self) {
        self.node_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_node_name(&mut self, v: ::std::string::String) {
        self.node_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_node_name(&mut self) -> &mut ::std::string::String {
        &mut self.node_name
    }

    // Take field
    pub fn take_node_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.node_name, ::std::string::String::new())
    }

    pub fn get_node_name(&self) -> &str {
        &self.node_name
    }

    fn get_node_name_for_reflect(&self) -> &::std::string::String {
        &self.node_name
    }

    fn mut_node_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.node_name
    }

    // int32 output_slot = 2;

    pub fn clear_output_slot(&mut self) {
        self.output_slot = 0;
    }

    // Param is passed by value, moved
    pub fn set_output_slot(&mut self, v: i32) {
        self.output_slot = v;
    }

    pub fn get_output_slot(&self) -> i32 {
        self.output_slot
    }

    fn get_output_slot_for_reflect(&self) -> &i32 {
        &self.output_slot
    }

    fn mut_output_slot_for_reflect(&mut self) -> &mut i32 {
        &mut self.output_slot
    }

    // repeated string debug_ops = 3;

    pub fn clear_debug_ops(&mut self) {
        self.debug_ops.clear();
    }

    // Param is passed by value, moved
    pub fn set_debug_ops(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.debug_ops = v;
    }

    // Mutable pointer to the field.
    pub fn mut_debug_ops(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.debug_ops
    }

    // Take field
    pub fn take_debug_ops(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.debug_ops, ::protobuf::RepeatedField::new())
    }

    pub fn get_debug_ops(&self) -> &[::std::string::String] {
        &self.debug_ops
    }

    fn get_debug_ops_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.debug_ops
    }

    fn mut_debug_ops_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.debug_ops
    }

    // repeated string debug_urls = 4;

    pub fn clear_debug_urls(&mut self) {
        self.debug_urls.clear();
    }

    // Param is passed by value, moved
    pub fn set_debug_urls(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.debug_urls = v;
    }

    // Mutable pointer to the field.
    pub fn mut_debug_urls(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.debug_urls
    }

    // Take field
    pub fn take_debug_urls(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.debug_urls, ::protobuf::RepeatedField::new())
    }

    pub fn get_debug_urls(&self) -> &[::std::string::String] {
        &self.debug_urls
    }

    fn get_debug_urls_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.debug_urls
    }

    fn mut_debug_urls_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.debug_urls
    }

    // bool tolerate_debug_op_creation_failures = 5;

    pub fn clear_tolerate_debug_op_creation_failures(&mut self) {
        self.tolerate_debug_op_creation_failures = false;
    }

    // Param is passed by value, moved
    pub fn set_tolerate_debug_op_creation_failures(&mut self, v: bool) {
        self.tolerate_debug_op_creation_failures = v;
    }

    pub fn get_tolerate_debug_op_creation_failures(&self) -> bool {
        self.tolerate_debug_op_creation_failures
    }

    fn get_tolerate_debug_op_creation_failures_for_reflect(&self) -> &bool {
        &self.tolerate_debug_op_creation_failures
    }

    fn mut_tolerate_debug_op_creation_failures_for_reflect(&mut self) -> &mut bool {
        &mut self.tolerate_debug_op_creation_failures
    }
}

impl ::protobuf::Message for DebugTensorWatch {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.node_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.output_slot = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.debug_ops)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.debug_urls)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.tolerate_debug_op_creation_failures = tmp;
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
        if !self.node_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.node_name);
        }
        if self.output_slot != 0 {
            my_size += ::protobuf::rt::value_size(2, self.output_slot, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.debug_ops {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.debug_urls {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if self.tolerate_debug_op_creation_failures != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.node_name.is_empty() {
            os.write_string(1, &self.node_name)?;
        }
        if self.output_slot != 0 {
            os.write_int32(2, self.output_slot)?;
        }
        for v in &self.debug_ops {
            os.write_string(3, &v)?;
        };
        for v in &self.debug_urls {
            os.write_string(4, &v)?;
        };
        if self.tolerate_debug_op_creation_failures != false {
            os.write_bool(5, self.tolerate_debug_op_creation_failures)?;
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

impl ::protobuf::MessageStatic for DebugTensorWatch {
    fn new() -> DebugTensorWatch {
        DebugTensorWatch::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugTensorWatch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "node_name",
                    DebugTensorWatch::get_node_name_for_reflect,
                    DebugTensorWatch::mut_node_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "output_slot",
                    DebugTensorWatch::get_output_slot_for_reflect,
                    DebugTensorWatch::mut_output_slot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "debug_ops",
                    DebugTensorWatch::get_debug_ops_for_reflect,
                    DebugTensorWatch::mut_debug_ops_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "debug_urls",
                    DebugTensorWatch::get_debug_urls_for_reflect,
                    DebugTensorWatch::mut_debug_urls_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "tolerate_debug_op_creation_failures",
                    DebugTensorWatch::get_tolerate_debug_op_creation_failures_for_reflect,
                    DebugTensorWatch::mut_tolerate_debug_op_creation_failures_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugTensorWatch>(
                    "DebugTensorWatch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugTensorWatch {
    fn clear(&mut self) {
        self.clear_node_name();
        self.clear_output_slot();
        self.clear_debug_ops();
        self.clear_debug_urls();
        self.clear_tolerate_debug_op_creation_failures();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugTensorWatch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugTensorWatch {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DebugOptions {
    // message fields
    pub debug_tensor_watch_opts: ::protobuf::RepeatedField<DebugTensorWatch>,
    pub global_step: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugOptions {}

impl DebugOptions {
    pub fn new() -> DebugOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugOptions {
        static mut instance: ::protobuf::lazy::Lazy<DebugOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugOptions,
        };
        unsafe {
            instance.get(DebugOptions::new)
        }
    }

    // repeated .tensorflow.DebugTensorWatch debug_tensor_watch_opts = 4;

    pub fn clear_debug_tensor_watch_opts(&mut self) {
        self.debug_tensor_watch_opts.clear();
    }

    // Param is passed by value, moved
    pub fn set_debug_tensor_watch_opts(&mut self, v: ::protobuf::RepeatedField<DebugTensorWatch>) {
        self.debug_tensor_watch_opts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_debug_tensor_watch_opts(&mut self) -> &mut ::protobuf::RepeatedField<DebugTensorWatch> {
        &mut self.debug_tensor_watch_opts
    }

    // Take field
    pub fn take_debug_tensor_watch_opts(&mut self) -> ::protobuf::RepeatedField<DebugTensorWatch> {
        ::std::mem::replace(&mut self.debug_tensor_watch_opts, ::protobuf::RepeatedField::new())
    }

    pub fn get_debug_tensor_watch_opts(&self) -> &[DebugTensorWatch] {
        &self.debug_tensor_watch_opts
    }

    fn get_debug_tensor_watch_opts_for_reflect(&self) -> &::protobuf::RepeatedField<DebugTensorWatch> {
        &self.debug_tensor_watch_opts
    }

    fn mut_debug_tensor_watch_opts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DebugTensorWatch> {
        &mut self.debug_tensor_watch_opts
    }

    // int64 global_step = 10;

    pub fn clear_global_step(&mut self) {
        self.global_step = 0;
    }

    // Param is passed by value, moved
    pub fn set_global_step(&mut self, v: i64) {
        self.global_step = v;
    }

    pub fn get_global_step(&self) -> i64 {
        self.global_step
    }

    fn get_global_step_for_reflect(&self) -> &i64 {
        &self.global_step
    }

    fn mut_global_step_for_reflect(&mut self) -> &mut i64 {
        &mut self.global_step
    }
}

impl ::protobuf::Message for DebugOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.debug_tensor_watch_opts {
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
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.debug_tensor_watch_opts)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.global_step = tmp;
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
        for value in &self.debug_tensor_watch_opts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.global_step != 0 {
            my_size += ::protobuf::rt::value_size(10, self.global_step, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.debug_tensor_watch_opts {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.global_step != 0 {
            os.write_int64(10, self.global_step)?;
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

impl ::protobuf::MessageStatic for DebugOptions {
    fn new() -> DebugOptions {
        DebugOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DebugTensorWatch>>(
                    "debug_tensor_watch_opts",
                    DebugOptions::get_debug_tensor_watch_opts_for_reflect,
                    DebugOptions::mut_debug_tensor_watch_opts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "global_step",
                    DebugOptions::get_global_step_for_reflect,
                    DebugOptions::mut_global_step_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugOptions>(
                    "DebugOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugOptions {
    fn clear(&mut self) {
        self.clear_debug_tensor_watch_opts();
        self.clear_global_step();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$tensorflow/core/protobuf/debug.proto\x12\ntensorflow\"\xda\x01\n\x10D\
    ebugTensorWatch\x12\x1b\n\tnode_name\x18\x01\x20\x01(\tR\x08nodeName\x12\
    \x1f\n\x0boutput_slot\x18\x02\x20\x01(\x05R\noutputSlot\x12\x1b\n\tdebug\
    _ops\x18\x03\x20\x03(\tR\x08debugOps\x12\x1d\n\ndebug_urls\x18\x04\x20\
    \x03(\tR\tdebugUrls\x12L\n#tolerate_debug_op_creation_failures\x18\x05\
    \x20\x01(\x08R\x1ftolerateDebugOpCreationFailures\"\x84\x01\n\x0cDebugOp\
    tions\x12S\n\x17debug_tensor_watch_opts\x18\x04\x20\x03(\x0b2\x1c.tensor\
    flow.DebugTensorWatchR\x14debugTensorWatchOpts\x12\x1f\n\x0bglobal_step\
    \x18\n\x20\x01(\x03R\nglobalStepB,\n\x18org.tensorflow.frameworkB\x0bDeb\
    ugProtosP\x01\xf8\x01\x01J\xa8\x14\n\x06\x12\x04\0\0=\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\n\x08\n\x01\x08\
    \x12\x03\x03\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\n\x0c\n\
    \x05\x08\xe7\x07\0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\
    \x12\x03\x03\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x03\x07\
    \x17\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\x01\x08\
    \x12\x03\x04\0,\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\0,\n\x0c\n\x05\
    \x08\xe7\x07\x01\x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\
    \x12\x03\x04\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x04\
    \x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e+\n\x08\n\x01\
    \x08\x12\x03\x05\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\x0c\n\
    \x05\x08\xe7\x07\x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\
    \x02\0\x12\x03\x05\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\
    \x05\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\n\
    \x01\x08\x12\x03\x06\01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\x0c\
    \n\x05\x08\xe7\x07\x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\x03\
    \x02\0\x12\x03\x06\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\
    \x06\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n7\n\x02\
    \x04\0\x12\x04\t\02\x01\x1a+\x20EXPERIMENTAL.\x20Option\x20for\x20watchi\
    ng\x20a\x20node.\n\n\n\n\x03\x04\0\x01\x12\x03\t\x08\x18\n)\n\x04\x04\0\
    \x02\0\x12\x03\x0b\x02\x17\x1a\x1c\x20Name\x20of\x20the\x20node\x20to\
    \x20watch.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x0b\x02\t\x1a\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x0b\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x0b\t\x12\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0b\x15\x16\n\xf2\x01\n\
    \x04\x04\0\x02\x01\x12\x03\x12\x02\x18\x1a\xe4\x01\x20Output\x20slot\x20\
    to\x20watch.\n\x20The\x20semantics\x20of\x20output_slot\x20==\x20-1\x20i\
    s\x20that\x20the\x20node\x20is\x20only\x20watched\x20for\n\x20completion\
    ,\x20but\x20not\x20for\x20any\x20output\x20tensors.\x20See\x20NodeComple\
    tionCallback\n\x20in\x20debug_gateway.h.\n\x20TODO(cais):\x20Implement\
    \x20this\x20semantics.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x12\x02\x0b\
    \x17\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x12\x02\x07\n\x0c\n\x05\x04\0\
    \x02\x01\x01\x12\x03\x12\x08\x13\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\
    \x12\x16\x17\n\x82\x01\n\x04\x04\0\x02\x02\x12\x03\x17\x02\x20\x1au\x20N\
    ame(s)\x20of\x20the\x20debugging\x20op(s).\n\x20One\x20or\x20more\x20tha\
    n\x20one\x20probes\x20on\x20a\x20tensor.\n\x20e.g.,\x20{\"DebugIdentity\
    \",\x20\"DebugNanCount\"}\n\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\x17\
    \x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x17\x0b\x11\n\x0c\n\x05\x04\
    \0\x02\x02\x01\x12\x03\x17\x12\x1b\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\
    \x17\x1e\x1f\n\xcb\x07\n\x04\x04\0\x02\x03\x12\x03-\x02!\x1a\xbd\x07\x20\
    URL(s)\x20for\x20debug\x20targets(s).\n\n\x20Supported\x20URL\x20formats\
    \x20are:\n\x20\x20\x20-\x20file:///foo/tfdbg_dump:\x20Writes\x20out\x20E\
    vent\x20content\x20to\x20file\n\x20\x20\x20\x20\x20/foo/tfdbg_dump.\x20\
    \x20Assumes\x20all\x20directories\x20can\x20be\x20created\x20if\x20they\
    \x20don't\n\x20\x20\x20\x20\x20already\x20exist.\n\x20\x20\x20-\x20grpc:\
    //localhost:11011:\x20Sends\x20an\x20RPC\x20request\x20to\x20an\x20Event\
    Listener\n\x20\x20\x20\x20\x20service\x20running\x20at\x20localhost:1101\
    1\x20with\x20the\x20event.\n\x20\x20\x20-\x20memcbk:///event_key:\x20Rou\
    tes\x20tensors\x20to\x20clients\x20using\x20the\n\x20\x20\x20\x20\x20cal\
    lback\x20registered\x20with\x20the\x20DebugCallbackRegistry\x20for\x20ev\
    ent_key.\n\n\x20Each\x20debug\x20op\x20listed\x20in\x20debug_ops\x20will\
    \x20publish\x20its\x20output\x20tensor\x20(debug\n\x20signal)\x20to\x20a\
    ll\x20URLs\x20in\x20debug_urls.\n\n\x20N.B.\x20Session::Run()\x20support\
    s\x20concurrent\x20invocations\x20of\x20the\x20same\x20inputs\n\x20(feed\
    \x20keys),\x20outputs\x20and\x20target\x20nodes.\x20If\x20such\x20concur\
    rent\x20invocations\n\x20are\x20to\x20be\x20debugged,\x20the\x20callers\
    \x20of\x20Session::Run()\x20must\x20use\x20distinct\n\x20debug_urls\x20t\
    o\x20make\x20sure\x20that\x20the\x20streamed\x20or\x20dumped\x20events\
    \x20do\x20not\x20overlap\n\x20among\x20the\x20invocations.\n\x20TODO(cai\
    s):\x20More\x20visible\x20documentation\x20of\x20this\x20in\x20g3docs.\n\
    \n\x0c\n\x05\x04\0\x02\x03\x04\x12\x03-\x02\n\n\x0c\n\x05\x04\0\x02\x03\
    \x05\x12\x03-\x0b\x11\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03-\x12\x1c\n\
    \x0c\n\x05\x04\0\x02\x03\x03\x12\x03-\x1f\x20\n\x80\x01\n\x04\x04\0\x02\
    \x04\x12\x031\x02/\x1as\x20Do\x20not\x20error\x20out\x20if\x20debug\x20o\
    p\x20creation\x20fails\x20(e.g.,\x20due\x20to\x20dtype\n\x20incompatibil\
    ity).\x20Instead,\x20just\x20log\x20the\x20failure.\n\n\r\n\x05\x04\0\
    \x02\x04\x04\x12\x041\x02-!\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x031\x02\
    \x06\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x031\x07*\n\x0c\n\x05\x04\0\x02\
    \x04\x03\x12\x031-.\nC\n\x02\x04\x01\x12\x045\0=\x01\x1a7\x20EXPERIMENTA\
    L.\x20Options\x20for\x20initializing\x20DebuggerState.\n\n\n\n\x03\x04\
    \x01\x01\x12\x035\x08\x14\n\x20\n\x04\x04\x01\x02\0\x12\x037\x028\x1a\
    \x13\x20Debugging\x20options\n\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x037\
    \x02\n\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x037\x0b\x1b\n\x0c\n\x05\x04\
    \x01\x02\0\x01\x12\x037\x1c3\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03767\n\
    \x87\x01\n\x04\x04\x01\x02\x01\x12\x03<\x02\x19\x1az\x20Caller-specified\
    \x20global\x20step\x20count.\n\x20Note\x20that\x20this\x20is\x20distinct\
    \x20from\x20the\x20session\x20run\x20count\x20and\x20the\x20executor\n\
    \x20step\x20count.\n\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04<\x0278\n\x0c\
    \n\x05\x04\x01\x02\x01\x05\x12\x03<\x02\x07\n\x0c\n\x05\x04\x01\x02\x01\
    \x01\x12\x03<\x08\x13\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03<\x16\x18b\
    \x06proto3\
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
