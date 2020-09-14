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
pub struct AllocatorMemoryUsed {
    // message fields
    pub allocator_name: ::std::string::String,
    pub total_bytes: i64,
    pub peak_bytes: i64,
    pub live_bytes: i64,
    pub allocator_bytes_in_use: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AllocatorMemoryUsed {}

impl AllocatorMemoryUsed {
    pub fn new() -> AllocatorMemoryUsed {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AllocatorMemoryUsed {
        static mut instance: ::protobuf::lazy::Lazy<AllocatorMemoryUsed> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AllocatorMemoryUsed,
        };
        unsafe {
            instance.get(AllocatorMemoryUsed::new)
        }
    }

    // string allocator_name = 1;

    pub fn clear_allocator_name(&mut self) {
        self.allocator_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_allocator_name(&mut self, v: ::std::string::String) {
        self.allocator_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allocator_name(&mut self) -> &mut ::std::string::String {
        &mut self.allocator_name
    }

    // Take field
    pub fn take_allocator_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.allocator_name, ::std::string::String::new())
    }

    pub fn get_allocator_name(&self) -> &str {
        &self.allocator_name
    }

    fn get_allocator_name_for_reflect(&self) -> &::std::string::String {
        &self.allocator_name
    }

    fn mut_allocator_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.allocator_name
    }

    // int64 total_bytes = 2;

