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
pub struct CostGraphDef {
    // message fields
    pub node: ::protobuf::RepeatedField<CostGraphDef_Node>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CostGraphDef {}

impl CostGraphDef {
    pub fn new() -> CostGraphDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CostGraphDef {
        static mut instance: ::protobuf::lazy::Lazy<CostGraphDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CostGraphDef,
        };
        unsafe {
            instance.get(CostGraphDef::new)
        }
    }

    // repeated .tensorflow.CostGraphDef.Node node = 1;

    pub fn clear_node(&mut self) {
        self.node.clear();
    }

    // Param is passed by value, moved
    pub fn set_node(&mut self, v: ::protobuf::RepeatedField<CostGraphDef_Node>) {
        self.node = v;
    }

    // Mutable pointer to the field.
    pub fn mut_node(&mut self) -> &mut ::protobuf::RepeatedField<CostGraphDef_Node> {
        &mut self.node
    }

    // Take field
    pub fn take_node(&mut self) -> ::protobuf::RepeatedField<CostGraphDef_Node> {
        ::std::mem::replace(&mut self.node, ::protobuf::RepeatedField::new())
    }

    pub fn get_node(&self) -> &[CostGraphDef_Node] {
        &self.node
    }

    fn get_node_for_reflect(&self) -> &::protobuf::RepeatedField<CostGraphDef_Node> {
        &self.node
    }

    fn mut_node_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CostGraphDef_Node> {
        &mut self.node
    }
}

