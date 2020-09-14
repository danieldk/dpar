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
pub struct AllocationDescription {
    // message fields
    pub requested_bytes: i64,
    pub allocated_bytes: i64,
    pub allocator_name: ::std::string::String,
    pub allocation_id: i64,
    pub has_single_reference: bool,
    pub ptr: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AllocationDescription {}

impl AllocationDescription {
    pub fn new() -> AllocationDescription {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AllocationDescription {
        static mut instance: ::protobuf::lazy::Lazy<AllocationDescription> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AllocationDescription,
        };
        unsafe {
            instance.get(AllocationDescription::new)
        }
    }

    // int64 requested_bytes = 1;

    pub fn clear_requested_bytes(&mut self) {
        self.requested_bytes = 0;
    }

    // Param is passed by value, moved
    pub fn set_requested_bytes(&mut self, v: i64) {
        self.requested_bytes = v;
    }

    pub fn get_requested_bytes(&self) -> i64 {
        self.requested_bytes
    }

    fn get_requested_bytes_for_reflect(&self) -> &i64 {
        &self.requested_bytes
    }

    fn mut_requested_bytes_for_reflect(&mut self) -> &mut i64 {
        &mut self.requested_bytes
    }

    // int64 allocated_bytes = 2;

    pub fn clear_allocated_bytes(&mut self) {
        self.allocated_bytes = 0;
    }

    // Param is passed by value, moved
    pub fn set_allocated_bytes(&mut self, v: i64) {
        self.allocated_bytes = v;
    }

    pub fn get_allocated_bytes(&self) -> i64 {
        self.allocated_bytes
    }

    fn get_allocated_bytes_for_reflect(&self) -> &i64 {
        &self.allocated_bytes
    }

    fn mut_allocated_bytes_for_reflect(&mut self) -> &mut i64 {
        &mut self.allocated_bytes
    }

    // string allocator_name = 3;

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

    // int64 allocation_id = 4;