    pub fn clear_total_bytes(&mut self) {
        self.total_bytes = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_bytes(&mut self, v: i64) {
        self.total_bytes = v;
    }

    pub fn get_total_bytes(&self) -> i64 {
        self.total_bytes
    }

    fn get_total_bytes_for_reflect(&self) -> &i64 {
        &self.total_bytes
    }

    fn mut_total_bytes_for_reflect(&mut self) -> &mut i64 {
        &mut self.total_bytes
    }

    // int64 peak_bytes = 3;

    pub fn clear_peak_bytes(&mut self) {
        self.peak_bytes = 0;
    }

    // Param is passed by value, moved
    pub fn set_peak_bytes(&mut self, v: i64) {
        self.peak_bytes = v;
    }

    pub fn get_peak_bytes(&self) -> i64 {
        self.peak_bytes
    }

    fn get_peak_bytes_for_reflect(&self) -> &i64 {
        &self.peak_bytes
    }

    fn mut_peak_bytes_for_reflect(&mut self) -> &mut i64 {
        &mut self.peak_bytes
    }

    // int64 live_bytes = 4;

    pub fn clear_live_bytes(&mut self) {
        self.live_bytes = 0;
    }

    // Param is passed by value, moved
    pub fn set_live_bytes(&mut self, v: i64) {
        self.live_bytes = v;
    }

    pub fn get_live_bytes(&self) -> i64 {
        self.live_bytes
    }

    fn get_live_bytes_for_reflect(&self) -> &i64 {
        &self.live_bytes
    }

    fn mut_live_bytes_for_reflect(&mut self) -> &mut i64 {
        &mut self.live_bytes
    }

    // int64 allocator_bytes_in_use = 5;

    pub fn clear_allocator_bytes_in_use(&mut self) {
        self.allocator_bytes_in_use = 0;
    }

    // Param is passed by value, moved
    pub fn set_allocator_bytes_in_use(&mut self, v: i64) {
        self.allocator_bytes_in_use = v;
    }

    pub fn get_allocator_bytes_in_use(&self) -> i64 {
        self.allocator_bytes_in_use
    }

    fn get_allocator_bytes_in_use_for_reflect(&self) -> &i64 {
        &self.allocator_bytes_in_use
    }

    fn mut_allocator_bytes_in_use_for_reflect(&mut self) -> &mut i64 {
        &mut self.allocator_bytes_in_use
    }
}

impl ::protobuf::Message for AllocatorMemoryUsed {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.allocator_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.total_bytes = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.peak_bytes = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.live_bytes = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.allocator_bytes_in_use = tmp;
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
        if !self.allocator_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.allocator_name);
        }
        if self.total_bytes != 0 {
            my_size += ::protobuf::rt::value_size(2, self.total_bytes, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.peak_bytes != 0 {
            my_size += ::protobuf::rt::value_size(3, self.peak_bytes, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.live_bytes != 0 {
            my_size += ::protobuf::rt::value_size(4, self.live_bytes, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.allocator_bytes_in_use != 0 {
            my_size += ::protobuf::rt::value_size(5, self.allocator_bytes_in_use, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.allocator_name.is_empty() {
            os.write_string(1, &self.allocator_name)?;
        }
        if self.total_bytes != 0 {
            os.write_int64(2, self.total_bytes)?;
        }
        if self.peak_bytes != 0 {
            os.write_int64(3, self.peak_bytes)?;
        }
        if self.live_bytes != 0 {
            os.write_int64(4, self.live_bytes)?;
        }
        if self.allocator_bytes_in_use != 0 {
            os.write_int64(5, self.allocator_bytes_in_use)?;
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

impl ::protobuf::MessageStatic for AllocatorMemoryUsed {
    fn new() -> AllocatorMemoryUsed {
        AllocatorMemoryUsed::new()
    }

    fn descriptor_static(_: ::std::option::Option<AllocatorMemoryUsed>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "allocator_name",
                    AllocatorMemoryUsed::get_allocator_name_for_reflect,
                    AllocatorMemoryUsed::mut_allocator_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "total_bytes",
                    AllocatorMemoryUsed::get_total_bytes_for_reflect,
                    AllocatorMemoryUsed::mut_total_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "peak_bytes",
                    AllocatorMemoryUsed::get_peak_bytes_for_reflect,
                    AllocatorMemoryUsed::mut_peak_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "live_bytes",
                    AllocatorMemoryUsed::get_live_bytes_for_reflect,
                    AllocatorMemoryUsed::mut_live_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "allocator_bytes_in_use",
                    AllocatorMemoryUsed::get_allocator_bytes_in_use_for_reflect,
                    AllocatorMemoryUsed::mut_allocator_bytes_in_use_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AllocatorMemoryUsed>(
                    "AllocatorMemoryUsed",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AllocatorMemoryUsed {
    fn clear(&mut self) {
        self.clear_allocator_name();
        self.clear_total_bytes();
        self.clear_peak_bytes();
        self.clear_live_bytes();
        self.clear_allocator_bytes_in_use();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AllocatorMemoryUsed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AllocatorMemoryUsed {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NodeOutput {
    // message fields
    pub slot: i32,
    pub tensor_description: ::protobuf::SingularPtrField<super::tensor_description::TensorDescription>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NodeOutput {}

impl NodeOutput {
    pub fn new() -> NodeOutput {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NodeOutput {
        static mut instance: ::protobuf::lazy::Lazy<NodeOutput> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NodeOutput,
        };
        unsafe {
            instance.get(NodeOutput::new)
        }
    }

    // int32 slot = 1;

    pub fn clear_slot(&mut self) {
        self.slot = 0;
    }

    // Param is passed by value, moved
    pub fn set_slot(&mut self, v: i32) {
        self.slot = v;
    }

    pub fn get_slot(&self) -> i32 {
        self.slot
    }

    fn get_slot_for_reflect(&self) -> &i32 {
        &self.slot
    }

    fn mut_slot_for_reflect(&mut self) -> &mut i32 {
        &mut self.slot
    }

    // .tensorflow.TensorDescription tensor_description = 3;

    pub fn clear_tensor_description(&mut self) {
        self.tensor_description.clear();
    }

    pub fn has_tensor_description(&self) -> bool {
        self.tensor_description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tensor_description(&mut self, v: super::tensor_description::TensorDescription) {
        self.tensor_description = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tensor_description(&mut self) -> &mut super::tensor_description::TensorDescription {
        if self.tensor_description.is_none() {
            self.tensor_description.set_default();
        }
        self.tensor_description.as_mut().unwrap()
    }

    // Take field
    pub fn take_tensor_description(&mut self) -> super::tensor_description::TensorDescription {
        self.tensor_description.take().unwrap_or_else(|| super::tensor_description::TensorDescription::new())
    }

    pub fn get_tensor_description(&self) -> &super::tensor_description::TensorDescription {
        self.tensor_description.as_ref().unwrap_or_else(|| super::tensor_description::TensorDescription::default_instance())
    }

    fn get_tensor_description_for_reflect(&self) -> &::protobuf::SingularPtrField<super::tensor_description::TensorDescription> {
        &self.tensor_description
    }

    fn mut_tensor_description_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::tensor_description::TensorDescription> {
        &mut self.tensor_description
    }
}

impl ::protobuf::Message for NodeOutput {
    fn is_initialized(&self) -> bool {
        for v in &self.tensor_description {
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
                    let tmp = is.read_int32()?;
                    self.slot = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tensor_description)?;
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
        if self.slot != 0 {
            my_size += ::protobuf::rt::value_size(1, self.slot, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.tensor_description.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.slot != 0 {
            os.write_int32(1, self.slot)?;
        }
        if let Some(ref v) = self.tensor_description.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for NodeOutput {
    fn new() -> NodeOutput {
        NodeOutput::new()
    }

    fn descriptor_static(_: ::std::option::Option<NodeOutput>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "slot",
                    NodeOutput::get_slot_for_reflect,
                    NodeOutput::mut_slot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tensor_description::TensorDescription>>(
                    "tensor_description",
                    NodeOutput::get_tensor_description_for_reflect,
                    NodeOutput::mut_tensor_description_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NodeOutput>(
                    "NodeOutput",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NodeOutput {
    fn clear(&mut self) {
        self.clear_slot();
        self.clear_tensor_description();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NodeOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NodeOutput {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MemoryStats {
    // message fields
    pub host_temp_memory_size: i64,
    pub device_temp_memory_size: i64,
    pub host_persistent_memory_size: i64,
    pub device_persistent_memory_size: i64,
    pub host_persistent_tensor_alloc_ids: ::std::vec::Vec<i64>,
    pub device_persistent_tensor_alloc_ids: ::std::vec::Vec<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MemoryStats {}

impl MemoryStats {
    pub fn new() -> MemoryStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MemoryStats {
        static mut instance: ::protobuf::lazy::Lazy<MemoryStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MemoryStats,
        };
        unsafe {
            instance.get(MemoryStats::new)
        }
    }

    // int64 host_temp_memory_size = 1;

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

    // int64 device_temp_memory_size = 2;

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

    // int64 host_persistent_memory_size = 3;

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

    // int64 device_persistent_memory_size = 4;

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

    // repeated int64 host_persistent_tensor_alloc_ids = 5;

    pub fn clear_host_persistent_tensor_alloc_ids(&mut self) {
        self.host_persistent_tensor_alloc_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_host_persistent_tensor_alloc_ids(&mut self, v: ::std::vec::Vec<i64>) {
        self.host_persistent_tensor_alloc_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_host_persistent_tensor_alloc_ids(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.host_persistent_tensor_alloc_ids
    }

    // Take field
    pub fn take_host_persistent_tensor_alloc_ids(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.host_persistent_tensor_alloc_ids, ::std::vec::Vec::new())
    }

    pub fn get_host_persistent_tensor_alloc_ids(&self) -> &[i64] {
        &self.host_persistent_tensor_alloc_ids
    }

    fn get_host_persistent_tensor_alloc_ids_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.host_persistent_tensor_alloc_ids
    }

    fn mut_host_persistent_tensor_alloc_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.host_persistent_tensor_alloc_ids
    }

    // repeated int64 device_persistent_tensor_alloc_ids = 6;

    pub fn clear_device_persistent_tensor_alloc_ids(&mut self) {
        self.device_persistent_tensor_alloc_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_device_persistent_tensor_alloc_ids(&mut self, v: ::std::vec::Vec<i64>) {
        self.device_persistent_tensor_alloc_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_device_persistent_tensor_alloc_ids(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.device_persistent_tensor_alloc_ids
    }

    // Take field
    pub fn take_device_persistent_tensor_alloc_ids(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.device_persistent_tensor_alloc_ids, ::std::vec::Vec::new())
    }

    pub fn get_device_persistent_tensor_alloc_ids(&self) -> &[i64] {
        &self.device_persistent_tensor_alloc_ids
    }

    fn get_device_persistent_tensor_alloc_ids_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.device_persistent_tensor_alloc_ids
    }

    fn mut_device_persistent_tensor_alloc_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.device_persistent_tensor_alloc_ids
    }
}

impl ::protobuf::Message for MemoryStats {
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
                    let tmp = is.read_int64()?;
                    self.host_temp_memory_size = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.device_temp_memory_size = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.host_persistent_memory_size = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.device_persistent_memory_size = tmp;
                },
                5 => {
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.host_persistent_tensor_alloc_ids)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.device_persistent_tensor_alloc_ids)?;
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
        if self.host_temp_memory_size != 0 {
            my_size += ::protobuf::rt::value_size(1, self.host_temp_memory_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.device_temp_memory_size != 0 {
            my_size += ::protobuf::rt::value_size(2, self.device_temp_memory_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.host_persistent_memory_size != 0 {
            my_size += ::protobuf::rt::value_size(3, self.host_persistent_memory_size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.device_persistent_memory_size != 0 {
            my_size += ::protobuf::rt::value_size(4, self.device_persistent_memory_size, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.host_persistent_tensor_alloc_ids {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.device_persistent_tensor_alloc_ids {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.host_temp_memory_size != 0 {
            os.write_int64(1, self.host_temp_memory_size)?;
        }
        if self.device_temp_memory_size != 0 {
            os.write_int64(2, self.device_temp_memory_size)?;
        }
        if self.host_persistent_memory_size != 0 {
            os.write_int64(3, self.host_persistent_memory_size)?;
        }
        if self.device_persistent_memory_size != 0 {
            os.write_int64(4, self.device_persistent_memory_size)?;
        }
        for v in &self.host_persistent_tensor_alloc_ids {
            os.write_int64(5, *v)?;
        };
        for v in &self.device_persistent_tensor_alloc_ids {
            os.write_int64(6, *v)?;
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

impl ::protobuf::MessageStatic for MemoryStats {
    fn new() -> MemoryStats {
        MemoryStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<MemoryStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "host_temp_memory_size",
                    MemoryStats::get_host_temp_memory_size_for_reflect,
                    MemoryStats::mut_host_temp_memory_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "device_temp_memory_size",
                    MemoryStats::get_device_temp_memory_size_for_reflect,
                    MemoryStats::mut_device_temp_memory_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "host_persistent_memory_size",
                    MemoryStats::get_host_persistent_memory_size_for_reflect,
                    MemoryStats::mut_host_persistent_memory_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "device_persistent_memory_size",
                    MemoryStats::get_device_persistent_memory_size_for_reflect,
                    MemoryStats::mut_device_persistent_memory_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "host_persistent_tensor_alloc_ids",
                    MemoryStats::get_host_persistent_tensor_alloc_ids_for_reflect,
                    MemoryStats::mut_host_persistent_tensor_alloc_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "device_persistent_tensor_alloc_ids",
                    MemoryStats::get_device_persistent_tensor_alloc_ids_for_reflect,
                    MemoryStats::mut_device_persistent_tensor_alloc_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MemoryStats>(
                    "MemoryStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MemoryStats {
    fn clear(&mut self) {
        self.clear_host_temp_memory_size();
        self.clear_device_temp_memory_size();
        self.clear_host_persistent_memory_size();
        self.clear_device_persistent_memory_size();
        self.clear_host_persistent_tensor_alloc_ids();
        self.clear_device_persistent_tensor_alloc_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MemoryStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MemoryStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NodeExecStats {
    // message fields
    pub node_name: ::std::string::String,
    pub all_start_micros: i64,
    pub op_start_rel_micros: i64,
    pub op_end_rel_micros: i64,
    pub all_end_rel_micros: i64,
    pub memory: ::protobuf::RepeatedField<AllocatorMemoryUsed>,
    pub output: ::protobuf::RepeatedField<NodeOutput>,
    pub timeline_label: ::std::string::String,
    pub scheduled_micros: i64,
    pub thread_id: u32,
    pub referenced_tensor: ::protobuf::RepeatedField<super::allocation_description::AllocationDescription>,
    pub memory_stats: ::protobuf::SingularPtrField<MemoryStats>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NodeExecStats {}

impl NodeExecStats {
    pub fn new() -> NodeExecStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NodeExecStats {
        static mut instance: ::protobuf::lazy::Lazy<NodeExecStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NodeExecStats,
        };
        unsafe {
            instance.get(NodeExecStats::new)
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

    // int64 all_start_micros = 2;

    pub fn clear_all_start_micros(&mut self) {
        self.all_start_micros = 0;
    }

    // Param is passed by value, moved
    pub fn set_all_start_micros(&mut self, v: i64) {
        self.all_start_micros = v;
    }

    pub fn get_all_start_micros(&self) -> i64 {
        self.all_start_micros
    }

    fn get_all_start_micros_for_reflect(&self) -> &i64 {
        &self.all_start_micros
    }

    fn mut_all_start_micros_for_reflect(&mut self) -> &mut i64 {
        &mut self.all_start_micros
    }

    // int64 op_start_rel_micros = 3;

    pub fn clear_op_start_rel_micros(&mut self) {
        self.op_start_rel_micros = 0;
    }

    // Param is passed by value, moved
    pub fn set_op_start_rel_micros(&mut self, v: i64) {
        self.op_start_rel_micros = v;
    }

    pub fn get_op_start_rel_micros(&self) -> i64 {
        self.op_start_rel_micros
    }

    fn get_op_start_rel_micros_for_reflect(&self) -> &i64 {
        &self.op_start_rel_micros
    }

    fn mut_op_start_rel_micros_for_reflect(&mut self) -> &mut i64 {
        &mut self.op_start_rel_micros
    }

    // int64 op_end_rel_micros = 4;

    pub fn clear_op_end_rel_micros(&mut self) {
        self.op_end_rel_micros = 0;
    }

    // Param is passed by value, moved
    pub fn set_op_end_rel_micros(&mut self, v: i64) {
        self.op_end_rel_micros = v;
    }

    pub fn get_op_end_rel_micros(&self) -> i64 {
        self.op_end_rel_micros
    }

    fn get_op_end_rel_micros_for_reflect(&self) -> &i64 {
        &self.op_end_rel_micros
    }

    fn mut_op_end_rel_micros_for_reflect(&mut self) -> &mut i64 {
        &mut self.op_end_rel_micros
    }

    // int64 all_end_rel_micros = 5;

    pub fn clear_all_end_rel_micros(&mut self) {
        self.all_end_rel_micros = 0;
    }

    // Param is passed by value, moved
    pub fn set_all_end_rel_micros(&mut self, v: i64) {
        self.all_end_rel_micros = v;
    }

    pub fn get_all_end_rel_micros(&self) -> i64 {
        self.all_end_rel_micros
    }

    fn get_all_end_rel_micros_for_reflect(&self) -> &i64 {
        &self.all_end_rel_micros
    }

    fn mut_all_end_rel_micros_for_reflect(&mut self) -> &mut i64 {
        &mut self.all_end_rel_micros
    }

    // repeated .tensorflow.AllocatorMemoryUsed memory = 6;

    pub fn clear_memory(&mut self) {
        self.memory.clear();
    }

    // Param is passed by value, moved
    pub fn set_memory(&mut self, v: ::protobuf::RepeatedField<AllocatorMemoryUsed>) {
        self.memory = v;
    }

    // Mutable pointer to the field.
    pub fn mut_memory(&mut self) -> &mut ::protobuf::RepeatedField<AllocatorMemoryUsed> {
        &mut self.memory
    }

    // Take field
    pub fn take_memory(&mut self) -> ::protobuf::RepeatedField<AllocatorMemoryUsed> {
        ::std::mem::replace(&mut self.memory, ::protobuf::RepeatedField::new())
    }

    pub fn get_memory(&self) -> &[AllocatorMemoryUsed] {
        &self.memory
    }

    fn get_memory_for_reflect(&self) -> &::protobuf::RepeatedField<AllocatorMemoryUsed> {
        &self.memory
    }

    fn mut_memory_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<AllocatorMemoryUsed> {
        &mut self.memory
    }

    // repeated .tensorflow.NodeOutput output = 7;

    pub fn clear_output(&mut self) {
        self.output.clear();
    }

    // Param is passed by value, moved
    pub fn set_output(&mut self, v: ::protobuf::RepeatedField<NodeOutput>) {
        self.output = v;
    }

    // Mutable pointer to the field.
    pub fn mut_output(&mut self) -> &mut ::protobuf::RepeatedField<NodeOutput> {
        &mut self.output
    }

    // Take field
    pub fn take_output(&mut self) -> ::protobuf::RepeatedField<NodeOutput> {
        ::std::mem::replace(&mut self.output, ::protobuf::RepeatedField::new())
    }

    pub fn get_output(&self) -> &[NodeOutput] {
        &self.output
    }

    fn get_output_for_reflect(&self) -> &::protobuf::RepeatedField<NodeOutput> {
        &self.output
    }

    fn mut_output_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<NodeOutput> {
        &mut self.output
    }

    // string timeline_label = 8;

    pub fn clear_timeline_label(&mut self) {
        self.timeline_label.clear();
    }

    // Param is passed by value, moved
    pub fn set_timeline_label(&mut self, v: ::std::string::String) {
        self.timeline_label = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_timeline_label(&mut self) -> &mut ::std::string::String {
        &mut self.timeline_label
    }

    // Take field
    pub fn take_timeline_label(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.timeline_label, ::std::string::String::new())
    }

    pub fn get_timeline_label(&self) -> &str {
        &self.timeline_label
    }

    fn get_timeline_label_for_reflect(&self) -> &::std::string::String {
        &self.timeline_label
    }

    fn mut_timeline_label_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.timeline_label
    }

    // int64 scheduled_micros = 9;

    pub fn clear_scheduled_micros(&mut self) {
        self.scheduled_micros = 0;
    }

    // Param is passed by value, moved
    pub fn set_scheduled_micros(&mut self, v: i64) {
        self.scheduled_micros = v;
    }

    pub fn get_scheduled_micros(&self) -> i64 {
        self.scheduled_micros
    }

    fn get_scheduled_micros_for_reflect(&self) -> &i64 {
        &self.scheduled_micros
    }

    fn mut_scheduled_micros_for_reflect(&mut self) -> &mut i64 {
        &mut self.scheduled_micros
    }

    // uint32 thread_id = 10;

    pub fn clear_thread_id(&mut self) {
        self.thread_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_thread_id(&mut self, v: u32) {
        self.thread_id = v;
    }

    pub fn get_thread_id(&self) -> u32 {
        self.thread_id
    }

    fn get_thread_id_for_reflect(&self) -> &u32 {
        &self.thread_id
    }

    fn mut_thread_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.thread_id
    }

    // repeated .tensorflow.AllocationDescription referenced_tensor = 11;

    pub fn clear_referenced_tensor(&mut self) {
        self.referenced_tensor.clear();
    }

    // Param is passed by value, moved
    pub fn set_referenced_tensor(&mut self, v: ::protobuf::RepeatedField<super::allocation_description::AllocationDescription>) {
        self.referenced_tensor = v;
    }

    // Mutable pointer to the field.
    pub fn mut_referenced_tensor(&mut self) -> &mut ::protobuf::RepeatedField<super::allocation_description::AllocationDescription> {
        &mut self.referenced_tensor
    }

    // Take field
    pub fn take_referenced_tensor(&mut self) -> ::protobuf::RepeatedField<super::allocation_description::AllocationDescription> {
        ::std::mem::replace(&mut self.referenced_tensor, ::protobuf::RepeatedField::new())
    }

    pub fn get_referenced_tensor(&self) -> &[super::allocation_description::AllocationDescription] {
        &self.referenced_tensor
    }

    fn get_referenced_tensor_for_reflect(&self) -> &::protobuf::RepeatedField<super::allocation_description::AllocationDescription> {
        &self.referenced_tensor
    }

    fn mut_referenced_tensor_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::allocation_description::AllocationDescription> {
        &mut self.referenced_tensor
    }

    // .tensorflow.MemoryStats memory_stats = 12;

    pub fn clear_memory_stats(&mut self) {
        self.memory_stats.clear();
    }

    pub fn has_memory_stats(&self) -> bool {
        self.memory_stats.is_some()
    }

    // Param is passed by value, moved
    pub fn set_memory_stats(&mut self, v: MemoryStats) {
        self.memory_stats = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_memory_stats(&mut self) -> &mut MemoryStats {
        if self.memory_stats.is_none() {
            self.memory_stats.set_default();
        }
        self.memory_stats.as_mut().unwrap()
    }

    // Take field
    pub fn take_memory_stats(&mut self) -> MemoryStats {
        self.memory_stats.take().unwrap_or_else(|| MemoryStats::new())
    }

    pub fn get_memory_stats(&self) -> &MemoryStats {
        self.memory_stats.as_ref().unwrap_or_else(|| MemoryStats::default_instance())
    }

    fn get_memory_stats_for_reflect(&self) -> &::protobuf::SingularPtrField<MemoryStats> {
        &self.memory_stats
    }

    fn mut_memory_stats_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MemoryStats> {
        &mut self.memory_stats
    }
}

impl ::protobuf::Message for NodeExecStats {
    fn is_initialized(&self) -> bool {
        for v in &self.memory {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.output {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.referenced_tensor {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.memory_stats {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.node_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.all_start_micros = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.op_start_rel_micros = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.op_end_rel_micros = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.all_end_rel_micros = tmp;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.memory)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.output)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.timeline_label)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.scheduled_micros = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.thread_id = tmp;
                },
                11 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.referenced_tensor)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.memory_stats)?;
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
        if self.all_start_micros != 0 {
            my_size += ::protobuf::rt::value_size(2, self.all_start_micros, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.op_start_rel_micros != 0 {
            my_size += ::protobuf::rt::value_size(3, self.op_start_rel_micros, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.op_end_rel_micros != 0 {
            my_size += ::protobuf::rt::value_size(4, self.op_end_rel_micros, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.all_end_rel_micros != 0 {
            my_size += ::protobuf::rt::value_size(5, self.all_end_rel_micros, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.memory {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.output {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.timeline_label.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.timeline_label);
        }
        if self.scheduled_micros != 0 {
            my_size += ::protobuf::rt::value_size(9, self.scheduled_micros, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.thread_id != 0 {
            my_size += ::protobuf::rt::value_size(10, self.thread_id, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.referenced_tensor {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.memory_stats.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.node_name.is_empty() {
            os.write_string(1, &self.node_name)?;
        }
        if self.all_start_micros != 0 {
            os.write_int64(2, self.all_start_micros)?;
        }
        if self.op_start_rel_micros != 0 {
            os.write_int64(3, self.op_start_rel_micros)?;
        }
        if self.op_end_rel_micros != 0 {
            os.write_int64(4, self.op_end_rel_micros)?;
        }
        if self.all_end_rel_micros != 0 {
            os.write_int64(5, self.all_end_rel_micros)?;
        }
        for v in &self.memory {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.output {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.timeline_label.is_empty() {
            os.write_string(8, &self.timeline_label)?;
        }
        if self.scheduled_micros != 0 {
            os.write_int64(9, self.scheduled_micros)?;
        }
        if self.thread_id != 0 {
            os.write_uint32(10, self.thread_id)?;
        }
        for v in &self.referenced_tensor {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.memory_stats.as_ref() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for NodeExecStats {
    fn new() -> NodeExecStats {
        NodeExecStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<NodeExecStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "node_name",
                    NodeExecStats::get_node_name_for_reflect,
                    NodeExecStats::mut_node_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "all_start_micros",
                    NodeExecStats::get_all_start_micros_for_reflect,
                    NodeExecStats::mut_all_start_micros_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "op_start_rel_micros",
                    NodeExecStats::get_op_start_rel_micros_for_reflect,
                    NodeExecStats::mut_op_start_rel_micros_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "op_end_rel_micros",
                    NodeExecStats::get_op_end_rel_micros_for_reflect,
                    NodeExecStats::mut_op_end_rel_micros_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "all_end_rel_micros",
                    NodeExecStats::get_all_end_rel_micros_for_reflect,
                    NodeExecStats::mut_all_end_rel_micros_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AllocatorMemoryUsed>>(
                    "memory",
                    NodeExecStats::get_memory_for_reflect,
                    NodeExecStats::mut_memory_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NodeOutput>>(
                    "output",
                    NodeExecStats::get_output_for_reflect,
                    NodeExecStats::mut_output_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "timeline_label",
                    NodeExecStats::get_timeline_label_for_reflect,
                    NodeExecStats::mut_timeline_label_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "scheduled_micros",
                    NodeExecStats::get_scheduled_micros_for_reflect,
                    NodeExecStats::mut_scheduled_micros_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "thread_id",
                    NodeExecStats::get_thread_id_for_reflect,
                    NodeExecStats::mut_thread_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::allocation_description::AllocationDescription>>(
                    "referenced_tensor",
                    NodeExecStats::get_referenced_tensor_for_reflect,
                    NodeExecStats::mut_referenced_tensor_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MemoryStats>>(
                    "memory_stats",
                    NodeExecStats::get_memory_stats_for_reflect,
                    NodeExecStats::mut_memory_stats_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NodeExecStats>(
                    "NodeExecStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NodeExecStats {
    fn clear(&mut self) {
        self.clear_node_name();
        self.clear_all_start_micros();
        self.clear_op_start_rel_micros();
        self.clear_op_end_rel_micros();
        self.clear_all_end_rel_micros();
        self.clear_memory();
        self.clear_output();
        self.clear_timeline_label();
        self.clear_scheduled_micros();
        self.clear_thread_id();
        self.clear_referenced_tensor();
        self.clear_memory_stats();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NodeExecStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NodeExecStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeviceStepStats {
    // message fields
    pub device: ::std::string::String,
    pub node_stats: ::protobuf::RepeatedField<NodeExecStats>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeviceStepStats {}

impl DeviceStepStats {
    pub fn new() -> DeviceStepStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeviceStepStats {
        static mut instance: ::protobuf::lazy::Lazy<DeviceStepStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeviceStepStats,
        };
        unsafe {
            instance.get(DeviceStepStats::new)
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

    // repeated .tensorflow.NodeExecStats node_stats = 2;

    pub fn clear_node_stats(&mut self) {
        self.node_stats.clear();
    }

    // Param is passed by value, moved
    pub fn set_node_stats(&mut self, v: ::protobuf::RepeatedField<NodeExecStats>) {
        self.node_stats = v;
    }

    // Mutable pointer to the field.
    pub fn mut_node_stats(&mut self) -> &mut ::protobuf::RepeatedField<NodeExecStats> {
        &mut self.node_stats
    }

    // Take field
    pub fn take_node_stats(&mut self) -> ::protobuf::RepeatedField<NodeExecStats> {
        ::std::mem::replace(&mut self.node_stats, ::protobuf::RepeatedField::new())
    }

    pub fn get_node_stats(&self) -> &[NodeExecStats] {
        &self.node_stats
    }

    fn get_node_stats_for_reflect(&self) -> &::protobuf::RepeatedField<NodeExecStats> {
        &self.node_stats
    }

    fn mut_node_stats_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<NodeExecStats> {
        &mut self.node_stats
    }
}

impl ::protobuf::Message for DeviceStepStats {
    fn is_initialized(&self) -> bool {
        for v in &self.node_stats {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.device)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.node_stats)?;
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
        for value in &self.node_stats {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.device.is_empty() {
            os.write_string(1, &self.device)?;
        }
        for v in &self.node_stats {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for DeviceStepStats {
    fn new() -> DeviceStepStats {
        DeviceStepStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeviceStepStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "device",
                    DeviceStepStats::get_device_for_reflect,
                    DeviceStepStats::mut_device_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NodeExecStats>>(
                    "node_stats",
                    DeviceStepStats::get_node_stats_for_reflect,
                    DeviceStepStats::mut_node_stats_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeviceStepStats>(
                    "DeviceStepStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeviceStepStats {
    fn clear(&mut self) {
        self.clear_device();
        self.clear_node_stats();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeviceStepStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeviceStepStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StepStats {
    // message fields
    pub dev_stats: ::protobuf::RepeatedField<DeviceStepStats>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StepStats {}

impl StepStats {
    pub fn new() -> StepStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StepStats {
        static mut instance: ::protobuf::lazy::Lazy<StepStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StepStats,
        };
        unsafe {
            instance.get(StepStats::new)
        }
    }

    // repeated .tensorflow.DeviceStepStats dev_stats = 1;

    pub fn clear_dev_stats(&mut self) {
        self.dev_stats.clear();
    }

    // Param is passed by value, moved
    pub fn set_dev_stats(&mut self, v: ::protobuf::RepeatedField<DeviceStepStats>) {
        self.dev_stats = v;
    }

    // Mutable pointer to the field.
    pub fn mut_dev_stats(&mut self) -> &mut ::protobuf::RepeatedField<DeviceStepStats> {
        &mut self.dev_stats
    }

    // Take field
    pub fn take_dev_stats(&mut self) -> ::protobuf::RepeatedField<DeviceStepStats> {
        ::std::mem::replace(&mut self.dev_stats, ::protobuf::RepeatedField::new())
    }

    pub fn get_dev_stats(&self) -> &[DeviceStepStats] {
        &self.dev_stats
    }

    fn get_dev_stats_for_reflect(&self) -> &::protobuf::RepeatedField<DeviceStepStats> {
        &self.dev_stats
    }

    fn mut_dev_stats_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DeviceStepStats> {
        &mut self.dev_stats
    }
}

impl ::protobuf::Message for StepStats {
    fn is_initialized(&self) -> bool {
        for v in &self.dev_stats {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.dev_stats)?;
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
        for value in &self.dev_stats {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.dev_stats {
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

impl ::protobuf::MessageStatic for StepStats {
    fn new() -> StepStats {
        StepStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<StepStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DeviceStepStats>>(
                    "dev_stats",
                    StepStats::get_dev_stats_for_reflect,
                    StepStats::mut_dev_stats_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StepStats>(
                    "StepStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StepStats {
    fn clear(&mut self) {
        self.clear_dev_stats();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StepStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StepStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n*tensorflow/core/framework/step_stats.proto\x12\ntensorflow\x1a6tensor\
    flow/core/framework/allocation_description.proto\x1a2tensorflow/core/fra\
    mework/tensor_description.proto\"\xd0\x01\n\x13AllocatorMemoryUsed\x12%\
    \n\x0eallocator_name\x18\x01\x20\x01(\tR\rallocatorName\x12\x1f\n\x0btot\
    al_bytes\x18\x02\x20\x01(\x03R\ntotalBytes\x12\x1d\n\npeak_bytes\x18\x03\
    \x20\x01(\x03R\tpeakBytes\x12\x1d\n\nlive_bytes\x18\x04\x20\x01(\x03R\tl\
    iveBytes\x123\n\x16allocator_bytes_in_use\x18\x05\x20\x01(\x03R\x13alloc\
    atorBytesInUse\"n\n\nNodeOutput\x12\x12\n\x04slot\x18\x01\x20\x01(\x05R\
    \x04slot\x12L\n\x12tensor_description\x18\x03\x20\x01(\x0b2\x1d.tensorfl\
    ow.TensorDescriptionR\x11tensorDescription\"\x8d\x03\n\x0bMemoryStats\
    \x121\n\x15host_temp_memory_size\x18\x01\x20\x01(\x03R\x12hostTempMemory\
    Size\x125\n\x17device_temp_memory_size\x18\x02\x20\x01(\x03R\x14deviceTe\
    mpMemorySize\x12=\n\x1bhost_persistent_memory_size\x18\x03\x20\x01(\x03R\
    \x18hostPersistentMemorySize\x12A\n\x1ddevice_persistent_memory_size\x18\
    \x04\x20\x01(\x03R\x1adevicePersistentMemorySize\x12F\n\x20host_persiste\
    nt_tensor_alloc_ids\x18\x05\x20\x03(\x03R\x1chostPersistentTensorAllocId\
    s\x12J\n\"device_persistent_tensor_alloc_ids\x18\x06\x20\x03(\x03R\x1ede\
    vicePersistentTensorAllocIds\"\xc1\x04\n\rNodeExecStats\x12\x1b\n\tnode_\
    name\x18\x01\x20\x01(\tR\x08nodeName\x12(\n\x10all_start_micros\x18\x02\
    \x20\x01(\x03R\x0eallStartMicros\x12-\n\x13op_start_rel_micros\x18\x03\
    \x20\x01(\x03R\x10opStartRelMicros\x12)\n\x11op_end_rel_micros\x18\x04\
    \x20\x01(\x03R\x0eopEndRelMicros\x12+\n\x12all_end_rel_micros\x18\x05\
    \x20\x01(\x03R\x0fallEndRelMicros\x127\n\x06memory\x18\x06\x20\x03(\x0b2\
    \x1f.tensorflow.AllocatorMemoryUsedR\x06memory\x12.\n\x06output\x18\x07\
    \x20\x03(\x0b2\x16.tensorflow.NodeOutputR\x06output\x12%\n\x0etimeline_l\
    abel\x18\x08\x20\x01(\tR\rtimelineLabel\x12)\n\x10scheduled_micros\x18\t\
    \x20\x01(\x03R\x0fscheduledMicros\x12\x1b\n\tthread_id\x18\n\x20\x01(\rR\
    \x08threadId\x12N\n\x11referenced_tensor\x18\x0b\x20\x03(\x0b2!.tensorfl\
    ow.AllocationDescriptionR\x10referencedTensor\x12:\n\x0cmemory_stats\x18\
    \x0c\x20\x01(\x0b2\x17.tensorflow.MemoryStatsR\x0bmemoryStats\"c\n\x0fDe\
    viceStepStats\x12\x16\n\x06device\x18\x01\x20\x01(\tR\x06device\x128\n\n\
    node_stats\x18\x02\x20\x03(\x0b2\x19.tensorflow.NodeExecStatsR\tnodeStat\
    s\"E\n\tStepStats\x128\n\tdev_stats\x18\x01\x20\x03(\x0b2\x1b.tensorflow\
    .DeviceStepStatsR\x08devStatsB0\n\x18org.tensorflow.frameworkB\x0fStepSt\
    atsProtosP\x01\xf8\x01\x01J\xfa\x18\n\x06\x12\x04\0\0G\x02\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\n\x08\n\x01\x08\
    \x12\x03\x03\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\n\x0c\n\
    \x05\x08\xe7\x07\0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\
    \x12\x03\x03\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x03\x07\
    \x17\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\x01\x08\
    \x12\x03\x04\00\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\00\n\x0c\n\x05\
    \x08\xe7\x07\x01\x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\
    \x12\x03\x04\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x04\
    \x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e/\n\x08\n\x01\
    \x08\x12\x03\x05\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\x0c\n\
    \x05\x08\xe7\x07\x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\
    \x02\0\x12\x03\x05\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\
    \x05\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\n\
    \x01\x08\x12\x03\x06\01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\x0c\
    \n\x05\x08\xe7\x07\x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\x03\
    \x02\0\x12\x03\x06\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\
    \x06\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\t\n\x02\
    \x03\0\x12\x03\x08\x07?\n\t\n\x02\x03\x01\x12\x03\t\x07;\n\x99\x01\n\x02\
    \x04\0\x12\x04\x0f\0\x1a\x012\x8c\x01\x20TODO(tucker):\x20The\x20next\
    \x204\x20message\x20defs\x20are\x20very\x20similar\x20to\n\x20the\x20*Lo\
    gEntry\x20messages\x20in\x20profile.proto.\x20\x20They\x20should\x20be\n\
    \x20unified\x20in\x20one\x20place.\n\n\n\n\x03\x04\0\x01\x12\x03\x0f\x08\
    \x1b\n\x0b\n\x04\x04\0\x02\0\x12\x03\x10\x02\x1c\n\r\n\x05\x04\0\x02\0\
    \x04\x12\x04\x10\x02\x0f\x1d\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x10\x02\
    \x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x10\t\x17\n\x0c\n\x05\x04\0\x02\
    \0\x03\x12\x03\x10\x1a\x1b\n9\n\x04\x04\0\x02\x01\x12\x03\x12\x02\x18\
    \x1a,\x20These\x20are\x20per-node\x20allocator\x20memory\x20stats.\n\n\r\
    \n\x05\x04\0\x02\x01\x04\x12\x04\x12\x02\x10\x1c\n\x0c\n\x05\x04\0\x02\
    \x01\x05\x12\x03\x12\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x12\
    \x08\x13\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x12\x16\x17\n\x0b\n\x04\
    \x04\0\x02\x02\x12\x03\x13\x02\x17\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\
    \x13\x02\x12\x18\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x13\x02\x07\n\x0c\
    \n\x05\x04\0\x02\x02\x01\x12\x03\x13\x08\x12\n\x0c\n\x05\x04\0\x02\x02\
    \x03\x12\x03\x13\x15\x16\n2\n\x04\x04\0\x02\x03\x12\x03\x15\x02\x17\x1a%\
    \x20The\x20bytes\x20that\x20are\x20not\x20deallocated.\n\n\r\n\x05\x04\0\
    \x02\x03\x04\x12\x04\x15\x02\x13\x17\n\x0c\n\x05\x04\0\x02\x03\x05\x12\
    \x03\x15\x02\x07\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x15\x08\x12\n\x0c\
    \n\x05\x04\0\x02\x03\x03\x12\x03\x15\x15\x16\n\x89\x01\n\x04\x04\0\x02\
    \x04\x12\x03\x19\x02#\x1a|\x20These\x20are\x20snapshots\x20of\x20the\x20\
    overall\x20allocator\x20memory\x20stats.\n\x20The\x20number\x20of\x20liv\
    e\x20bytes\x20currently\x20allocated\x20by\x20the\x20allocator.\n\n\r\n\
    \x05\x04\0\x02\x04\x04\x12\x04\x19\x02\x15\x17\n\x0c\n\x05\x04\0\x02\x04\
    \x05\x12\x03\x19\x02\x07\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x19\x08\
    \x1e\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x19!\"\nK\n\x02\x04\x01\x12\
    \x04\x1d\0\x20\x01\x1a?\x20Output\x20sizes\x20recorded\x20for\x20a\x20si\
    ngle\x20execution\x20of\x20a\x20graph\x20node.\n\n\n\n\x03\x04\x01\x01\
    \x12\x03\x1d\x08\x12\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x1e\x02\x11\n\r\n\
    \x05\x04\x01\x02\0\x04\x12\x04\x1e\x02\x1d\x14\n\x0c\n\x05\x04\x01\x02\0\
    \x05\x12\x03\x1e\x02\x07\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x1e\x08\
    \x0c\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x1e\x0f\x10\n\x0b\n\x04\x04\
    \x01\x02\x01\x12\x03\x1f\x02+\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x1f\
    \x02\x1e\x11\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03\x1f\x02\x13\n\x0c\n\
    \x05\x04\x01\x02\x01\x01\x12\x03\x1f\x14&\n\x0c\n\x05\x04\x01\x02\x01\
    \x03\x12\x03\x1f)*\n\"\n\x02\x04\x02\x12\x04#\0*\x01\x1a\x16\x20For\x20m\
    emory\x20tracking.\n\n\n\n\x03\x04\x02\x01\x12\x03#\x08\x13\n\x0b\n\x04\
    \x04\x02\x02\0\x12\x03$\x02\"\n\r\n\x05\x04\x02\x02\0\x04\x12\x04$\x02#\
    \x15\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03$\x02\x07\n\x0c\n\x05\x04\x02\
    \x02\0\x01\x12\x03$\x08\x1d\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03$\x20!\
    \n\x0b\n\x04\x04\x02\x02\x01\x12\x03%\x02$\n\r\n\x05\x04\x02\x02\x01\x04\
    \x12\x04%\x02$\"\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03%\x02\x07\n\x0c\
    \n\x05\x04\x02\x02\x01\x01\x12\x03%\x08\x1f\n\x0c\n\x05\x04\x02\x02\x01\
    \x03\x12\x03%\"#\n\x0b\n\x04\x04\x02\x02\x02\x12\x03&\x02(\n\r\n\x05\x04\
    \x02\x02\x02\x04\x12\x04&\x02%$\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03&\
    \x02\x07\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03&\x08#\n\x0c\n\x05\x04\
    \x02\x02\x02\x03\x12\x03&&'\n\x0b\n\x04\x04\x02\x02\x03\x12\x03'\x02*\n\
    \r\n\x05\x04\x02\x02\x03\x04\x12\x04'\x02&(\n\x0c\n\x05\x04\x02\x02\x03\
    \x05\x12\x03'\x02\x07\n\x0c\n\x05\x04\x02\x02\x03\x01\x12\x03'\x08%\n\
    \x0c\n\x05\x04\x02\x02\x03\x03\x12\x03'()\n\x0b\n\x04\x04\x02\x02\x04\
    \x12\x03(\x026\n\x0c\n\x05\x04\x02\x02\x04\x04\x12\x03(\x02\n\n\x0c\n\
    \x05\x04\x02\x02\x04\x05\x12\x03(\x0b\x10\n\x0c\n\x05\x04\x02\x02\x04\
    \x01\x12\x03(\x111\n\x0c\n\x05\x04\x02\x02\x04\x03\x12\x03(45\n\x0b\n\
    \x04\x04\x02\x02\x05\x12\x03)\x028\n\x0c\n\x05\x04\x02\x02\x05\x04\x12\
    \x03)\x02\n\n\x0c\n\x05\x04\x02\x02\x05\x05\x12\x03)\x0b\x10\n\x0c\n\x05\
    \x04\x02\x02\x05\x01\x12\x03)\x113\n\x0c\n\x05\x04\x02\x02\x05\x03\x12\
    \x03)67\nN\n\x02\x04\x03\x12\x04-\0>\x01\x1aB\x20Time/size\x20stats\x20r\
    ecorded\x20for\x20a\x20single\x20execution\x20of\x20a\x20graph\x20node.\
    \n\n\n\n\x03\x04\x03\x01\x12\x03-\x08\x15\n\xd8\x01\n\x04\x04\x03\x02\0\
    \x12\x032\x02\x17\x1a\xca\x01\x20TODO(tucker):\x20Use\x20some\x20more\
    \x20compact\x20form\x20of\x20node\x20identity\x20than\n\x20the\x20full\
    \x20string\x20name.\x20\x20Either\x20all\x20processes\x20should\x20agree\
    \x20on\x20a\n\x20global\x20id\x20(cost_id?)\x20for\x20each\x20node,\x20o\
    r\x20we\x20should\x20use\x20a\x20hash\x20of\n\x20the\x20name.\n\n\r\n\
    \x05\x04\x03\x02\0\x04\x12\x042\x02-\x17\n\x0c\n\x05\x04\x03\x02\0\x05\
    \x12\x032\x02\x08\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x032\t\x12\n\x0c\n\
    \x05\x04\x03\x02\0\x03\x12\x032\x15\x16\n\x0b\n\x04\x04\x03\x02\x01\x12\
    \x033\x02\x1d\n\r\n\x05\x04\x03\x02\x01\x04\x12\x043\x022\x17\n\x0c\n\
    \x05\x04\x03\x02\x01\x05\x12\x033\x02\x07\n\x0c\n\x05\x04\x03\x02\x01\
    \x01\x12\x033\x08\x18\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x033\x1b\x1c\n\
    \x0b\n\x04\x04\x03\x02\x02\x12\x034\x02\x20\n\r\n\x05\x04\x03\x02\x02\
    \x04\x12\x044\x023\x1d\n\x0c\n\x05\x04\x03\x02\x02\x05\x12\x034\x02\x07\
    \n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x034\x08\x1b\n\x0c\n\x05\x04\x03\
    \x02\x02\x03\x12\x034\x1e\x1f\n\x0b\n\x04\x04\x03\x02\x03\x12\x035\x02\
    \x1e\n\r\n\x05\x04\x03\x02\x03\x04\x12\x045\x024\x20\n\x0c\n\x05\x04\x03\
    \x02\x03\x05\x12\x035\x02\x07\n\x0c\n\x05\x04\x03\x02\x03\x01\x12\x035\
    \x08\x19\n\x0c\n\x05\x04\x03\x02\x03\x03\x12\x035\x1c\x1d\n\x0b\n\x04\
    \x04\x03\x02\x04\x12\x036\x02\x1f\n\r\n\x05\x04\x03\x02\x04\x04\x12\x046\
    \x025\x1e\n\x0c\n\x05\x04\x03\x02\x04\x05\x12\x036\x02\x07\n\x0c\n\x05\
    \x04\x03\x02\x04\x01\x12\x036\x08\x1a\n\x0c\n\x05\x04\x03\x02\x04\x03\
    \x12\x036\x1d\x1e\n\x0b\n\x04\x04\x03\x02\x05\x12\x037\x02*\n\x0c\n\x05\
    \x04\x03\x02\x05\x04\x12\x037\x02\n\n\x0c\n\x05\x04\x03\x02\x05\x06\x12\
    \x037\x0b\x1e\n\x0c\n\x05\x04\x03\x02\x05\x01\x12\x037\x1f%\n\x0c\n\x05\
    \x04\x03\x02\x05\x03\x12\x037()\n\x0b\n\x04\x04\x03\x02\x06\x12\x038\x02\
    !\n\x0c\n\x05\x04\x03\x02\x06\x04\x12\x038\x02\n\n\x0c\n\x05\x04\x03\x02\
    \x06\x06\x12\x038\x0b\x15\n\x0c\n\x05\x04\x03\x02\x06\x01\x12\x038\x16\
    \x1c\n\x0c\n\x05\x04\x03\x02\x06\x03\x12\x038\x1f\x20\n\x0b\n\x04\x04\
    \x03\x02\x07\x12\x039\x02\x1c\n\r\n\x05\x04\x03\x02\x07\x04\x12\x049\x02\
    8!\n\x0c\n\x05\x04\x03\x02\x07\x05\x12\x039\x02\x08\n\x0c\n\x05\x04\x03\
    \x02\x07\x01\x12\x039\t\x17\n\x0c\n\x05\x04\x03\x02\x07\x03\x12\x039\x1a\
    \x1b\n\x0b\n\x04\x04\x03\x02\x08\x12\x03:\x02\x1d\n\r\n\x05\x04\x03\x02\
    \x08\x04\x12\x04:\x029\x1c\n\x0c\n\x05\x04\x03\x02\x08\x05\x12\x03:\x02\
    \x07\n\x0c\n\x05\x04\x03\x02\x08\x01\x12\x03:\x08\x18\n\x0c\n\x05\x04\
    \x03\x02\x08\x03\x12\x03:\x1b\x1c\n\x0b\n\x04\x04\x03\x02\t\x12\x03;\x02\
    \x18\n\r\n\x05\x04\x03\x02\t\x04\x12\x04;\x02:\x1d\n\x0c\n\x05\x04\x03\
    \x02\t\x05\x12\x03;\x02\x08\n\x0c\n\x05\x04\x03\x02\t\x01\x12\x03;\t\x12\
    \n\x0c\n\x05\x04\x03\x02\t\x03\x12\x03;\x15\x17\n\x0b\n\x04\x04\x03\x02\
    \n\x12\x03<\x028\n\x0c\n\x05\x04\x03\x02\n\x04\x12\x03<\x02\n\n\x0c\n\
    \x05\x04\x03\x02\n\x06\x12\x03<\x0b\x20\n\x0c\n\x05\x04\x03\x02\n\x01\
    \x12\x03<!2\n\x0c\n\x05\x04\x03\x02\n\x03\x12\x03<57\n\x0b\n\x04\x04\x03\
    \x02\x0b\x12\x03=\x02\x20\n\r\n\x05\x04\x03\x02\x0b\x04\x12\x04=\x02<8\n\
    \x0c\n\x05\x04\x03\x02\x0b\x06\x12\x03=\x02\r\n\x0c\n\x05\x04\x03\x02\
    \x0b\x01\x12\x03=\x0e\x1a\n\x0c\n\x05\x04\x03\x02\x0b\x03\x12\x03=\x1d\
    \x1f\n\n\n\x02\x04\x04\x12\x04@\0C\x01\n\n\n\x03\x04\x04\x01\x12\x03@\
    \x08\x17\n\x0b\n\x04\x04\x04\x02\0\x12\x03A\x02\x14\n\r\n\x05\x04\x04\
    \x02\0\x04\x12\x04A\x02@\x19\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03A\x02\
    \x08\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03A\t\x0f\n\x0c\n\x05\x04\x04\
    \x02\0\x03\x12\x03A\x12\x13\n\x0b\n\x04\x04\x04\x02\x01\x12\x03B\x02(\n\
    \x0c\n\x05\x04\x04\x02\x01\x04\x12\x03B\x02\n\n\x0c\n\x05\x04\x04\x02\
    \x01\x06\x12\x03B\x0b\x18\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03B\x19#\
    \n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03B&'\n\n\n\x02\x04\x05\x12\x04E\0\
    G\x01\n\n\n\x03\x04\x05\x01\x12\x03E\x08\x11\n\x0b\n\x04\x04\x05\x02\0\
    \x12\x03F\x02)\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03F\x02\n\n\x0c\n\x05\
    \x04\x05\x02\0\x06\x12\x03F\x0b\x1a\n\x0c\n\x05\x04\x05\x02\0\x01\x12\
    \x03F\x1b$\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03F'(b\x06proto3\
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