impl ::protobuf::Message for CostGraphDef {
    fn is_initialized(&self) -> bool {
        for v in &self.node {
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

impl ::protobuf::MessageStatic for CostGraphDef {
    fn new() -> CostGraphDef {
        CostGraphDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<CostGraphDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CostGraphDef_Node>>(
                    "node",
                    CostGraphDef::get_node_for_reflect,
                    CostGraphDef::mut_node_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CostGraphDef>(
                    "CostGraphDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CostGraphDef {
    fn clear(&mut self) {
        self.clear_node();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CostGraphDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CostGraphDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CostGraphDef_Node {
    // message fields
    pub name: ::std::string::String,
    pub device: ::std::string::String,
    pub id: i32,
    pub input_info: ::protobuf::RepeatedField<CostGraphDef_Node_InputInfo>,
    pub output_info: ::protobuf::RepeatedField<CostGraphDef_Node_OutputInfo>,
    pub temporary_memory_size: i64,
    pub host_temp_memory_size: i64,
    pub device_temp_memory_size: i64,
    pub host_persistent_memory_size: i64,
    pub device_persistent_memory_size: i64,
    pub compute_cost: i64,
    pub compute_time: i64,
    pub memory_time: i64,
    pub is_final: bool,
    pub control_input: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CostGraphDef_Node {}

impl CostGraphDef_Node {
    pub fn new() -> CostGraphDef_Node {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CostGraphDef_Node {
        static mut instance: ::protobuf::lazy::Lazy<CostGraphDef_Node> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CostGraphDef_Node,
        };
        unsafe {
            instance.get(CostGraphDef_Node::new)
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

    // string device = 2;

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

    // int32 id = 3;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = v;
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &i32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut i32 {
        &mut self.id
    }

    // repeated .tensorflow.CostGraphDef.Node.InputInfo input_info = 4;

    pub fn clear_input_info(&mut self) {
        self.input_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_input_info(&mut self, v: ::protobuf::RepeatedField<CostGraphDef_Node_InputInfo>) {
        self.input_info = v;
    }

    // Mutable pointer to the field.
    pub fn mut_input_info(&mut self) -> &mut ::protobuf::RepeatedField<CostGraphDef_Node_InputInfo> {
        &mut self.input_info
    }

    // Take field
    pub fn take_input_info(&mut self) -> ::protobuf::RepeatedField<CostGraphDef_Node_InputInfo> {
        ::std::mem::replace(&mut self.input_info, ::protobuf::RepeatedField::new())
    }

    pub fn get_input_info(&self) -> &[CostGraphDef_Node_InputInfo] {
        &self.input_info
    }

    fn get_input_info_for_reflect(&self) -> &::protobuf::RepeatedField<CostGraphDef_Node_InputInfo> {
        &self.input_info
    }

    fn mut_input_info_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CostGraphDef_Node_InputInfo> {
        &mut self.input_info
    }

    // repeated .tensorflow.CostGraphDef.Node.OutputInfo output_info = 5;

    pub fn clear_output_info(&mut self) {
        self.output_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_output_info(&mut self, v: ::protobuf::RepeatedField<CostGraphDef_Node_OutputInfo>) {
        self.output_info = v;
    }

    // Mutable pointer to the field.
    pub fn mut_output_info(&mut self) -> &mut ::protobuf::RepeatedField<CostGraphDef_Node_OutputInfo> {
        &mut self.output_info
    }

    // Take field
    pub fn take_output_info(&mut self) -> ::protobuf::RepeatedField<CostGraphDef_Node_OutputInfo> {
        ::std::mem::replace(&mut self.output_info, ::protobuf::RepeatedField::new())
    }

    pub fn get_output_info(&self) -> &[CostGraphDef_Node_OutputInfo] {
        &self.output_info
    }

    fn get_output_info_for_reflect(&self) -> &::protobuf::RepeatedField<CostGraphDef_Node_OutputInfo> {
        &self.output_info
    }

    fn mut_output_info_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CostGraphDef_Node_OutputInfo> {
        &mut self.output_info
    }

    // int64 temporary_memory_size = 6;

    pub fn clear_temporary_memory_size(&mut self) {
        self.temporary_memory_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_temporary_memory_size(&mut self, v: i64) {
        self.temporary_memory_size = v;
    }

    pub fn get_temporary_memory_size(&self) -> i64 {
        self.temporary_memory_size
    }

    fn get_temporary_memory_size_for_reflect(&self) -> &i64 {
        &self.temporary_memory_size
    }

    fn mut_temporary_memory_size_for_reflect(&mut self) -> &mut i64 {
        &mut self.temporary_memory_size
    }

    // int64 host_temp_memory_size = 10;

    pub fn clear_host_temp_memory_size(&mut self) {
        self.host_temp_memory_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_host_temp_memory_size(&mut self, v: i64) {
        self.host_temp_memory_size = v;
    }

    pub fn get_host_temp_memory_size(&self) -> i64 {
        self.host_temp_memory_size
    }

    fn get_host_temp_memory_size_for_reflect(&self) -> &i64 {
        &self.host_temp_memory_size
    }

    fn mut_host_temp_memory_size_for_reflect(&mut self) -> &mut i64 {
        &mut self.host_temp_memory_size
    }

    // int64 device_temp_memory_size = 11;

    pub fn clear_device_temp_memory_size(&mut self) {
        self.device_temp_memory_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_device_temp_memory_size(&mut self, v: i64) {
        self.device_temp_memory_size = v;
    }

    pub fn get_device_temp_memory_size(&self) -> i64 {
        self.device_temp_memory_size
    }

    fn get_device_temp_memory_size_for_reflect(&self) -> &i64 {
        &self.device_temp_memory_size
    }

    fn mut_device_temp_memory_size_for_reflect(&mut self) -> &mut i64 {
        &mut self.device_temp_memory_size
    }

    // int64 host_persistent_memory_size = 12;

    pub fn clear_host_persistent_memory_size(&mut self) {
        self.host_persistent_memory_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_host_persistent_memory_size(&mut self, v: i64) {
        self.host_persistent_memory_size = v;
    }

    pub fn get_host_persistent_memory_size(&self) -> i64 {
        self.host_persistent_memory_size
    }

    fn get_host_persistent_memory_size_for_reflect(&self) -> &i64 {
        &self.host_persistent_memory_size
    }

    fn mut_host_persistent_memory_size_for_reflect(&mut self) -> &mut i64 {
        &mut self.host_persistent_memory_size
    }

    // int64 device_persistent_memory_size = 16;

    pub fn clear_device_persistent_memory_size(&mut self) {
        self.device_persistent_memory_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_device_persistent_memory_size(&mut self, v: i64) {
        self.device_persistent_memory_size = v;
    }

    pub fn get_device_persistent_memory_size(&self) -> i64 {
        self.device_persistent_memory_size
    }

    fn get_device_persistent_memory_size_for_reflect(&self) -> &i64 {
        &self.device_persistent_memory_size
    }

    fn mut_device_persistent_memory_size_for_reflect(&mut self) -> &mut i64 {
        &mut self.device_persistent_memory_size
    }

    // int64 compute_cost = 9;

    pub fn clear_compute_cost(&mut self) {
        self.compute_cost = 0;
    }

    // Param is passed by value, moved
    pub fn set_compute_cost(&mut self, v: i64) {
        self.compute_cost = v;
    }

    pub fn get_compute_cost(&self) -> i64 {
        self.compute_cost
    }

    fn get_compute_cost_for_reflect(&self) -> &i64 {
        &self.compute_cost
    }

    fn mut_compute_cost_for_reflect(&mut self) -> &mut i64 {
        &mut self.compute_cost
    }

    // int64 compute_time = 14;

    pub fn clear_compute_time(&mut self) {
        self.compute_time = 0;
    }

    // Param is passed by value, moved
    pub fn set_compute_time(&mut self, v: i64) {
        self.compute_time = v;
    }

    pub fn get_compute_time(&self) -> i64 {
        self.compute_time
    }

    fn get_compute_time_for_reflect(&self) -> &i64 {
        &self.compute_time
    }

    fn mut_compute_time_for_reflect(&mut self) -> &mut i64 {
        &mut self.compute_time
    }

    // int64 memory_time = 15;

    pub fn clear_memory_time(&mut self) {
        self.memory_time = 0;
    }

    // Param is passed by value, moved
    pub fn set_memory_time(&mut self, v: i64) {
        self.memory_time = v;
    }

    pub fn get_memory_time(&self) -> i64 {
        self.memory_time
    }

    fn get_memory_time_for_reflect(&self) -> &i64 {
        &self.memory_time
    }

    fn mut_memory_time_for_reflect(&mut self) -> &mut i64 {
        &mut self.memory_time
    }

    // bool is_final = 7;

    pub fn clear_is_final(&mut self) {
        self.is_final = false;
    }

    // Param is passed by value, moved
    pub fn set_is_final(&mut self, v: bool) {
        self.is_final = v;
    }

    pub fn get_is_final(&self) -> bool {
        self.is_final
    }

    fn get_is_final_for_reflect(&self) -> &bool {
        &self.is_final
    }

    fn mut_is_final_for_reflect(&mut self) -> &mut bool {
        &mut self.is_final
    }

    // repeated int32 control_input = 8;

    pub fn clear_control_input(&mut self) {
        self.control_input.clear();
    }

    // Param is passed by value, moved
    pub fn set_control_input(&mut self, v: ::std::vec::Vec<i32>) {
        self.control_input = v;
    }

    // Mutable pointer to the field.
    pub fn mut_control_input(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.control_input
    }

    // Take field
    pub fn take_control_input(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.control_input, ::std::vec::Vec::new())
    }

    pub fn get_control_input(&self) -> &[i32] {
        &self.control_input
    }

    fn get_control_input_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.control_input
    }

    fn mut_control_input_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.control_input
    }
}

impl ::protobuf::Message for CostGraphDef_Node {
    fn is_initialized(&self) -> bool {
        for v in &self.input_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.output_info {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.device)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.id = tmp;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.input_info)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.output_info)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.temporary_memory_size = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.host_temp_memory_size = tmp;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.device_temp_memory_size = tmp;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.host_persistent_memory_size = tmp;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.device_persistent_memory_size = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.compute_cost = tmp;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.compute_time = tmp;
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.memory_time = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_final = tmp;
                },
                8 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.control_input)?;
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
        if !self.device.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.device);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(3, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.input_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.output_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.temporary_memory_size != 0 {
            my_size += ::protobuf::rt::value_size(6, self.temporary_memory_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.host_temp_memory_size != 0 {
            my_size += ::protobuf::rt::value_size(10, self.host_temp_memory_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.device_temp_memory_size != 0 {
            my_size += ::protobuf::rt::value_size(11, self.device_temp_memory_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.host_persistent_memory_size != 0 {
            my_size += ::protobuf::rt::value_size(12, self.host_persistent_memory_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.device_persistent_memory_size != 0 {
            my_size += ::protobuf::rt::value_size(16, self.device_persistent_memory_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.compute_cost != 0 {
            my_size += ::protobuf::rt::value_size(9, self.compute_cost, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.compute_time != 0 {
            my_size += ::protobuf::rt::value_size(14, self.compute_time, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.memory_time != 0 {
            my_size += ::protobuf::rt::value_size(15, self.memory_time, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.is_final != false {
            my_size += 2;
        }
        for value in &self.control_input {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.device.is_empty() {
            os.write_string(2, &self.device)?;
        }
        if self.id != 0 {
            os.write_int32(3, self.id)?;
        }
        for v in &self.input_info {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.output_info {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.temporary_memory_size != 0 {
            os.write_int64(6, self.temporary_memory_size)?;
        }
        if self.host_temp_memory_size != 0 {
            os.write_int64(10, self.host_temp_memory_size)?;
        }
        if self.device_temp_memory_size != 0 {
            os.write_int64(11, self.device_temp_memory_size)?;
        }
        if self.host_persistent_memory_size != 0 {
            os.write_int64(12, self.host_persistent_memory_size)?;
        }
        if self.device_persistent_memory_size != 0 {
            os.write_int64(16, self.device_persistent_memory_size)?;
        }
        if self.compute_cost != 0 {
            os.write_int64(9, self.compute_cost)?;
        }
        if self.compute_time != 0 {
            os.write_int64(14, self.compute_time)?;
        }
        if self.memory_time != 0 {
            os.write_int64(15, self.memory_time)?;
        }
        if self.is_final != false {
            os.write_bool(7, self.is_final)?;
        }
        for v in &self.control_input {
            os.write_int32(8, *v)?;
        };
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

impl ::protobuf::MessageStatic for CostGraphDef_Node {
    fn new() -> CostGraphDef_Node {
        CostGraphDef_Node::new()
    }

    fn descriptor_static(_: ::std::option::Option<CostGraphDef_Node>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CostGraphDef_Node::get_name_for_reflect,
                    CostGraphDef_Node::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "device",
                    CostGraphDef_Node::get_device_for_reflect,
                    CostGraphDef_Node::mut_device_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "id",
                    CostGraphDef_Node::get_id_for_reflect,
                    CostGraphDef_Node::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CostGraphDef_Node_InputInfo>>(
                    "input_info",
                    CostGraphDef_Node::get_input_info_for_reflect,
                    CostGraphDef_Node::mut_input_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CostGraphDef_Node_OutputInfo>>(
                    "output_info",
                    CostGraphDef_Node::get_output_info_for_reflect,
                    CostGraphDef_Node::mut_output_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "temporary_memory_size",
                    CostGraphDef_Node::get_temporary_memory_size_for_reflect,
                    CostGraphDef_Node::mut_temporary_memory_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "host_temp_memory_size",
                    CostGraphDef_Node::get_host_temp_memory_size_for_reflect,
                    CostGraphDef_Node::mut_host_temp_memory_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "device_temp_memory_size",
                    CostGraphDef_Node::get_device_temp_memory_size_for_reflect,
                    CostGraphDef_Node::mut_device_temp_memory_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "host_persistent_memory_size",
                    CostGraphDef_Node::get_host_persistent_memory_size_for_reflect,
                    CostGraphDef_Node::mut_host_persistent_memory_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "device_persistent_memory_size",
                    CostGraphDef_Node::get_device_persistent_memory_size_for_reflect,
                    CostGraphDef_Node::mut_device_persistent_memory_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "compute_cost",
                    CostGraphDef_Node::get_compute_cost_for_reflect,
                    CostGraphDef_Node::mut_compute_cost_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "compute_time",
                    CostGraphDef_Node::get_compute_time_for_reflect,
                    CostGraphDef_Node::mut_compute_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "memory_time",
                    CostGraphDef_Node::get_memory_time_for_reflect,
                    CostGraphDef_Node::mut_memory_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_final",
                    CostGraphDef_Node::get_is_final_for_reflect,
                    CostGraphDef_Node::mut_is_final_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "control_input",
                    CostGraphDef_Node::get_control_input_for_reflect,
                    CostGraphDef_Node::mut_control_input_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CostGraphDef_Node>(
                    "CostGraphDef_Node",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CostGraphDef_Node {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_device();
        self.clear_id();
        self.clear_input_info();
        self.clear_output_info();
        self.clear_temporary_memory_size();
        self.clear_host_temp_memory_size();
        self.clear_device_temp_memory_size();
        self.clear_host_persistent_memory_size();
        self.clear_device_persistent_memory_size();
        self.clear_compute_cost();
        self.clear_compute_time();
        self.clear_memory_time();
        self.clear_is_final();
        self.clear_control_input();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CostGraphDef_Node {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CostGraphDef_Node {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CostGraphDef_Node_InputInfo {
    // message fields
    pub preceding_node: i32,
    pub preceding_port: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CostGraphDef_Node_InputInfo {}

impl CostGraphDef_Node_InputInfo {
    pub fn new() -> CostGraphDef_Node_InputInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CostGraphDef_Node_InputInfo {
        static mut instance: ::protobuf::lazy::Lazy<CostGraphDef_Node_InputInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CostGraphDef_Node_InputInfo,
        };
        unsafe {
            instance.get(CostGraphDef_Node_InputInfo::new)
        }
    }

    // int32 preceding_node = 1;

    pub fn clear_preceding_node(&mut self) {
        self.preceding_node = 0;
    }

    // Param is passed by value, moved
    pub fn set_preceding_node(&mut self, v: i32) {
        self.preceding_node = v;
    }

    pub fn get_preceding_node(&self) -> i32 {
        self.preceding_node
    }

    fn get_preceding_node_for_reflect(&self) -> &i32 {
        &self.preceding_node
    }

    fn mut_preceding_node_for_reflect(&mut self) -> &mut i32 {
        &mut self.preceding_node
    }

    // int32 preceding_port = 2;

    pub fn clear_preceding_port(&mut self) {
        self.preceding_port = 0;
    }

    // Param is passed by value, moved
    pub fn set_preceding_port(&mut self, v: i32) {
        self.preceding_port = v;
    }

    pub fn get_preceding_port(&self) -> i32 {
        self.preceding_port
    }

    fn get_preceding_port_for_reflect(&self) -> &i32 {
        &self.preceding_port
    }

    fn mut_preceding_port_for_reflect(&mut self) -> &mut i32 {
        &mut self.preceding_port
    }
}

impl ::protobuf::Message for CostGraphDef_Node_InputInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.preceding_node = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.preceding_port = tmp;
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
        if self.preceding_node != 0 {
            my_size += ::protobuf::rt::value_size(1, self.preceding_node, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.preceding_port != 0 {
            my_size += ::protobuf::rt::value_size(2, self.preceding_port, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.preceding_node != 0 {
            os.write_int32(1, self.preceding_node)?;
        }
        if self.preceding_port != 0 {
            os.write_int32(2, self.preceding_port)?;
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

impl ::protobuf::MessageStatic for CostGraphDef_Node_InputInfo {
    fn new() -> CostGraphDef_Node_InputInfo {
        CostGraphDef_Node_InputInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CostGraphDef_Node_InputInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "preceding_node",
                    CostGraphDef_Node_InputInfo::get_preceding_node_for_reflect,
                    CostGraphDef_Node_InputInfo::mut_preceding_node_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "preceding_port",
                    CostGraphDef_Node_InputInfo::get_preceding_port_for_reflect,
                    CostGraphDef_Node_InputInfo::mut_preceding_port_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CostGraphDef_Node_InputInfo>(
                    "CostGraphDef_Node_InputInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CostGraphDef_Node_InputInfo {
    fn clear(&mut self) {
        self.clear_preceding_node();
        self.clear_preceding_port();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CostGraphDef_Node_InputInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CostGraphDef_Node_InputInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CostGraphDef_Node_OutputInfo {
    // message fields
    pub size: i64,
    pub alias_input_port: i64,
    pub shape: ::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto>,
    pub dtype: super::types::DataType,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CostGraphDef_Node_OutputInfo {}

impl CostGraphDef_Node_OutputInfo {
    pub fn new() -> CostGraphDef_Node_OutputInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CostGraphDef_Node_OutputInfo {
        static mut instance: ::protobuf::lazy::Lazy<CostGraphDef_Node_OutputInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CostGraphDef_Node_OutputInfo,
        };
        unsafe {
            instance.get(CostGraphDef_Node_OutputInfo::new)
        }
    }

    // int64 size = 1;

    pub fn clear_size(&mut self) {
        self.size = 0;
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: i64) {
        self.size = v;
    }

    pub fn get_size(&self) -> i64 {
        self.size
    }

    fn get_size_for_reflect(&self) -> &i64 {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut i64 {
        &mut self.size
    }

    // int64 alias_input_port = 2;

    pub fn clear_alias_input_port(&mut self) {
        self.alias_input_port = 0;
    }

    // Param is passed by value, moved
    pub fn set_alias_input_port(&mut self, v: i64) {
        self.alias_input_port = v;
    }

    pub fn get_alias_input_port(&self) -> i64 {
        self.alias_input_port
    }

    fn get_alias_input_port_for_reflect(&self) -> &i64 {
        &self.alias_input_port
    }

    fn mut_alias_input_port_for_reflect(&mut self) -> &mut i64 {
        &mut self.alias_input_port
    }

    // .tensorflow.TensorShapeProto shape = 3;

    pub fn clear_shape(&mut self) {
        self.shape.clear();
    }

    pub fn has_shape(&self) -> bool {
        self.shape.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shape(&mut self, v: super::tensor_shape::TensorShapeProto) {
        self.shape = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shape(&mut self) -> &mut super::tensor_shape::TensorShapeProto {
        if self.shape.is_none() {
            self.shape.set_default();
        }
        self.shape.as_mut().unwrap()
    }

    // Take field
    pub fn take_shape(&mut self) -> super::tensor_shape::TensorShapeProto {
        self.shape.take().unwrap_or_else(|| super::tensor_shape::TensorShapeProto::new())
    }

    pub fn get_shape(&self) -> &super::tensor_shape::TensorShapeProto {
        self.shape.as_ref().unwrap_or_else(|| super::tensor_shape::TensorShapeProto::default_instance())
    }

    fn get_shape_for_reflect(&self) -> &::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto> {
        &self.shape
    }

    fn mut_shape_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto> {
        &mut self.shape
    }

    // .tensorflow.DataType dtype = 4;

    pub fn clear_dtype(&mut self) {
        self.dtype = super::types::DataType::DT_INVALID;
    }

    // Param is passed by value, moved
    pub fn set_dtype(&mut self, v: super::types::DataType) {
        self.dtype = v;
    }

    pub fn get_dtype(&self) -> super::types::DataType {
        self.dtype
    }

    fn get_dtype_for_reflect(&self) -> &super::types::DataType {
        &self.dtype
    }

    fn mut_dtype_for_reflect(&mut self) -> &mut super::types::DataType {
        &mut self.dtype
    }
}

impl ::protobuf::Message for CostGraphDef_Node_OutputInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.shape {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.size = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.alias_input_port = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.shape)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.dtype = tmp;
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
        if self.size != 0 {
            my_size += ::protobuf::rt::value_size(1, self.size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.alias_input_port != 0 {
            my_size += ::protobuf::rt::value_size(2, self.alias_input_port, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.shape.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.dtype != super::types::DataType::DT_INVALID {
            my_size += ::protobuf::rt::enum_size(4, self.dtype);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.size != 0 {
            os.write_int64(1, self.size)?;
        }
        if self.alias_input_port != 0 {
            os.write_int64(2, self.alias_input_port)?;
        }
        if let Some(ref v) = self.shape.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.dtype != super::types::DataType::DT_INVALID {
            os.write_enum(4, self.dtype.value())?;
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

impl ::protobuf::MessageStatic for CostGraphDef_Node_OutputInfo {
    fn new() -> CostGraphDef_Node_OutputInfo {
        CostGraphDef_Node_OutputInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CostGraphDef_Node_OutputInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "size",
                    CostGraphDef_Node_OutputInfo::get_size_for_reflect,
                    CostGraphDef_Node_OutputInfo::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "alias_input_port",
                    CostGraphDef_Node_OutputInfo::get_alias_input_port_for_reflect,
                    CostGraphDef_Node_OutputInfo::mut_alias_input_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tensor_shape::TensorShapeProto>>(
                    "shape",
                    CostGraphDef_Node_OutputInfo::get_shape_for_reflect,
                    CostGraphDef_Node_OutputInfo::mut_shape_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::types::DataType>>(
                    "dtype",
                    CostGraphDef_Node_OutputInfo::get_dtype_for_reflect,
                    CostGraphDef_Node_OutputInfo::mut_dtype_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CostGraphDef_Node_OutputInfo>(
                    "CostGraphDef_Node_OutputInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CostGraphDef_Node_OutputInfo {
    fn clear(&mut self) {
        self.clear_size();
        self.clear_alias_input_port();
        self.clear_shape();
        self.clear_dtype();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CostGraphDef_Node_OutputInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CostGraphDef_Node_OutputInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n*tensorflow/core/framework/cost_graph.proto\x12\ntensorflow\x1a,tensor\
    flow/core/framework/tensor_shape.proto\x1a%tensorflow/core/framework/typ\
    es.proto\"\xe8\x07\n\x0cCostGraphDef\x121\n\x04node\x18\x01\x20\x03(\x0b\
    2\x1d.tensorflow.CostGraphDef.NodeR\x04node\x1a\xa4\x07\n\x04Node\x12\
    \x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x16\n\x06device\x18\x02\
    \x20\x01(\tR\x06device\x12\x0e\n\x02id\x18\x03\x20\x01(\x05R\x02id\x12F\
    \n\ninput_info\x18\x04\x20\x03(\x0b2'.tensorflow.CostGraphDef.Node.Input\
    InfoR\tinputInfo\x12I\n\x0boutput_info\x18\x05\x20\x03(\x0b2(.tensorflow\
    .CostGraphDef.Node.OutputInfoR\noutputInfo\x122\n\x15temporary_memory_si\
    ze\x18\x06\x20\x01(\x03R\x13temporaryMemorySize\x121\n\x15host_temp_memo\
    ry_size\x18\n\x20\x01(\x03R\x12hostTempMemorySize\x125\n\x17device_temp_\
    memory_size\x18\x0b\x20\x01(\x03R\x14deviceTempMemorySize\x12=\n\x1bhost\
    _persistent_memory_size\x18\x0c\x20\x01(\x03R\x18hostPersistentMemorySiz\
    e\x12A\n\x1ddevice_persistent_memory_size\x18\x10\x20\x01(\x03R\x1adevic\
    ePersistentMemorySize\x12!\n\x0ccompute_cost\x18\t\x20\x01(\x03R\x0bcomp\
    uteCost\x12!\n\x0ccompute_time\x18\x0e\x20\x01(\x03R\x0bcomputeTime\x12\
    \x1f\n\x0bmemory_time\x18\x0f\x20\x01(\x03R\nmemoryTime\x12\x19\n\x08is_\
    final\x18\x07\x20\x01(\x08R\x07isFinal\x12#\n\rcontrol_input\x18\x08\x20\
    \x03(\x05R\x0ccontrolInput\x1aY\n\tInputInfo\x12%\n\x0epreceding_node\
    \x18\x01\x20\x01(\x05R\rprecedingNode\x12%\n\x0epreceding_port\x18\x02\
    \x20\x01(\x05R\rprecedingPort\x1a\xaa\x01\n\nOutputInfo\x12\x12\n\x04siz\
    e\x18\x01\x20\x01(\x03R\x04size\x12(\n\x10alias_input_port\x18\x02\x20\
    \x01(\x03R\x0ealiasInputPort\x122\n\x05shape\x18\x03\x20\x01(\x0b2\x1c.t\
    ensorflow.TensorShapeProtoR\x05shape\x12*\n\x05dtype\x18\x04\x20\x01(\
    \x0e2\x14.tensorflow.DataTypeR\x05dtypeB0\n\x18org.tensorflow.frameworkB\
    \x0fCostGraphProtosP\x01\xf8\x01\x01J\xab\x1a\n\x06\x12\x04\0\0G\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\n\x08\
    \n\x01\x08\x12\x03\x03\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\
    \n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\
    \0\x02\0\x12\x03\x03\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\
    \x03\x07\x17\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\
    \x01\x08\x12\x03\x04\00\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\00\n\x0c\
    \n\x05\x08\xe7\x07\x01\x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\
    \x02\0\x12\x03\x04\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\
    \x04\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e/\n\x08\n\
    \x01\x08\x12\x03\x05\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\
    \x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\
    \x02\x02\0\x12\x03\x05\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\
    \x03\x05\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\
    \n\x01\x08\x12\x03\x06\01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\
    \x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\
    \x03\x02\0\x12\x03\x06\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\
    \x03\x06\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\t\n\
    \x02\x03\0\x12\x03\x08\x075\n\t\n\x02\x03\x01\x12\x03\t\x07.\n\n\n\x02\
    \x04\0\x12\x04\x0b\0G\x01\n\n\n\x03\x04\0\x01\x12\x03\x0b\x08\x14\n\x0c\
    \n\x04\x04\0\x03\0\x12\x04\x0c\x02E\x03\n\x0c\n\x05\x04\0\x03\0\x01\x12\
    \x03\x0c\n\x0e\nA\n\x06\x04\0\x03\0\x02\0\x12\x03\x0e\x04\x14\x1a2\x20Th\
    e\x20name\x20of\x20the\x20node.\x20Names\x20are\x20globally\x20unique.\n\
    \n\x0f\n\x07\x04\0\x03\0\x02\0\x04\x12\x04\x0e\x04\x0c\x10\n\x0e\n\x07\
    \x04\0\x03\0\x02\0\x05\x12\x03\x0e\x04\n\n\x0e\n\x07\x04\0\x03\0\x02\0\
    \x01\x12\x03\x0e\x0b\x0f\n\x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\x03\x0e\
    \x12\x13\n\x8a\x01\n\x06\x04\0\x03\0\x02\x01\x12\x03\x12\x04\x16\x1a{\
    \x20The\x20device\x20of\x20the\x20node.\x20Can\x20be\x20empty\x20if\x20t\
    he\x20node\x20is\x20mapped\x20to\x20the\n\x20default\x20partition\x20or\
    \x20partitioning\x20hasn't\x20been\x20run\x20yet.\n\n\x0f\n\x07\x04\0\
    \x03\0\x02\x01\x04\x12\x04\x12\x04\x0e\x14\n\x0e\n\x07\x04\0\x03\0\x02\
    \x01\x05\x12\x03\x12\x04\n\n\x0e\n\x07\x04\0\x03\0\x02\x01\x01\x12\x03\
    \x12\x0b\x11\n\x0e\n\x07\x04\0\x03\0\x02\x01\x03\x12\x03\x12\x14\x15\nQ\
    \n\x06\x04\0\x03\0\x02\x02\x12\x03\x15\x04\x11\x1aB\x20The\x20id\x20of\
    \x20the\x20node.\x20Node\x20ids\x20are\x20only\x20unique\x20inside\x20a\
    \x20partition.\n\n\x0f\n\x07\x04\0\x03\0\x02\x02\x04\x12\x04\x15\x04\x12\
    \x16\n\x0e\n\x07\x04\0\x03\0\x02\x02\x05\x12\x03\x15\x04\t\n\x0e\n\x07\
    \x04\0\x03\0\x02\x02\x01\x12\x03\x15\n\x0c\n\x0e\n\x07\x04\0\x03\0\x02\
    \x02\x03\x12\x03\x15\x0f\x10\n\xc1\x01\n\x06\x04\0\x03\0\x03\0\x12\x04\
    \x1a\x04\x1d\x05\x1a\xb0\x01\x20Inputs\x20of\x20this\x20node.\x20They\
    \x20must\x20be\x20executed\x20before\x20this\x20node\x20can\x20be\n\x20e\
    xecuted.\x20An\x20input\x20is\x20a\x20particular\x20output\x20of\x20anot\
    her\x20node,\x20specified\n\x20by\x20the\x20node\x20id\x20and\x20the\x20\
    output\x20index.\n\n\x0e\n\x07\x04\0\x03\0\x03\0\x01\x12\x03\x1a\x0c\x15\
    \n\x0f\n\x08\x04\0\x03\0\x03\0\x02\0\x12\x03\x1b\x06\x1f\n\x11\n\t\x04\0\
    \x03\0\x03\0\x02\0\x04\x12\x04\x1b\x06\x1a\x17\n\x10\n\t\x04\0\x03\0\x03\
    \0\x02\0\x05\x12\x03\x1b\x06\x0b\n\x10\n\t\x04\0\x03\0\x03\0\x02\0\x01\
    \x12\x03\x1b\x0c\x1a\n\x10\n\t\x04\0\x03\0\x03\0\x02\0\x03\x12\x03\x1b\
    \x1d\x1e\n\x0f\n\x08\x04\0\x03\0\x03\0\x02\x01\x12\x03\x1c\x06\x1f\n\x11\
    \n\t\x04\0\x03\0\x03\0\x02\x01\x04\x12\x04\x1c\x06\x1b\x1f\n\x10\n\t\x04\
    \0\x03\0\x03\0\x02\x01\x05\x12\x03\x1c\x06\x0b\n\x10\n\t\x04\0\x03\0\x03\
    \0\x02\x01\x01\x12\x03\x1c\x0c\x1a\n\x10\n\t\x04\0\x03\0\x03\0\x02\x01\
    \x03\x12\x03\x1c\x1d\x1e\n\r\n\x06\x04\0\x03\0\x02\x03\x12\x03\x1e\x04&\
    \n\x0e\n\x07\x04\0\x03\0\x02\x03\x04\x12\x03\x1e\x04\x0c\n\x0e\n\x07\x04\
    \0\x03\0\x02\x03\x06\x12\x03\x1e\r\x16\n\x0e\n\x07\x04\0\x03\0\x02\x03\
    \x01\x12\x03\x1e\x17!\n\x0e\n\x07\x04\0\x03\0\x02\x03\x03\x12\x03\x1e$%\
    \n'\n\x06\x04\0\x03\0\x03\x01\x12\x04!\x04)\x05\x1a\x17\x20Outputs\x20of\
    \x20this\x20node.\n\n\x0e\n\x07\x04\0\x03\0\x03\x01\x01\x12\x03!\x0c\x16\
    \n\x0f\n\x08\x04\0\x03\0\x03\x01\x02\0\x12\x03\"\x06\x15\n\x11\n\t\x04\0\
    \x03\0\x03\x01\x02\0\x04\x12\x04\"\x06!\x18\n\x10\n\t\x04\0\x03\0\x03\
    \x01\x02\0\x05\x12\x03\"\x06\x0b\n\x10\n\t\x04\0\x03\0\x03\x01\x02\0\x01\
    \x12\x03\"\x0c\x10\n\x10\n\t\x04\0\x03\0\x03\x01\x02\0\x03\x12\x03\"\x13\
    \x14\n\xaf\x01\n\x08\x04\0\x03\0\x03\x01\x02\x01\x12\x03&\x06!\x1a\x9d\
    \x01\x20If\x20>=\x200,\x20the\x20output\x20is\x20an\x20alias\x20of\x20an\
    \x20input.\x20Note\x20that\x20an\x20alias\x20input\n\x20may\x20itself\
    \x20be\x20an\x20alias.\x20The\x20algorithm\x20will\x20therefore\x20need\
    \x20to\x20follow\n\x20those\x20pointers.\n\n\x11\n\t\x04\0\x03\0\x03\x01\
    \x02\x01\x04\x12\x04&\x06\"\x15\n\x10\n\t\x04\0\x03\0\x03\x01\x02\x01\
    \x05\x12\x03&\x06\x0b\n\x10\n\t\x04\0\x03\0\x03\x01\x02\x01\x01\x12\x03&\
    \x0c\x1c\n\x10\n\t\x04\0\x03\0\x03\x01\x02\x01\x03\x12\x03&\x1f\x20\n\
    \x0f\n\x08\x04\0\x03\0\x03\x01\x02\x02\x12\x03'\x06!\n\x11\n\t\x04\0\x03\
    \0\x03\x01\x02\x02\x04\x12\x04'\x06&!\n\x10\n\t\x04\0\x03\0\x03\x01\x02\
    \x02\x06\x12\x03'\x06\x16\n\x10\n\t\x04\0\x03\0\x03\x01\x02\x02\x01\x12\
    \x03'\x17\x1c\n\x10\n\t\x04\0\x03\0\x03\x01\x02\x02\x03\x12\x03'\x1f\x20\
    \n\x0f\n\x08\x04\0\x03\0\x03\x01\x02\x03\x12\x03(\x06\x19\n\x11\n\t\x04\
    \0\x03\0\x03\x01\x02\x03\x04\x12\x04(\x06'!\n\x10\n\t\x04\0\x03\0\x03\
    \x01\x02\x03\x06\x12\x03(\x06\x0e\n\x10\n\t\x04\0\x03\0\x03\x01\x02\x03\
    \x01\x12\x03(\x0f\x14\n\x10\n\t\x04\0\x03\0\x03\x01\x02\x03\x03\x12\x03(\
    \x17\x18\n\r\n\x06\x04\0\x03\0\x02\x04\x12\x03*\x04(\n\x0e\n\x07\x04\0\
    \x03\0\x02\x04\x04\x12\x03*\x04\x0c\n\x0e\n\x07\x04\0\x03\0\x02\x04\x06\
    \x12\x03*\r\x17\n\x0e\n\x07\x04\0\x03\0\x02\x04\x01\x12\x03*\x18#\n\x0e\
    \n\x07\x04\0\x03\0\x02\x04\x03\x12\x03*&'\n4\n\x06\x04\0\x03\0\x02\x05\
    \x12\x03-\x04$\x1a%\x20Temporary\x20memory\x20used\x20by\x20this\x20node\
    .\n\n\x0f\n\x07\x04\0\x03\0\x02\x05\x04\x12\x04-\x04*(\n\x0e\n\x07\x04\0\
    \x03\0\x02\x05\x05\x12\x03-\x04\t\n\x0e\n\x07\x04\0\x03\0\x02\x05\x01\
    \x12\x03-\n\x1f\n\x0e\n\x07\x04\0\x03\0\x02\x05\x03\x12\x03-\"#\n\r\n\
    \x06\x04\0\x03\0\x02\x06\x12\x03/\x04%\n\x0f\n\x07\x04\0\x03\0\x02\x06\
    \x04\x12\x04/\x04-$\n\x0e\n\x07\x04\0\x03\0\x02\x06\x05\x12\x03/\x04\t\n\
    \x0e\n\x07\x04\0\x03\0\x02\x06\x01\x12\x03/\n\x1f\n\x0e\n\x07\x04\0\x03\
    \0\x02\x06\x03\x12\x03/\"$\n\r\n\x06\x04\0\x03\0\x02\x07\x12\x030\x04'\n\
    \x0f\n\x07\x04\0\x03\0\x02\x07\x04\x12\x040\x04/%\n\x0e\n\x07\x04\0\x03\
    \0\x02\x07\x05\x12\x030\x04\t\n\x0e\n\x07\x04\0\x03\0\x02\x07\x01\x12\
    \x030\n!\n\x0e\n\x07\x04\0\x03\0\x02\x07\x03\x12\x030$&\n\r\n\x06\x04\0\
    \x03\0\x02\x08\x12\x031\x04+\n\x0f\n\x07\x04\0\x03\0\x02\x08\x04\x12\x04\
    1\x040'\n\x0e\n\x07\x04\0\x03\0\x02\x08\x05\x12\x031\x04\t\n\x0e\n\x07\
    \x04\0\x03\0\x02\x08\x01\x12\x031\n%\n\x0e\n\x07\x04\0\x03\0\x02\x08\x03\
    \x12\x031(*\n\r\n\x06\x04\0\x03\0\x02\t\x12\x032\x04-\n\x0f\n\x07\x04\0\
    \x03\0\x02\t\x04\x12\x042\x041+\n\x0e\n\x07\x04\0\x03\0\x02\t\x05\x12\
    \x032\x04\t\n\x0e\n\x07\x04\0\x03\0\x02\t\x01\x12\x032\n'\n\x0e\n\x07\
    \x04\0\x03\0\x02\t\x03\x12\x032*,\nR\n\x06\x04\0\x03\0\x02\n\x12\x035\
    \x04\x1b\x1aC\x20Estimate\x20of\x20the\x20computational\x20cost\x20of\
    \x20this\x20node,\x20in\x20microseconds.\n\n\x0f\n\x07\x04\0\x03\0\x02\n\
    \x04\x12\x045\x042-\n\x0e\n\x07\x04\0\x03\0\x02\n\x05\x12\x035\x04\t\n\
    \x0e\n\x07\x04\0\x03\0\x02\n\x01\x12\x035\n\x16\n\x0e\n\x07\x04\0\x03\0\
    \x02\n\x03\x12\x035\x19\x1a\n^\n\x06\x04\0\x03\0\x02\x0b\x12\x039\x04\
    \x1c\x1aO\x20Analytical\x20estimate\x20of\x20the\x20computational\x20cos\
    t\x20of\x20this\x20node,\x20in\n\x20microseconds.\n\n\x0f\n\x07\x04\0\
    \x03\0\x02\x0b\x04\x12\x049\x045\x1b\n\x0e\n\x07\x04\0\x03\0\x02\x0b\x05\
    \x12\x039\x04\t\n\x0e\n\x07\x04\0\x03\0\x02\x0b\x01\x12\x039\n\x16\n\x0e\
    \n\x07\x04\0\x03\0\x02\x0b\x03\x12\x039\x19\x1b\n^\n\x06\x04\0\x03\0\x02\
    \x0c\x12\x03=\x04\x1b\x1aO\x20Analytical\x20estimate\x20of\x20the\x20mem\
    ory\x20access\x20cost\x20of\x20this\x20node,\x20in\n\x20microseconds.\n\
    \n\x0f\n\x07\x04\0\x03\0\x02\x0c\x04\x12\x04=\x049\x1c\n\x0e\n\x07\x04\0\
    \x03\0\x02\x0c\x05\x12\x03=\x04\t\n\x0e\n\x07\x04\0\x03\0\x02\x0c\x01\
    \x12\x03=\n\x15\n\x0e\n\x07\x04\0\x03\0\x02\x0c\x03\x12\x03=\x18\x1a\n\
    \x9d\x01\n\x06\x04\0\x03\0\x02\r\x12\x03A\x04\x16\x1a\x8d\x01\x20If\x20t\
    rue,\x20the\x20output\x20is\x20permanent:\x20it\x20can't\x20be\x20discar\
    ded,\x20because\x20this\n\x20node\x20is\x20part\x20of\x20the\x20\"final\
    \x20output\".\x20Nodes\x20may\x20depend\x20on\x20final\x20nodes.\n\n\x0f\
    \n\x07\x04\0\x03\0\x02\r\x04\x12\x04A\x04=\x1b\n\x0e\n\x07\x04\0\x03\0\
    \x02\r\x05\x12\x03A\x04\x08\n\x0e\n\x07\x04\0\x03\0\x02\r\x01\x12\x03A\t\
    \x11\n\x0e\n\x07\x04\0\x03\0\x02\r\x03\x12\x03A\x14\x15\n9\n\x06\x04\0\
    \x03\0\x02\x0e\x12\x03D\x04%\x1a*\x20Ids\x20of\x20the\x20control\x20inpu\
    ts\x20for\x20this\x20node.\n\n\x0e\n\x07\x04\0\x03\0\x02\x0e\x04\x12\x03\
    D\x04\x0c\n\x0e\n\x07\x04\0\x03\0\x02\x0e\x05\x12\x03D\r\x12\n\x0e\n\x07\
    \x04\0\x03\0\x02\x0e\x01\x12\x03D\x13\x20\n\x0e\n\x07\x04\0\x03\0\x02\
    \x0e\x03\x12\x03D#$\n\x0b\n\x04\x04\0\x02\0\x12\x03F\x02\x19\n\x0c\n\x05\
    \x04\0\x02\0\x04\x12\x03F\x02\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03F\x0b\
    \x0f\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03F\x10\x14\n\x0c\n\x05\x04\0\x02\
    \0\x03\x12\x03F\x17\x18b\x06proto3\
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