    pub fn clear_allocation_id(&mut self) {
        self.allocation_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_allocation_id(&mut self, v: i64) {
        self.allocation_id = v;
    }

    pub fn get_allocation_id(&self) -> i64 {
        self.allocation_id
    }

    fn get_allocation_id_for_reflect(&self) -> &i64 {
        &self.allocation_id
    }

    fn mut_allocation_id_for_reflect(&mut self) -> &mut i64 {
        &mut self.allocation_id
    }

    // bool has_single_reference = 5;

    pub fn clear_has_single_reference(&mut self) {
        self.has_single_reference = false;
    }

    // Param is passed by value, moved
    pub fn set_has_single_reference(&mut self, v: bool) {
        self.has_single_reference = v;
    }

    pub fn get_has_single_reference(&self) -> bool {
        self.has_single_reference
    }

    fn get_has_single_reference_for_reflect(&self) -> &bool {
        &self.has_single_reference
    }

    fn mut_has_single_reference_for_reflect(&mut self) -> &mut bool {
        &mut self.has_single_reference
    }

    // uint64 ptr = 6;

    pub fn clear_ptr(&mut self) {
        self.ptr = 0;
    }

    // Param is passed by value, moved
    pub fn set_ptr(&mut self, v: u64) {
        self.ptr = v;
    }

    pub fn get_ptr(&self) -> u64 {
        self.ptr
    }

    fn get_ptr_for_reflect(&self) -> &u64 {
        &self.ptr
    }

    fn mut_ptr_for_reflect(&mut self) -> &mut u64 {
        &mut self.ptr
    }
}

impl ::protobuf::Message for AllocationDescription {
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
                    self.requested_bytes = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.allocated_bytes = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.allocator_name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.allocation_id = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.has_single_reference = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.ptr = tmp;
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
        if self.requested_bytes != 0 {
            my_size += ::protobuf::rt::value_size(1, self.requested_bytes, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.allocated_bytes != 0 {
            my_size += ::protobuf::rt::value_size(2, self.allocated_bytes, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.allocator_name.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.allocator_name);
        }
        if self.allocation_id != 0 {
            my_size += ::protobuf::rt::value_size(4, self.allocation_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.has_single_reference != false {
            my_size += 2;
        }
        if self.ptr != 0 {
            my_size += ::protobuf::rt::value_size(6, self.ptr, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.requested_bytes != 0 {
            os.write_int64(1, self.requested_bytes)?;
        }
        if self.allocated_bytes != 0 {
            os.write_int64(2, self.allocated_bytes)?;
        }
        if !self.allocator_name.is_empty() {
            os.write_string(3, &self.allocator_name)?;
        }
        if self.allocation_id != 0 {
            os.write_int64(4, self.allocation_id)?;
        }
        if self.has_single_reference != false {
            os.write_bool(5, self.has_single_reference)?;
        }
        if self.ptr != 0 {
            os.write_uint64(6, self.ptr)?;
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

impl ::protobuf::MessageStatic for AllocationDescription {
    fn new() -> AllocationDescription {
        AllocationDescription::new()
    }

    fn descriptor_static(_: ::std::option::Option<AllocationDescription>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "requested_bytes",
                    AllocationDescription::get_requested_bytes_for_reflect,
                    AllocationDescription::mut_requested_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "allocated_bytes",
                    AllocationDescription::get_allocated_bytes_for_reflect,
                    AllocationDescription::mut_allocated_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "allocator_name",
                    AllocationDescription::get_allocator_name_for_reflect,
                    AllocationDescription::mut_allocator_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "allocation_id",
                    AllocationDescription::get_allocation_id_for_reflect,
                    AllocationDescription::mut_allocation_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_single_reference",
                    AllocationDescription::get_has_single_reference_for_reflect,
                    AllocationDescription::mut_has_single_reference_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ptr",
                    AllocationDescription::get_ptr_for_reflect,
                    AllocationDescription::mut_ptr_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AllocationDescription>(
                    "AllocationDescription",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AllocationDescription {
    fn clear(&mut self) {
        self.clear_requested_bytes();
        self.clear_allocated_bytes();
        self.clear_allocator_name();
        self.clear_allocation_id();
        self.clear_has_single_reference();
        self.clear_ptr();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AllocationDescription {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AllocationDescription {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n6tensorflow/core/framework/allocation_description.proto\x12\ntensorflo\
    w\"\xf9\x01\n\x15AllocationDescription\x12'\n\x0frequested_bytes\x18\x01\
    \x20\x01(\x03R\x0erequestedBytes\x12'\n\x0fallocated_bytes\x18\x02\x20\
    \x01(\x03R\x0eallocatedBytes\x12%\n\x0eallocator_name\x18\x03\x20\x01(\t\
    R\rallocatorName\x12#\n\rallocation_id\x18\x04\x20\x01(\x03R\x0callocati\
    onId\x120\n\x14has_single_reference\x18\x05\x20\x01(\x08R\x12hasSingleRe\
    ference\x12\x10\n\x03ptr\x18\x06\x20\x01(\x04R\x03ptrB<\n\x18org.tensorf\
    low.frameworkB\x1bAllocationDescriptionProtosP\x01\xf8\x01\x01J\x91\x08\
    \n\x06\x12\x04\0\0\x1a\x02\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\
    \x02\x12\x03\x02\x08\x12\n\x08\n\x01\x08\x12\x03\x03\0\x1f\n\x0b\n\x04\
    \x08\xe7\x07\0\x12\x03\x03\0\x1f\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\
    \x03\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x03\x07\x17\n\x0e\n\
    \x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x03\x07\x17\n\x0c\n\x05\x08\xe7\x07\
    \0\x03\x12\x03\x03\x1a\x1e\n\x08\n\x01\x08\x12\x03\x04\0<\n\x0b\n\x04\
    \x08\xe7\x07\x01\x12\x03\x04\0<\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\
    \x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x04\x07\x1b\n\x0e\n\
    \x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x04\x07\x1b\n\x0c\n\x05\x08\xe7\
    \x07\x01\x07\x12\x03\x04\x1e;\n\x08\n\x01\x08\x12\x03\x05\0\"\n\x0b\n\
    \x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\
    \x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x05\x07\x1a\n\
    \x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x05\x07\x1a\n\x0c\n\x05\x08\
    \xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\n\x01\x08\x12\x03\x06\01\n\x0b\n\
    \x04\x08\xe7\x07\x03\x12\x03\x06\01\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\
    \x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x06\x07\x13\n\
    \x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x06\x07\x13\n\x0c\n\x05\x08\
    \xe7\x07\x03\x07\x12\x03\x06\x160\n\n\n\x02\x04\0\x12\x04\x08\0\x1a\x01\
    \n\n\n\x03\x04\0\x01\x12\x03\x08\x08\x1d\n.\n\x04\x04\0\x02\0\x12\x03\n\
    \x02\x1c\x1a!\x20Total\x20number\x20of\x20bytes\x20requested\n\n\r\n\x05\
    \x04\0\x02\0\x04\x12\x04\n\x02\x08\x1f\n\x0c\n\x05\x04\0\x02\0\x05\x12\
    \x03\n\x02\x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\n\x08\x17\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\n\x1a\x1b\n7\n\x04\x04\0\x02\x01\x12\x03\r\x02\
    \x1c\x1a*\x20Total\x20number\x20of\x20bytes\x20allocated\x20if\x20known\
    \n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\r\x02\n\x1c\n\x0c\n\x05\x04\0\x02\
    \x01\x05\x12\x03\r\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\r\x08\
    \x17\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\r\x1a\x1b\n)\n\x04\x04\0\x02\
    \x02\x12\x03\x10\x02\x1c\x1a\x1c\x20Name\x20of\x20the\x20allocator\x20us\
    ed\n\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x10\x02\r\x1c\n\x0c\n\x05\x04\0\
    \x02\x02\x05\x12\x03\x10\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\
    \x10\t\x17\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x10\x1a\x1b\n:\n\x04\
    \x04\0\x02\x03\x12\x03\x13\x02\x1a\x1a-\x20Identifier\x20of\x20the\x20al\
    located\x20buffer\x20if\x20known\n\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\
    \x13\x02\x10\x1c\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x13\x02\x07\n\x0c\
    \n\x05\x04\0\x02\x03\x01\x12\x03\x13\x08\x15\n\x0c\n\x05\x04\0\x02\x03\
    \x03\x12\x03\x13\x18\x19\nB\n\x04\x04\0\x02\x04\x12\x03\x16\x02\x20\x1a5\
    \x20Set\x20if\x20this\x20tensor\x20only\x20has\x20one\x20remaining\x20re\
    ference\n\n\r\n\x05\x04\0\x02\x04\x04\x12\x04\x16\x02\x13\x1a\n\x0c\n\
    \x05\x04\0\x02\x04\x05\x12\x03\x16\x02\x06\n\x0c\n\x05\x04\0\x02\x04\x01\
    \x12\x03\x16\x07\x1b\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x16\x1e\x1f\n\
    )\n\x04\x04\0\x02\x05\x12\x03\x19\x02\x11\x1a\x1c\x20Address\x20of\x20th\
    e\x20allocation.\n\n\r\n\x05\x04\0\x02\x05\x04\x12\x04\x19\x02\x16\x20\n\
    \x0c\n\x05\x04\0\x02\x05\x05\x12\x03\x19\x02\x08\n\x0c\n\x05\x04\0\x02\
    \x05\x01\x12\x03\x19\t\x0c\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x19\x0f\
    \x10b\x06proto3\
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
