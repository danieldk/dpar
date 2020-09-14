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
pub struct GPUOptions {
    // message fields
    pub per_process_gpu_memory_fraction: f64,
    pub allocator_type: ::std::string::String,
    pub deferred_deletion_bytes: i64,
    pub allow_growth: bool,
    pub visible_device_list: ::std::string::String,
    pub polling_active_delay_usecs: i32,
    pub polling_inactive_delay_msecs: i32,
    pub force_gpu_compatible: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GPUOptions {}

impl GPUOptions {
    pub fn new() -> GPUOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GPUOptions {
        static mut instance: ::protobuf::lazy::Lazy<GPUOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GPUOptions,
        };
        unsafe {
            instance.get(GPUOptions::new)
        }
    }

    // double per_process_gpu_memory_fraction = 1;

    pub fn clear_per_process_gpu_memory_fraction(&mut self) {
        self.per_process_gpu_memory_fraction = 0.;
    }

    // Param is passed by value, moved
    pub fn set_per_process_gpu_memory_fraction(&mut self, v: f64) {
        self.per_process_gpu_memory_fraction = v;
    }

    pub fn get_per_process_gpu_memory_fraction(&self) -> f64 {
        self.per_process_gpu_memory_fraction
    }

    fn get_per_process_gpu_memory_fraction_for_reflect(&self) -> &f64 {
        &self.per_process_gpu_memory_fraction
    }

    fn mut_per_process_gpu_memory_fraction_for_reflect(&mut self) -> &mut f64 {
        &mut self.per_process_gpu_memory_fraction
    }

    // string allocator_type = 2;

    pub fn clear_allocator_type(&mut self) {
        self.allocator_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_allocator_type(&mut self, v: ::std::string::String) {
        self.allocator_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_allocator_type(&mut self) -> &mut ::std::string::String {
        &mut self.allocator_type
    }

    // Take field
    pub fn take_allocator_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.allocator_type, ::std::string::String::new())
    }

    pub fn get_allocator_type(&self) -> &str {
        &self.allocator_type
    }

    fn get_allocator_type_for_reflect(&self) -> &::std::string::String {
        &self.allocator_type
    }

    fn mut_allocator_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.allocator_type
    }

    // int64 deferred_deletion_bytes = 3;

    pub fn clear_deferred_deletion_bytes(&mut self) {
        self.deferred_deletion_bytes = 0;
    }

    // Param is passed by value, moved
    pub fn set_deferred_deletion_bytes(&mut self, v: i64) {
        self.deferred_deletion_bytes = v;
    }

    pub fn get_deferred_deletion_bytes(&self) -> i64 {
        self.deferred_deletion_bytes
    }

    fn get_deferred_deletion_bytes_for_reflect(&self) -> &i64 {
        &self.deferred_deletion_bytes
    }

    fn mut_deferred_deletion_bytes_for_reflect(&mut self) -> &mut i64 {
        &mut self.deferred_deletion_bytes
    }

    // bool allow_growth = 4;

    pub fn clear_allow_growth(&mut self) {
        self.allow_growth = false;
    }

    // Param is passed by value, moved
    pub fn set_allow_growth(&mut self, v: bool) {
        self.allow_growth = v;
    }

    pub fn get_allow_growth(&self) -> bool {
        self.allow_growth
    }

    fn get_allow_growth_for_reflect(&self) -> &bool {
        &self.allow_growth
    }

    fn mut_allow_growth_for_reflect(&mut self) -> &mut bool {
        &mut self.allow_growth
    }

    // string visible_device_list = 5;

    pub fn clear_visible_device_list(&mut self) {
        self.visible_device_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_visible_device_list(&mut self, v: ::std::string::String) {
        self.visible_device_list = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_visible_device_list(&mut self) -> &mut ::std::string::String {
        &mut self.visible_device_list
    }

    // Take field
    pub fn take_visible_device_list(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.visible_device_list, ::std::string::String::new())
    }

    pub fn get_visible_device_list(&self) -> &str {
        &self.visible_device_list
    }

    fn get_visible_device_list_for_reflect(&self) -> &::std::string::String {
        &self.visible_device_list
    }

    fn mut_visible_device_list_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.visible_device_list
    }

    // int32 polling_active_delay_usecs = 6;

    pub fn clear_polling_active_delay_usecs(&mut self) {
        self.polling_active_delay_usecs = 0;
    }

    // Param is passed by value, moved
    pub fn set_polling_active_delay_usecs(&mut self, v: i32) {
        self.polling_active_delay_usecs = v;
    }

    pub fn get_polling_active_delay_usecs(&self) -> i32 {
        self.polling_active_delay_usecs
    }

    fn get_polling_active_delay_usecs_for_reflect(&self) -> &i32 {
        &self.polling_active_delay_usecs
    }

    fn mut_polling_active_delay_usecs_for_reflect(&mut self) -> &mut i32 {
        &mut self.polling_active_delay_usecs
    }

    // int32 polling_inactive_delay_msecs = 7;

    pub fn clear_polling_inactive_delay_msecs(&mut self) {
        self.polling_inactive_delay_msecs = 0;
    }

    // Param is passed by value, moved
    pub fn set_polling_inactive_delay_msecs(&mut self, v: i32) {
        self.polling_inactive_delay_msecs = v;
    }

    pub fn get_polling_inactive_delay_msecs(&self) -> i32 {
        self.polling_inactive_delay_msecs
    }

    fn get_polling_inactive_delay_msecs_for_reflect(&self) -> &i32 {
        &self.polling_inactive_delay_msecs
    }

    fn mut_polling_inactive_delay_msecs_for_reflect(&mut self) -> &mut i32 {
        &mut self.polling_inactive_delay_msecs
    }

    // bool force_gpu_compatible = 8;

    pub fn clear_force_gpu_compatible(&mut self) {
        self.force_gpu_compatible = false;
    }

    // Param is passed by value, moved
    pub fn set_force_gpu_compatible(&mut self, v: bool) {
        self.force_gpu_compatible = v;
    }

    pub fn get_force_gpu_compatible(&self) -> bool {
        self.force_gpu_compatible
    }

    fn get_force_gpu_compatible_for_reflect(&self) -> &bool {
        &self.force_gpu_compatible
    }

    fn mut_force_gpu_compatible_for_reflect(&mut self) -> &mut bool {
        &mut self.force_gpu_compatible
    }
}

impl ::protobuf::Message for GPUOptions {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.per_process_gpu_memory_fraction = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.allocator_type)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.deferred_deletion_bytes = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.allow_growth = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.visible_device_list)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.polling_active_delay_usecs = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.polling_inactive_delay_msecs = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.force_gpu_compatible = tmp;
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
        if self.per_process_gpu_memory_fraction != 0. {
            my_size += 9;
        }
        if !self.allocator_type.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.allocator_type);
        }
        if self.deferred_deletion_bytes != 0 {
            my_size += ::protobuf::rt::value_size(3, self.deferred_deletion_bytes, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.allow_growth != false {
            my_size += 2;
        }
        if !self.visible_device_list.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.visible_device_list);
        }
        if self.polling_active_delay_usecs != 0 {
            my_size += ::protobuf::rt::value_size(6, self.polling_active_delay_usecs, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.polling_inactive_delay_msecs != 0 {
            my_size += ::protobuf::rt::value_size(7, self.polling_inactive_delay_msecs, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.force_gpu_compatible != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.per_process_gpu_memory_fraction != 0. {
            os.write_double(1, self.per_process_gpu_memory_fraction)?;
        }
        if !self.allocator_type.is_empty() {
            os.write_string(2, &self.allocator_type)?;
        }
        if self.deferred_deletion_bytes != 0 {
            os.write_int64(3, self.deferred_deletion_bytes)?;
        }
        if self.allow_growth != false {
            os.write_bool(4, self.allow_growth)?;
        }
        if !self.visible_device_list.is_empty() {
            os.write_string(5, &self.visible_device_list)?;
        }
        if self.polling_active_delay_usecs != 0 {
            os.write_int32(6, self.polling_active_delay_usecs)?;
        }
        if self.polling_inactive_delay_msecs != 0 {
            os.write_int32(7, self.polling_inactive_delay_msecs)?;
        }
        if self.force_gpu_compatible != false {
            os.write_bool(8, self.force_gpu_compatible)?;
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

impl ::protobuf::MessageStatic for GPUOptions {
    fn new() -> GPUOptions {
        GPUOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<GPUOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "per_process_gpu_memory_fraction",
                    GPUOptions::get_per_process_gpu_memory_fraction_for_reflect,
                    GPUOptions::mut_per_process_gpu_memory_fraction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "allocator_type",
                    GPUOptions::get_allocator_type_for_reflect,
                    GPUOptions::mut_allocator_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "deferred_deletion_bytes",
                    GPUOptions::get_deferred_deletion_bytes_for_reflect,
                    GPUOptions::mut_deferred_deletion_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allow_growth",
                    GPUOptions::get_allow_growth_for_reflect,
                    GPUOptions::mut_allow_growth_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "visible_device_list",
                    GPUOptions::get_visible_device_list_for_reflect,
                    GPUOptions::mut_visible_device_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "polling_active_delay_usecs",
                    GPUOptions::get_polling_active_delay_usecs_for_reflect,
                    GPUOptions::mut_polling_active_delay_usecs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "polling_inactive_delay_msecs",
                    GPUOptions::get_polling_inactive_delay_msecs_for_reflect,
                    GPUOptions::mut_polling_inactive_delay_msecs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "force_gpu_compatible",
                    GPUOptions::get_force_gpu_compatible_for_reflect,
                    GPUOptions::mut_force_gpu_compatible_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GPUOptions>(
                    "GPUOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GPUOptions {
    fn clear(&mut self) {
        self.clear_per_process_gpu_memory_fraction();
        self.clear_allocator_type();
        self.clear_deferred_deletion_bytes();
        self.clear_allow_growth();
        self.clear_visible_device_list();
        self.clear_polling_active_delay_usecs();
        self.clear_polling_inactive_delay_msecs();
        self.clear_force_gpu_compatible();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GPUOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GPUOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OptimizerOptions {
    // message fields
    pub do_common_subexpression_elimination: bool,
    pub do_constant_folding: bool,
    pub do_function_inlining: bool,
    pub opt_level: OptimizerOptions_Level,
    pub global_jit_level: OptimizerOptions_GlobalJitLevel,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OptimizerOptions {}

impl OptimizerOptions {
    pub fn new() -> OptimizerOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OptimizerOptions {
        static mut instance: ::protobuf::lazy::Lazy<OptimizerOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OptimizerOptions,
        };
        unsafe {
            instance.get(OptimizerOptions::new)
        }
    }

    // bool do_common_subexpression_elimination = 1;

    pub fn clear_do_common_subexpression_elimination(&mut self) {
        self.do_common_subexpression_elimination = false;
    }

    // Param is passed by value, moved
    pub fn set_do_common_subexpression_elimination(&mut self, v: bool) {
        self.do_common_subexpression_elimination = v;
    }

    pub fn get_do_common_subexpression_elimination(&self) -> bool {
        self.do_common_subexpression_elimination
    }

    fn get_do_common_subexpression_elimination_for_reflect(&self) -> &bool {
        &self.do_common_subexpression_elimination
    }

    fn mut_do_common_subexpression_elimination_for_reflect(&mut self) -> &mut bool {
        &mut self.do_common_subexpression_elimination
    }

    // bool do_constant_folding = 2;

    pub fn clear_do_constant_folding(&mut self) {
        self.do_constant_folding = false;
    }

    // Param is passed by value, moved
    pub fn set_do_constant_folding(&mut self, v: bool) {
        self.do_constant_folding = v;
    }

    pub fn get_do_constant_folding(&self) -> bool {
        self.do_constant_folding
    }

    fn get_do_constant_folding_for_reflect(&self) -> &bool {
        &self.do_constant_folding
    }

    fn mut_do_constant_folding_for_reflect(&mut self) -> &mut bool {
        &mut self.do_constant_folding
    }

    // bool do_function_inlining = 4;

    pub fn clear_do_function_inlining(&mut self) {
        self.do_function_inlining = false;
    }

    // Param is passed by value, moved
    pub fn set_do_function_inlining(&mut self, v: bool) {
        self.do_function_inlining = v;
    }

    pub fn get_do_function_inlining(&self) -> bool {
        self.do_function_inlining
    }

    fn get_do_function_inlining_for_reflect(&self) -> &bool {
        &self.do_function_inlining
    }

    fn mut_do_function_inlining_for_reflect(&mut self) -> &mut bool {
        &mut self.do_function_inlining
    }

    // .tensorflow.OptimizerOptions.Level opt_level = 3;

    pub fn clear_opt_level(&mut self) {
        self.opt_level = OptimizerOptions_Level::L1;
    }

    // Param is passed by value, moved
    pub fn set_opt_level(&mut self, v: OptimizerOptions_Level) {
        self.opt_level = v;
    }

    pub fn get_opt_level(&self) -> OptimizerOptions_Level {
        self.opt_level
    }

    fn get_opt_level_for_reflect(&self) -> &OptimizerOptions_Level {
        &self.opt_level
    }

    fn mut_opt_level_for_reflect(&mut self) -> &mut OptimizerOptions_Level {
        &mut self.opt_level
    }

    // .tensorflow.OptimizerOptions.GlobalJitLevel global_jit_level = 5;

    pub fn clear_global_jit_level(&mut self) {
        self.global_jit_level = OptimizerOptions_GlobalJitLevel::DEFAULT;
    }

    // Param is passed by value, moved
    pub fn set_global_jit_level(&mut self, v: OptimizerOptions_GlobalJitLevel) {
        self.global_jit_level = v;
    }

    pub fn get_global_jit_level(&self) -> OptimizerOptions_GlobalJitLevel {
        self.global_jit_level
    }

    fn get_global_jit_level_for_reflect(&self) -> &OptimizerOptions_GlobalJitLevel {
        &self.global_jit_level
    }

    fn mut_global_jit_level_for_reflect(&mut self) -> &mut OptimizerOptions_GlobalJitLevel {
        &mut self.global_jit_level
    }
}

impl ::protobuf::Message for OptimizerOptions {
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
                    let tmp = is.read_bool()?;
                    self.do_common_subexpression_elimination = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.do_constant_folding = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.do_function_inlining = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.opt_level = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.global_jit_level = tmp;
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
        if self.do_common_subexpression_elimination != false {
            my_size += 2;
        }
        if self.do_constant_folding != false {
            my_size += 2;
        }
        if self.do_function_inlining != false {
            my_size += 2;
        }
        if self.opt_level != OptimizerOptions_Level::L1 {
            my_size += ::protobuf::rt::enum_size(3, self.opt_level);
        }
        if self.global_jit_level != OptimizerOptions_GlobalJitLevel::DEFAULT {
            my_size += ::protobuf::rt::enum_size(5, self.global_jit_level);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.do_common_subexpression_elimination != false {
            os.write_bool(1, self.do_common_subexpression_elimination)?;
        }
        if self.do_constant_folding != false {
            os.write_bool(2, self.do_constant_folding)?;
        }
        if self.do_function_inlining != false {
            os.write_bool(4, self.do_function_inlining)?;
        }
        if self.opt_level != OptimizerOptions_Level::L1 {
            os.write_enum(3, self.opt_level.value())?;
        }
        if self.global_jit_level != OptimizerOptions_GlobalJitLevel::DEFAULT {
            os.write_enum(5, self.global_jit_level.value())?;
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

impl ::protobuf::MessageStatic for OptimizerOptions {
    fn new() -> OptimizerOptions {
        OptimizerOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<OptimizerOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "do_common_subexpression_elimination",
                    OptimizerOptions::get_do_common_subexpression_elimination_for_reflect,
                    OptimizerOptions::mut_do_common_subexpression_elimination_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "do_constant_folding",
                    OptimizerOptions::get_do_constant_folding_for_reflect,
                    OptimizerOptions::mut_do_constant_folding_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "do_function_inlining",
                    OptimizerOptions::get_do_function_inlining_for_reflect,
                    OptimizerOptions::mut_do_function_inlining_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OptimizerOptions_Level>>(
                    "opt_level",
                    OptimizerOptions::get_opt_level_for_reflect,
                    OptimizerOptions::mut_opt_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OptimizerOptions_GlobalJitLevel>>(
                    "global_jit_level",
                    OptimizerOptions::get_global_jit_level_for_reflect,
                    OptimizerOptions::mut_global_jit_level_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OptimizerOptions>(
                    "OptimizerOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OptimizerOptions {
    fn clear(&mut self) {
        self.clear_do_common_subexpression_elimination();
        self.clear_do_constant_folding();
        self.clear_do_function_inlining();
        self.clear_opt_level();
        self.clear_global_jit_level();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OptimizerOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OptimizerOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OptimizerOptions_Level {
    L1 = 0,
    L0 = -1,
}

impl ::protobuf::ProtobufEnum for OptimizerOptions_Level {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OptimizerOptions_Level> {
        match value {
            0 => ::std::option::Option::Some(OptimizerOptions_Level::L1),
            -1 => ::std::option::Option::Some(OptimizerOptions_Level::L0),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OptimizerOptions_Level] = &[
            OptimizerOptions_Level::L1,
            OptimizerOptions_Level::L0,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<OptimizerOptions_Level>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OptimizerOptions_Level", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OptimizerOptions_Level {
}

impl ::std::default::Default for OptimizerOptions_Level {
    fn default() -> Self {
        OptimizerOptions_Level::L1
    }
}

impl ::protobuf::reflect::ProtobufValue for OptimizerOptions_Level {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OptimizerOptions_GlobalJitLevel {
    DEFAULT = 0,
    OFF = -1,
    ON_1 = 1,
    ON_2 = 2,
}

impl ::protobuf::ProtobufEnum for OptimizerOptions_GlobalJitLevel {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OptimizerOptions_GlobalJitLevel> {
        match value {
            0 => ::std::option::Option::Some(OptimizerOptions_GlobalJitLevel::DEFAULT),
            -1 => ::std::option::Option::Some(OptimizerOptions_GlobalJitLevel::OFF),
            1 => ::std::option::Option::Some(OptimizerOptions_GlobalJitLevel::ON_1),
            2 => ::std::option::Option::Some(OptimizerOptions_GlobalJitLevel::ON_2),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OptimizerOptions_GlobalJitLevel] = &[
            OptimizerOptions_GlobalJitLevel::DEFAULT,
            OptimizerOptions_GlobalJitLevel::OFF,
            OptimizerOptions_GlobalJitLevel::ON_1,
            OptimizerOptions_GlobalJitLevel::ON_2,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<OptimizerOptions_GlobalJitLevel>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OptimizerOptions_GlobalJitLevel", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OptimizerOptions_GlobalJitLevel {
}

impl ::std::default::Default for OptimizerOptions_GlobalJitLevel {
    fn default() -> Self {
        OptimizerOptions_GlobalJitLevel::DEFAULT
    }
}

impl ::protobuf::reflect::ProtobufValue for OptimizerOptions_GlobalJitLevel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GraphOptions {
    // message fields
    pub enable_recv_scheduling: bool,
    pub optimizer_options: ::protobuf::SingularPtrField<OptimizerOptions>,
    pub build_cost_model: i64,
    pub build_cost_model_after: i64,
    pub infer_shapes: bool,
    pub place_pruned_graph: bool,
    pub enable_bfloat16_sendrecv: bool,
    pub timeline_step: i32,
    pub rewrite_options: ::protobuf::SingularPtrField<super::rewriter_config::RewriterConfig>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GraphOptions {}

impl GraphOptions {
    pub fn new() -> GraphOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GraphOptions {
        static mut instance: ::protobuf::lazy::Lazy<GraphOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GraphOptions,
        };
        unsafe {
            instance.get(GraphOptions::new)
        }
    }

    // bool enable_recv_scheduling = 2;

    pub fn clear_enable_recv_scheduling(&mut self) {
        self.enable_recv_scheduling = false;
    }

    // Param is passed by value, moved
    pub fn set_enable_recv_scheduling(&mut self, v: bool) {
        self.enable_recv_scheduling = v;
    }

    pub fn get_enable_recv_scheduling(&self) -> bool {
        self.enable_recv_scheduling
    }

    fn get_enable_recv_scheduling_for_reflect(&self) -> &bool {
        &self.enable_recv_scheduling
    }

    fn mut_enable_recv_scheduling_for_reflect(&mut self) -> &mut bool {
        &mut self.enable_recv_scheduling
    }

    // .tensorflow.OptimizerOptions optimizer_options = 3;

    pub fn clear_optimizer_options(&mut self) {
        self.optimizer_options.clear();
    }

    pub fn has_optimizer_options(&self) -> bool {
        self.optimizer_options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_optimizer_options(&mut self, v: OptimizerOptions) {
        self.optimizer_options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_optimizer_options(&mut self) -> &mut OptimizerOptions {
        if self.optimizer_options.is_none() {
            self.optimizer_options.set_default();
        }
        self.optimizer_options.as_mut().unwrap()
    }

    // Take field
    pub fn take_optimizer_options(&mut self) -> OptimizerOptions {
        self.optimizer_options.take().unwrap_or_else(|| OptimizerOptions::new())
    }

    pub fn get_optimizer_options(&self) -> &OptimizerOptions {
        self.optimizer_options.as_ref().unwrap_or_else(|| OptimizerOptions::default_instance())
    }

    fn get_optimizer_options_for_reflect(&self) -> &::protobuf::SingularPtrField<OptimizerOptions> {
        &self.optimizer_options
    }

    fn mut_optimizer_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OptimizerOptions> {
        &mut self.optimizer_options
    }

    // int64 build_cost_model = 4;

    pub fn clear_build_cost_model(&mut self) {
        self.build_cost_model = 0;
    }

    // Param is passed by value, moved
    pub fn set_build_cost_model(&mut self, v: i64) {
        self.build_cost_model = v;
    }

    pub fn get_build_cost_model(&self) -> i64 {
        self.build_cost_model
    }

    fn get_build_cost_model_for_reflect(&self) -> &i64 {
        &self.build_cost_model
    }

    fn mut_build_cost_model_for_reflect(&mut self) -> &mut i64 {
        &mut self.build_cost_model
    }

    // int64 build_cost_model_after = 9;

    pub fn clear_build_cost_model_after(&mut self) {
        self.build_cost_model_after = 0;
    }

    // Param is passed by value, moved
    pub fn set_build_cost_model_after(&mut self, v: i64) {
        self.build_cost_model_after = v;
    }

    pub fn get_build_cost_model_after(&self) -> i64 {
        self.build_cost_model_after
    }

    fn get_build_cost_model_after_for_reflect(&self) -> &i64 {
        &self.build_cost_model_after
    }

    fn mut_build_cost_model_after_for_reflect(&mut self) -> &mut i64 {
        &mut self.build_cost_model_after
    }

    // bool infer_shapes = 5;

    pub fn clear_infer_shapes(&mut self) {
        self.infer_shapes = false;
    }

    // Param is passed by value, moved
    pub fn set_infer_shapes(&mut self, v: bool) {
        self.infer_shapes = v;
    }

    pub fn get_infer_shapes(&self) -> bool {
        self.infer_shapes
    }

    fn get_infer_shapes_for_reflect(&self) -> &bool {
        &self.infer_shapes
    }

    fn mut_infer_shapes_for_reflect(&mut self) -> &mut bool {
        &mut self.infer_shapes
    }

    // bool place_pruned_graph = 6;

    pub fn clear_place_pruned_graph(&mut self) {
        self.place_pruned_graph = false;
    }

    // Param is passed by value, moved
    pub fn set_place_pruned_graph(&mut self, v: bool) {
        self.place_pruned_graph = v;
    }

    pub fn get_place_pruned_graph(&self) -> bool {
        self.place_pruned_graph
    }

    fn get_place_pruned_graph_for_reflect(&self) -> &bool {
        &self.place_pruned_graph
    }

    fn mut_place_pruned_graph_for_reflect(&mut self) -> &mut bool {
        &mut self.place_pruned_graph
    }

    // bool enable_bfloat16_sendrecv = 7;

    pub fn clear_enable_bfloat16_sendrecv(&mut self) {
        self.enable_bfloat16_sendrecv = false;
    }

    // Param is passed by value, moved
    pub fn set_enable_bfloat16_sendrecv(&mut self, v: bool) {
        self.enable_bfloat16_sendrecv = v;
    }

    pub fn get_enable_bfloat16_sendrecv(&self) -> bool {
        self.enable_bfloat16_sendrecv
    }

    fn get_enable_bfloat16_sendrecv_for_reflect(&self) -> &bool {
        &self.enable_bfloat16_sendrecv
    }

    fn mut_enable_bfloat16_sendrecv_for_reflect(&mut self) -> &mut bool {
        &mut self.enable_bfloat16_sendrecv
    }

    // int32 timeline_step = 8;

    pub fn clear_timeline_step(&mut self) {
        self.timeline_step = 0;
    }

    // Param is passed by value, moved
    pub fn set_timeline_step(&mut self, v: i32) {
        self.timeline_step = v;
    }

    pub fn get_timeline_step(&self) -> i32 {
        self.timeline_step
    }

    fn get_timeline_step_for_reflect(&self) -> &i32 {
        &self.timeline_step
    }

    fn mut_timeline_step_for_reflect(&mut self) -> &mut i32 {
        &mut self.timeline_step
    }

    // .tensorflow.RewriterConfig rewrite_options = 10;

    pub fn clear_rewrite_options(&mut self) {
        self.rewrite_options.clear();
    }

    pub fn has_rewrite_options(&self) -> bool {
        self.rewrite_options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rewrite_options(&mut self, v: super::rewriter_config::RewriterConfig) {
        self.rewrite_options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rewrite_options(&mut self) -> &mut super::rewriter_config::RewriterConfig {
        if self.rewrite_options.is_none() {
            self.rewrite_options.set_default();
        }
        self.rewrite_options.as_mut().unwrap()
    }

    // Take field
    pub fn take_rewrite_options(&mut self) -> super::rewriter_config::RewriterConfig {
        self.rewrite_options.take().unwrap_or_else(|| super::rewriter_config::RewriterConfig::new())
    }

    pub fn get_rewrite_options(&self) -> &super::rewriter_config::RewriterConfig {
        self.rewrite_options.as_ref().unwrap_or_else(|| super::rewriter_config::RewriterConfig::default_instance())
    }

    fn get_rewrite_options_for_reflect(&self) -> &::protobuf::SingularPtrField<super::rewriter_config::RewriterConfig> {
        &self.rewrite_options
    }

    fn mut_rewrite_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::rewriter_config::RewriterConfig> {
        &mut self.rewrite_options
    }
}

impl ::protobuf::Message for GraphOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.optimizer_options {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.rewrite_options {
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
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.enable_recv_scheduling = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.optimizer_options)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.build_cost_model = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.build_cost_model_after = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.infer_shapes = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.place_pruned_graph = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.enable_bfloat16_sendrecv = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.timeline_step = tmp;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rewrite_options)?;
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
        if self.enable_recv_scheduling != false {
            my_size += 2;
        }
        if let Some(ref v) = self.optimizer_options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.build_cost_model != 0 {
            my_size += ::protobuf::rt::value_size(4, self.build_cost_model, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.build_cost_model_after != 0 {
            my_size += ::protobuf::rt::value_size(9, self.build_cost_model_after, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.infer_shapes != false {
            my_size += 2;
        }
        if self.place_pruned_graph != false {
            my_size += 2;
        }
        if self.enable_bfloat16_sendrecv != false {
            my_size += 2;
        }
        if self.timeline_step != 0 {
            my_size += ::protobuf::rt::value_size(8, self.timeline_step, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.rewrite_options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.enable_recv_scheduling != false {
            os.write_bool(2, self.enable_recv_scheduling)?;
        }
        if let Some(ref v) = self.optimizer_options.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.build_cost_model != 0 {
            os.write_int64(4, self.build_cost_model)?;
        }
        if self.build_cost_model_after != 0 {
            os.write_int64(9, self.build_cost_model_after)?;
        }
        if self.infer_shapes != false {
            os.write_bool(5, self.infer_shapes)?;
        }
        if self.place_pruned_graph != false {
            os.write_bool(6, self.place_pruned_graph)?;
        }
        if self.enable_bfloat16_sendrecv != false {
            os.write_bool(7, self.enable_bfloat16_sendrecv)?;
        }
        if self.timeline_step != 0 {
            os.write_int32(8, self.timeline_step)?;
        }
        if let Some(ref v) = self.rewrite_options.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for GraphOptions {
    fn new() -> GraphOptions {
        GraphOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<GraphOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "enable_recv_scheduling",
                    GraphOptions::get_enable_recv_scheduling_for_reflect,
                    GraphOptions::mut_enable_recv_scheduling_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OptimizerOptions>>(
                    "optimizer_options",
                    GraphOptions::get_optimizer_options_for_reflect,
                    GraphOptions::mut_optimizer_options_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "build_cost_model",
                    GraphOptions::get_build_cost_model_for_reflect,
                    GraphOptions::mut_build_cost_model_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "build_cost_model_after",
                    GraphOptions::get_build_cost_model_after_for_reflect,
                    GraphOptions::mut_build_cost_model_after_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "infer_shapes",
                    GraphOptions::get_infer_shapes_for_reflect,
                    GraphOptions::mut_infer_shapes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "place_pruned_graph",
                    GraphOptions::get_place_pruned_graph_for_reflect,
                    GraphOptions::mut_place_pruned_graph_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "enable_bfloat16_sendrecv",
                    GraphOptions::get_enable_bfloat16_sendrecv_for_reflect,
                    GraphOptions::mut_enable_bfloat16_sendrecv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "timeline_step",
                    GraphOptions::get_timeline_step_for_reflect,
                    GraphOptions::mut_timeline_step_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::rewriter_config::RewriterConfig>>(
                    "rewrite_options",
                    GraphOptions::get_rewrite_options_for_reflect,
                    GraphOptions::mut_rewrite_options_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GraphOptions>(
                    "GraphOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GraphOptions {
    fn clear(&mut self) {
        self.clear_enable_recv_scheduling();
        self.clear_optimizer_options();
        self.clear_build_cost_model();
        self.clear_build_cost_model_after();
        self.clear_infer_shapes();
        self.clear_place_pruned_graph();
        self.clear_enable_bfloat16_sendrecv();
        self.clear_timeline_step();
        self.clear_rewrite_options();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GraphOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GraphOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ThreadPoolOptionProto {
    // message fields
    pub num_threads: i32,
    pub global_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ThreadPoolOptionProto {}

impl ThreadPoolOptionProto {
    pub fn new() -> ThreadPoolOptionProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ThreadPoolOptionProto {
        static mut instance: ::protobuf::lazy::Lazy<ThreadPoolOptionProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ThreadPoolOptionProto,
        };
        unsafe {
            instance.get(ThreadPoolOptionProto::new)
        }
    }

    // int32 num_threads = 1;

    pub fn clear_num_threads(&mut self) {
        self.num_threads = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_threads(&mut self, v: i32) {
        self.num_threads = v;
    }

    pub fn get_num_threads(&self) -> i32 {
        self.num_threads
    }

    fn get_num_threads_for_reflect(&self) -> &i32 {
        &self.num_threads
    }

    fn mut_num_threads_for_reflect(&mut self) -> &mut i32 {
        &mut self.num_threads
    }

    // string global_name = 2;

    pub fn clear_global_name(&mut self) {
        self.global_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_global_name(&mut self, v: ::std::string::String) {
        self.global_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_global_name(&mut self) -> &mut ::std::string::String {
        &mut self.global_name
    }

    // Take field
    pub fn take_global_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.global_name, ::std::string::String::new())
    }

    pub fn get_global_name(&self) -> &str {
        &self.global_name
    }

    fn get_global_name_for_reflect(&self) -> &::std::string::String {
        &self.global_name
    }

    fn mut_global_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.global_name
    }
}

impl ::protobuf::Message for ThreadPoolOptionProto {
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
                    self.num_threads = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.global_name)?;
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
        if self.num_threads != 0 {
            my_size += ::protobuf::rt::value_size(1, self.num_threads, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.global_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.global_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.num_threads != 0 {
            os.write_int32(1, self.num_threads)?;
        }
        if !self.global_name.is_empty() {
            os.write_string(2, &self.global_name)?;
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

impl ::protobuf::MessageStatic for ThreadPoolOptionProto {
    fn new() -> ThreadPoolOptionProto {
        ThreadPoolOptionProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ThreadPoolOptionProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_threads",
                    ThreadPoolOptionProto::get_num_threads_for_reflect,
                    ThreadPoolOptionProto::mut_num_threads_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "global_name",
                    ThreadPoolOptionProto::get_global_name_for_reflect,
                    ThreadPoolOptionProto::mut_global_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ThreadPoolOptionProto>(
                    "ThreadPoolOptionProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ThreadPoolOptionProto {
    fn clear(&mut self) {
        self.clear_num_threads();
        self.clear_global_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ThreadPoolOptionProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ThreadPoolOptionProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RPCOptions {
    // message fields
    pub use_rpc_for_inprocess_master: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RPCOptions {}

impl RPCOptions {
    pub fn new() -> RPCOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RPCOptions {
        static mut instance: ::protobuf::lazy::Lazy<RPCOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RPCOptions,
        };
        unsafe {
            instance.get(RPCOptions::new)
        }
    }

    // bool use_rpc_for_inprocess_master = 1;

    pub fn clear_use_rpc_for_inprocess_master(&mut self) {
        self.use_rpc_for_inprocess_master = false;
    }

    // Param is passed by value, moved
    pub fn set_use_rpc_for_inprocess_master(&mut self, v: bool) {
        self.use_rpc_for_inprocess_master = v;
    }

    pub fn get_use_rpc_for_inprocess_master(&self) -> bool {
        self.use_rpc_for_inprocess_master
    }

    fn get_use_rpc_for_inprocess_master_for_reflect(&self) -> &bool {
        &self.use_rpc_for_inprocess_master
    }

    fn mut_use_rpc_for_inprocess_master_for_reflect(&mut self) -> &mut bool {
        &mut self.use_rpc_for_inprocess_master
    }
}

impl ::protobuf::Message for RPCOptions {
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
                    let tmp = is.read_bool()?;
                    self.use_rpc_for_inprocess_master = tmp;
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
        if self.use_rpc_for_inprocess_master != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.use_rpc_for_inprocess_master != false {
            os.write_bool(1, self.use_rpc_for_inprocess_master)?;
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

impl ::protobuf::MessageStatic for RPCOptions {
    fn new() -> RPCOptions {
        RPCOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<RPCOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "use_rpc_for_inprocess_master",
                    RPCOptions::get_use_rpc_for_inprocess_master_for_reflect,
                    RPCOptions::mut_use_rpc_for_inprocess_master_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RPCOptions>(
                    "RPCOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RPCOptions {
    fn clear(&mut self) {
        self.clear_use_rpc_for_inprocess_master();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RPCOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RPCOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConfigProto {
    // message fields
    pub device_count: ::std::collections::HashMap<::std::string::String, i32>,
    pub intra_op_parallelism_threads: i32,
    pub inter_op_parallelism_threads: i32,
    pub use_per_session_threads: bool,
    pub session_inter_op_thread_pool: ::protobuf::RepeatedField<ThreadPoolOptionProto>,
    pub placement_period: i32,
    pub device_filters: ::protobuf::RepeatedField<::std::string::String>,
    pub gpu_options: ::protobuf::SingularPtrField<GPUOptions>,
    pub allow_soft_placement: bool,
    pub log_device_placement: bool,
    pub graph_options: ::protobuf::SingularPtrField<GraphOptions>,
    pub operation_timeout_in_ms: i64,
    pub rpc_options: ::protobuf::SingularPtrField<RPCOptions>,
    pub cluster_def: ::protobuf::SingularPtrField<super::cluster::ClusterDef>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ConfigProto {}

impl ConfigProto {
    pub fn new() -> ConfigProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ConfigProto {
        static mut instance: ::protobuf::lazy::Lazy<ConfigProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ConfigProto,
        };
        unsafe {
            instance.get(ConfigProto::new)
        }
    }

    // repeated .tensorflow.ConfigProto.DeviceCountEntry device_count = 1;

    pub fn clear_device_count(&mut self) {
        self.device_count.clear();
    }

    // Param is passed by value, moved
    pub fn set_device_count(&mut self, v: ::std::collections::HashMap<::std::string::String, i32>) {
        self.device_count = v;
    }

    // Mutable pointer to the field.
    pub fn mut_device_count(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, i32> {
        &mut self.device_count
    }

    // Take field
    pub fn take_device_count(&mut self) -> ::std::collections::HashMap<::std::string::String, i32> {
        ::std::mem::replace(&mut self.device_count, ::std::collections::HashMap::new())
    }

    pub fn get_device_count(&self) -> &::std::collections::HashMap<::std::string::String, i32> {
        &self.device_count
    }

    fn get_device_count_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, i32> {
        &self.device_count
    }

    fn mut_device_count_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, i32> {
        &mut self.device_count
    }

    // int32 intra_op_parallelism_threads = 2;

    pub fn clear_intra_op_parallelism_threads(&mut self) {
        self.intra_op_parallelism_threads = 0;
    }

    // Param is passed by value, moved
    pub fn set_intra_op_parallelism_threads(&mut self, v: i32) {
        self.intra_op_parallelism_threads = v;
    }

    pub fn get_intra_op_parallelism_threads(&self) -> i32 {
        self.intra_op_parallelism_threads
    }

    fn get_intra_op_parallelism_threads_for_reflect(&self) -> &i32 {
        &self.intra_op_parallelism_threads
    }

    fn mut_intra_op_parallelism_threads_for_reflect(&mut self) -> &mut i32 {
        &mut self.intra_op_parallelism_threads
    }

    // int32 inter_op_parallelism_threads = 5;

    pub fn clear_inter_op_parallelism_threads(&mut self) {
        self.inter_op_parallelism_threads = 0;
    }

    // Param is passed by value, moved
    pub fn set_inter_op_parallelism_threads(&mut self, v: i32) {
        self.inter_op_parallelism_threads = v;
    }

    pub fn get_inter_op_parallelism_threads(&self) -> i32 {
        self.inter_op_parallelism_threads
    }

    fn get_inter_op_parallelism_threads_for_reflect(&self) -> &i32 {
        &self.inter_op_parallelism_threads
    }

    fn mut_inter_op_parallelism_threads_for_reflect(&mut self) -> &mut i32 {
        &mut self.inter_op_parallelism_threads
    }

    // bool use_per_session_threads = 9;

    pub fn clear_use_per_session_threads(&mut self) {
        self.use_per_session_threads = false;
    }

    // Param is passed by value, moved
    pub fn set_use_per_session_threads(&mut self, v: bool) {
        self.use_per_session_threads = v;
    }

    pub fn get_use_per_session_threads(&self) -> bool {
        self.use_per_session_threads
    }

    fn get_use_per_session_threads_for_reflect(&self) -> &bool {
        &self.use_per_session_threads
    }

    fn mut_use_per_session_threads_for_reflect(&mut self) -> &mut bool {
        &mut self.use_per_session_threads
    }

    // repeated .tensorflow.ThreadPoolOptionProto session_inter_op_thread_pool = 12;

    pub fn clear_session_inter_op_thread_pool(&mut self) {
        self.session_inter_op_thread_pool.clear();
    }

    // Param is passed by value, moved
    pub fn set_session_inter_op_thread_pool(&mut self, v: ::protobuf::RepeatedField<ThreadPoolOptionProto>) {
        self.session_inter_op_thread_pool = v;
    }

    // Mutable pointer to the field.
    pub fn mut_session_inter_op_thread_pool(&mut self) -> &mut ::protobuf::RepeatedField<ThreadPoolOptionProto> {
        &mut self.session_inter_op_thread_pool
    }

    // Take field
    pub fn take_session_inter_op_thread_pool(&mut self) -> ::protobuf::RepeatedField<ThreadPoolOptionProto> {
        ::std::mem::replace(&mut self.session_inter_op_thread_pool, ::protobuf::RepeatedField::new())
    }

    pub fn get_session_inter_op_thread_pool(&self) -> &[ThreadPoolOptionProto] {
        &self.session_inter_op_thread_pool
    }

    fn get_session_inter_op_thread_pool_for_reflect(&self) -> &::protobuf::RepeatedField<ThreadPoolOptionProto> {
        &self.session_inter_op_thread_pool
    }

    fn mut_session_inter_op_thread_pool_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ThreadPoolOptionProto> {
        &mut self.session_inter_op_thread_pool
    }

    // int32 placement_period = 3;

    pub fn clear_placement_period(&mut self) {
        self.placement_period = 0;
    }

    // Param is passed by value, moved
    pub fn set_placement_period(&mut self, v: i32) {
        self.placement_period = v;
    }

    pub fn get_placement_period(&self) -> i32 {
        self.placement_period
    }

    fn get_placement_period_for_reflect(&self) -> &i32 {
        &self.placement_period
    }

    fn mut_placement_period_for_reflect(&mut self) -> &mut i32 {
        &mut self.placement_period
    }

    // repeated string device_filters = 4;

    pub fn clear_device_filters(&mut self) {
        self.device_filters.clear();
    }

    // Param is passed by value, moved
    pub fn set_device_filters(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.device_filters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_device_filters(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.device_filters
    }

    // Take field
    pub fn take_device_filters(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.device_filters, ::protobuf::RepeatedField::new())
    }

    pub fn get_device_filters(&self) -> &[::std::string::String] {
        &self.device_filters
    }

    fn get_device_filters_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.device_filters
    }

    fn mut_device_filters_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.device_filters
    }

    // .tensorflow.GPUOptions gpu_options = 6;

    pub fn clear_gpu_options(&mut self) {
        self.gpu_options.clear();
    }

    pub fn has_gpu_options(&self) -> bool {
        self.gpu_options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gpu_options(&mut self, v: GPUOptions) {
        self.gpu_options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gpu_options(&mut self) -> &mut GPUOptions {
        if self.gpu_options.is_none() {
            self.gpu_options.set_default();
        }
        self.gpu_options.as_mut().unwrap()
    }

    // Take field
    pub fn take_gpu_options(&mut self) -> GPUOptions {
        self.gpu_options.take().unwrap_or_else(|| GPUOptions::new())
    }

    pub fn get_gpu_options(&self) -> &GPUOptions {
        self.gpu_options.as_ref().unwrap_or_else(|| GPUOptions::default_instance())
    }

    fn get_gpu_options_for_reflect(&self) -> &::protobuf::SingularPtrField<GPUOptions> {
        &self.gpu_options
    }

    fn mut_gpu_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<GPUOptions> {
        &mut self.gpu_options
    }

    // bool allow_soft_placement = 7;

    pub fn clear_allow_soft_placement(&mut self) {
        self.allow_soft_placement = false;
    }

    // Param is passed by value, moved
    pub fn set_allow_soft_placement(&mut self, v: bool) {
        self.allow_soft_placement = v;
    }

    pub fn get_allow_soft_placement(&self) -> bool {
        self.allow_soft_placement
    }

    fn get_allow_soft_placement_for_reflect(&self) -> &bool {
        &self.allow_soft_placement
    }

    fn mut_allow_soft_placement_for_reflect(&mut self) -> &mut bool {
        &mut self.allow_soft_placement
    }

    // bool log_device_placement = 8;

    pub fn clear_log_device_placement(&mut self) {
        self.log_device_placement = false;
    }

    // Param is passed by value, moved
    pub fn set_log_device_placement(&mut self, v: bool) {
        self.log_device_placement = v;
    }

    pub fn get_log_device_placement(&self) -> bool {
        self.log_device_placement
    }

    fn get_log_device_placement_for_reflect(&self) -> &bool {
        &self.log_device_placement
    }

    fn mut_log_device_placement_for_reflect(&mut self) -> &mut bool {
        &mut self.log_device_placement
    }

    // .tensorflow.GraphOptions graph_options = 10;

    pub fn clear_graph_options(&mut self) {
        self.graph_options.clear();
    }

    pub fn has_graph_options(&self) -> bool {
        self.graph_options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_graph_options(&mut self, v: GraphOptions) {
        self.graph_options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_graph_options(&mut self) -> &mut GraphOptions {
        if self.graph_options.is_none() {
            self.graph_options.set_default();
        }
        self.graph_options.as_mut().unwrap()
    }

    // Take field
    pub fn take_graph_options(&mut self) -> GraphOptions {
        self.graph_options.take().unwrap_or_else(|| GraphOptions::new())
    }

    pub fn get_graph_options(&self) -> &GraphOptions {
        self.graph_options.as_ref().unwrap_or_else(|| GraphOptions::default_instance())
    }

    fn get_graph_options_for_reflect(&self) -> &::protobuf::SingularPtrField<GraphOptions> {
        &self.graph_options
    }

    fn mut_graph_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<GraphOptions> {
        &mut self.graph_options
    }

    // int64 operation_timeout_in_ms = 11;

    pub fn clear_operation_timeout_in_ms(&mut self) {
        self.operation_timeout_in_ms = 0;
    }

    // Param is passed by value, moved
    pub fn set_operation_timeout_in_ms(&mut self, v: i64) {
        self.operation_timeout_in_ms = v;
    }

    pub fn get_operation_timeout_in_ms(&self) -> i64 {
        self.operation_timeout_in_ms
    }

    fn get_operation_timeout_in_ms_for_reflect(&self) -> &i64 {
        &self.operation_timeout_in_ms
    }

    fn mut_operation_timeout_in_ms_for_reflect(&mut self) -> &mut i64 {
        &mut self.operation_timeout_in_ms
    }

    // .tensorflow.RPCOptions rpc_options = 13;

    pub fn clear_rpc_options(&mut self) {
        self.rpc_options.clear();
    }

    pub fn has_rpc_options(&self) -> bool {
        self.rpc_options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rpc_options(&mut self, v: RPCOptions) {
        self.rpc_options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rpc_options(&mut self) -> &mut RPCOptions {
        if self.rpc_options.is_none() {
            self.rpc_options.set_default();
        }
        self.rpc_options.as_mut().unwrap()
    }

    // Take field
    pub fn take_rpc_options(&mut self) -> RPCOptions {
        self.rpc_options.take().unwrap_or_else(|| RPCOptions::new())
    }

    pub fn get_rpc_options(&self) -> &RPCOptions {
        self.rpc_options.as_ref().unwrap_or_else(|| RPCOptions::default_instance())
    }

    fn get_rpc_options_for_reflect(&self) -> &::protobuf::SingularPtrField<RPCOptions> {
        &self.rpc_options
    }

    fn mut_rpc_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RPCOptions> {
        &mut self.rpc_options
    }

    // .tensorflow.ClusterDef cluster_def = 14;

    pub fn clear_cluster_def(&mut self) {
        self.cluster_def.clear();
    }

    pub fn has_cluster_def(&self) -> bool {
        self.cluster_def.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cluster_def(&mut self, v: super::cluster::ClusterDef) {
        self.cluster_def = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cluster_def(&mut self) -> &mut super::cluster::ClusterDef {
        if self.cluster_def.is_none() {
            self.cluster_def.set_default();
        }
        self.cluster_def.as_mut().unwrap()
    }

    // Take field
    pub fn take_cluster_def(&mut self) -> super::cluster::ClusterDef {
        self.cluster_def.take().unwrap_or_else(|| super::cluster::ClusterDef::new())
    }

    pub fn get_cluster_def(&self) -> &super::cluster::ClusterDef {
        self.cluster_def.as_ref().unwrap_or_else(|| super::cluster::ClusterDef::default_instance())
    }

    fn get_cluster_def_for_reflect(&self) -> &::protobuf::SingularPtrField<super::cluster::ClusterDef> {
        &self.cluster_def
    }

    fn mut_cluster_def_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::cluster::ClusterDef> {
        &mut self.cluster_def
    }
}

impl ::protobuf::Message for ConfigProto {
    fn is_initialized(&self) -> bool {
        for v in &self.session_inter_op_thread_pool {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.gpu_options {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.graph_options {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.rpc_options {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cluster_def {
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
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeInt32>(wire_type, is, &mut self.device_count)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.intra_op_parallelism_threads = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.inter_op_parallelism_threads = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.use_per_session_threads = tmp;
                },
                12 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.session_inter_op_thread_pool)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.placement_period = tmp;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.device_filters)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.gpu_options)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.allow_soft_placement = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.log_device_placement = tmp;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.graph_options)?;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.operation_timeout_in_ms = tmp;
                },
                13 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rpc_options)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cluster_def)?;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeInt32>(1, &self.device_count);
        if self.intra_op_parallelism_threads != 0 {
            my_size += ::protobuf::rt::value_size(2, self.intra_op_parallelism_threads, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.inter_op_parallelism_threads != 0 {
            my_size += ::protobuf::rt::value_size(5, self.inter_op_parallelism_threads, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.use_per_session_threads != false {
            my_size += 2;
        }
        for value in &self.session_inter_op_thread_pool {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.placement_period != 0 {
            my_size += ::protobuf::rt::value_size(3, self.placement_period, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.device_filters {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if let Some(ref v) = self.gpu_options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.allow_soft_placement != false {
            my_size += 2;
        }
        if self.log_device_placement != false {
            my_size += 2;
        }
        if let Some(ref v) = self.graph_options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.operation_timeout_in_ms != 0 {
            my_size += ::protobuf::rt::value_size(11, self.operation_timeout_in_ms, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.rpc_options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.cluster_def.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeInt32>(1, &self.device_count, os)?;
        if self.intra_op_parallelism_threads != 0 {
            os.write_int32(2, self.intra_op_parallelism_threads)?;
        }
        if self.inter_op_parallelism_threads != 0 {
            os.write_int32(5, self.inter_op_parallelism_threads)?;
        }
        if self.use_per_session_threads != false {
            os.write_bool(9, self.use_per_session_threads)?;
        }
        for v in &self.session_inter_op_thread_pool {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.placement_period != 0 {
            os.write_int32(3, self.placement_period)?;
        }
        for v in &self.device_filters {
            os.write_string(4, &v)?;
        };
        if let Some(ref v) = self.gpu_options.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.allow_soft_placement != false {
            os.write_bool(7, self.allow_soft_placement)?;
        }
        if self.log_device_placement != false {
            os.write_bool(8, self.log_device_placement)?;
        }
        if let Some(ref v) = self.graph_options.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.operation_timeout_in_ms != 0 {
            os.write_int64(11, self.operation_timeout_in_ms)?;
        }
        if let Some(ref v) = self.rpc_options.as_ref() {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.cluster_def.as_ref() {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ConfigProto {
    fn new() -> ConfigProto {
        ConfigProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ConfigProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeInt32>(
                    "device_count",
                    ConfigProto::get_device_count_for_reflect,
                    ConfigProto::mut_device_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "intra_op_parallelism_threads",
                    ConfigProto::get_intra_op_parallelism_threads_for_reflect,
                    ConfigProto::mut_intra_op_parallelism_threads_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "inter_op_parallelism_threads",
                    ConfigProto::get_inter_op_parallelism_threads_for_reflect,
                    ConfigProto::mut_inter_op_parallelism_threads_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "use_per_session_threads",
                    ConfigProto::get_use_per_session_threads_for_reflect,
                    ConfigProto::mut_use_per_session_threads_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ThreadPoolOptionProto>>(
                    "session_inter_op_thread_pool",
                    ConfigProto::get_session_inter_op_thread_pool_for_reflect,
                    ConfigProto::mut_session_inter_op_thread_pool_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "placement_period",
                    ConfigProto::get_placement_period_for_reflect,
                    ConfigProto::mut_placement_period_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "device_filters",
                    ConfigProto::get_device_filters_for_reflect,
                    ConfigProto::mut_device_filters_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GPUOptions>>(
                    "gpu_options",
                    ConfigProto::get_gpu_options_for_reflect,
                    ConfigProto::mut_gpu_options_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allow_soft_placement",
                    ConfigProto::get_allow_soft_placement_for_reflect,
                    ConfigProto::mut_allow_soft_placement_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "log_device_placement",
                    ConfigProto::get_log_device_placement_for_reflect,
                    ConfigProto::mut_log_device_placement_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GraphOptions>>(
                    "graph_options",
                    ConfigProto::get_graph_options_for_reflect,
                    ConfigProto::mut_graph_options_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "operation_timeout_in_ms",
                    ConfigProto::get_operation_timeout_in_ms_for_reflect,
                    ConfigProto::mut_operation_timeout_in_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RPCOptions>>(
                    "rpc_options",
                    ConfigProto::get_rpc_options_for_reflect,
                    ConfigProto::mut_rpc_options_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::cluster::ClusterDef>>(
                    "cluster_def",
                    ConfigProto::get_cluster_def_for_reflect,
                    ConfigProto::mut_cluster_def_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ConfigProto>(
                    "ConfigProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ConfigProto {
    fn clear(&mut self) {
        self.clear_device_count();
        self.clear_intra_op_parallelism_threads();
        self.clear_inter_op_parallelism_threads();
        self.clear_use_per_session_threads();
        self.clear_session_inter_op_thread_pool();
        self.clear_placement_period();
        self.clear_device_filters();
        self.clear_gpu_options();
        self.clear_allow_soft_placement();
        self.clear_log_device_placement();
        self.clear_graph_options();
        self.clear_operation_timeout_in_ms();
        self.clear_rpc_options();
        self.clear_cluster_def();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConfigProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConfigProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RunOptions {
    // message fields
    pub trace_level: RunOptions_TraceLevel,
    pub timeout_in_ms: i64,
    pub inter_op_thread_pool: i32,
    pub output_partition_graphs: bool,
    pub debug_options: ::protobuf::SingularPtrField<super::debug::DebugOptions>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RunOptions {}

impl RunOptions {
    pub fn new() -> RunOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RunOptions {
        static mut instance: ::protobuf::lazy::Lazy<RunOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RunOptions,
        };
        unsafe {
            instance.get(RunOptions::new)
        }
    }

    // .tensorflow.RunOptions.TraceLevel trace_level = 1;

    pub fn clear_trace_level(&mut self) {
        self.trace_level = RunOptions_TraceLevel::NO_TRACE;
    }

    // Param is passed by value, moved
    pub fn set_trace_level(&mut self, v: RunOptions_TraceLevel) {
        self.trace_level = v;
    }

    pub fn get_trace_level(&self) -> RunOptions_TraceLevel {
        self.trace_level
    }

    fn get_trace_level_for_reflect(&self) -> &RunOptions_TraceLevel {
        &self.trace_level
    }

    fn mut_trace_level_for_reflect(&mut self) -> &mut RunOptions_TraceLevel {
        &mut self.trace_level
    }

    // int64 timeout_in_ms = 2;

    pub fn clear_timeout_in_ms(&mut self) {
        self.timeout_in_ms = 0;
    }

    // Param is passed by value, moved
    pub fn set_timeout_in_ms(&mut self, v: i64) {
        self.timeout_in_ms = v;
    }

    pub fn get_timeout_in_ms(&self) -> i64 {
        self.timeout_in_ms
    }

    fn get_timeout_in_ms_for_reflect(&self) -> &i64 {
        &self.timeout_in_ms
    }

    fn mut_timeout_in_ms_for_reflect(&mut self) -> &mut i64 {
        &mut self.timeout_in_ms
    }

    // int32 inter_op_thread_pool = 3;

    pub fn clear_inter_op_thread_pool(&mut self) {
        self.inter_op_thread_pool = 0;
    }

    // Param is passed by value, moved
    pub fn set_inter_op_thread_pool(&mut self, v: i32) {
        self.inter_op_thread_pool = v;
    }

    pub fn get_inter_op_thread_pool(&self) -> i32 {
        self.inter_op_thread_pool
    }

    fn get_inter_op_thread_pool_for_reflect(&self) -> &i32 {
        &self.inter_op_thread_pool
    }

    fn mut_inter_op_thread_pool_for_reflect(&mut self) -> &mut i32 {
        &mut self.inter_op_thread_pool
    }

    // bool output_partition_graphs = 5;

    pub fn clear_output_partition_graphs(&mut self) {
        self.output_partition_graphs = false;
    }

    // Param is passed by value, moved
    pub fn set_output_partition_graphs(&mut self, v: bool) {
        self.output_partition_graphs = v;
    }

    pub fn get_output_partition_graphs(&self) -> bool {
        self.output_partition_graphs
    }

    fn get_output_partition_graphs_for_reflect(&self) -> &bool {
        &self.output_partition_graphs
    }

    fn mut_output_partition_graphs_for_reflect(&mut self) -> &mut bool {
        &mut self.output_partition_graphs
    }

    // .tensorflow.DebugOptions debug_options = 6;

    pub fn clear_debug_options(&mut self) {
        self.debug_options.clear();
    }

    pub fn has_debug_options(&self) -> bool {
        self.debug_options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_debug_options(&mut self, v: super::debug::DebugOptions) {
        self.debug_options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_debug_options(&mut self) -> &mut super::debug::DebugOptions {
        if self.debug_options.is_none() {
            self.debug_options.set_default();
        }
        self.debug_options.as_mut().unwrap()
    }

    // Take field
    pub fn take_debug_options(&mut self) -> super::debug::DebugOptions {
        self.debug_options.take().unwrap_or_else(|| super::debug::DebugOptions::new())
    }

    pub fn get_debug_options(&self) -> &super::debug::DebugOptions {
        self.debug_options.as_ref().unwrap_or_else(|| super::debug::DebugOptions::default_instance())
    }

    fn get_debug_options_for_reflect(&self) -> &::protobuf::SingularPtrField<super::debug::DebugOptions> {
        &self.debug_options
    }

    fn mut_debug_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::debug::DebugOptions> {
        &mut self.debug_options
    }
}

impl ::protobuf::Message for RunOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.debug_options {
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
                    let tmp = is.read_enum()?;
                    self.trace_level = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.timeout_in_ms = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.inter_op_thread_pool = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.output_partition_graphs = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.debug_options)?;
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
        if self.trace_level != RunOptions_TraceLevel::NO_TRACE {
            my_size += ::protobuf::rt::enum_size(1, self.trace_level);
        }
        if self.timeout_in_ms != 0 {
            my_size += ::protobuf::rt::value_size(2, self.timeout_in_ms, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.inter_op_thread_pool != 0 {
            my_size += ::protobuf::rt::value_size(3, self.inter_op_thread_pool, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.output_partition_graphs != false {
            my_size += 2;
        }
        if let Some(ref v) = self.debug_options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.trace_level != RunOptions_TraceLevel::NO_TRACE {
            os.write_enum(1, self.trace_level.value())?;
        }
        if self.timeout_in_ms != 0 {
            os.write_int64(2, self.timeout_in_ms)?;
        }
        if self.inter_op_thread_pool != 0 {
            os.write_int32(3, self.inter_op_thread_pool)?;
        }
        if self.output_partition_graphs != false {
            os.write_bool(5, self.output_partition_graphs)?;
        }
        if let Some(ref v) = self.debug_options.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RunOptions {
    fn new() -> RunOptions {
        RunOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<RunOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RunOptions_TraceLevel>>(
                    "trace_level",
                    RunOptions::get_trace_level_for_reflect,
                    RunOptions::mut_trace_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "timeout_in_ms",
                    RunOptions::get_timeout_in_ms_for_reflect,
                    RunOptions::mut_timeout_in_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "inter_op_thread_pool",
                    RunOptions::get_inter_op_thread_pool_for_reflect,
                    RunOptions::mut_inter_op_thread_pool_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "output_partition_graphs",
                    RunOptions::get_output_partition_graphs_for_reflect,
                    RunOptions::mut_output_partition_graphs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::debug::DebugOptions>>(
                    "debug_options",
                    RunOptions::get_debug_options_for_reflect,
                    RunOptions::mut_debug_options_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RunOptions>(
                    "RunOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RunOptions {
    fn clear(&mut self) {
        self.clear_trace_level();
        self.clear_timeout_in_ms();
        self.clear_inter_op_thread_pool();
        self.clear_output_partition_graphs();
        self.clear_debug_options();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RunOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RunOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RunOptions_TraceLevel {
    NO_TRACE = 0,
    SOFTWARE_TRACE = 1,
    HARDWARE_TRACE = 2,
    FULL_TRACE = 3,
}

impl ::protobuf::ProtobufEnum for RunOptions_TraceLevel {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RunOptions_TraceLevel> {
        match value {
            0 => ::std::option::Option::Some(RunOptions_TraceLevel::NO_TRACE),
            1 => ::std::option::Option::Some(RunOptions_TraceLevel::SOFTWARE_TRACE),
            2 => ::std::option::Option::Some(RunOptions_TraceLevel::HARDWARE_TRACE),
            3 => ::std::option::Option::Some(RunOptions_TraceLevel::FULL_TRACE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RunOptions_TraceLevel] = &[
            RunOptions_TraceLevel::NO_TRACE,
            RunOptions_TraceLevel::SOFTWARE_TRACE,
            RunOptions_TraceLevel::HARDWARE_TRACE,
            RunOptions_TraceLevel::FULL_TRACE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RunOptions_TraceLevel>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RunOptions_TraceLevel", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RunOptions_TraceLevel {
}

impl ::std::default::Default for RunOptions_TraceLevel {
    fn default() -> Self {
        RunOptions_TraceLevel::NO_TRACE
    }
}

impl ::protobuf::reflect::ProtobufValue for RunOptions_TraceLevel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RunMetadata {
    // message fields
    pub step_stats: ::protobuf::SingularPtrField<super::step_stats::StepStats>,
    pub cost_graph: ::protobuf::SingularPtrField<super::cost_graph::CostGraphDef>,
    pub partition_graphs: ::protobuf::RepeatedField<super::graph::GraphDef>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RunMetadata {}

impl RunMetadata {
    pub fn new() -> RunMetadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RunMetadata {
        static mut instance: ::protobuf::lazy::Lazy<RunMetadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RunMetadata,
        };
        unsafe {
            instance.get(RunMetadata::new)
        }
    }

    // .tensorflow.StepStats step_stats = 1;

    pub fn clear_step_stats(&mut self) {
        self.step_stats.clear();
    }

    pub fn has_step_stats(&self) -> bool {
        self.step_stats.is_some()
    }

    // Param is passed by value, moved
    pub fn set_step_stats(&mut self, v: super::step_stats::StepStats) {
        self.step_stats = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_step_stats(&mut self) -> &mut super::step_stats::StepStats {
        if self.step_stats.is_none() {
            self.step_stats.set_default();
        }
        self.step_stats.as_mut().unwrap()
    }

    // Take field
    pub fn take_step_stats(&mut self) -> super::step_stats::StepStats {
        self.step_stats.take().unwrap_or_else(|| super::step_stats::StepStats::new())
    }

    pub fn get_step_stats(&self) -> &super::step_stats::StepStats {
        self.step_stats.as_ref().unwrap_or_else(|| super::step_stats::StepStats::default_instance())
    }

    fn get_step_stats_for_reflect(&self) -> &::protobuf::SingularPtrField<super::step_stats::StepStats> {
        &self.step_stats
    }

    fn mut_step_stats_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::step_stats::StepStats> {
        &mut self.step_stats
    }

    // .tensorflow.CostGraphDef cost_graph = 2;

    pub fn clear_cost_graph(&mut self) {
        self.cost_graph.clear();
    }

    pub fn has_cost_graph(&self) -> bool {
        self.cost_graph.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cost_graph(&mut self, v: super::cost_graph::CostGraphDef) {
        self.cost_graph = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cost_graph(&mut self) -> &mut super::cost_graph::CostGraphDef {
        if self.cost_graph.is_none() {
            self.cost_graph.set_default();
        }
        self.cost_graph.as_mut().unwrap()
    }

    // Take field
    pub fn take_cost_graph(&mut self) -> super::cost_graph::CostGraphDef {
        self.cost_graph.take().unwrap_or_else(|| super::cost_graph::CostGraphDef::new())
    }

    pub fn get_cost_graph(&self) -> &super::cost_graph::CostGraphDef {
        self.cost_graph.as_ref().unwrap_or_else(|| super::cost_graph::CostGraphDef::default_instance())
    }

    fn get_cost_graph_for_reflect(&self) -> &::protobuf::SingularPtrField<super::cost_graph::CostGraphDef> {
        &self.cost_graph
    }

    fn mut_cost_graph_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::cost_graph::CostGraphDef> {
        &mut self.cost_graph
    }

    // repeated .tensorflow.GraphDef partition_graphs = 3;

    pub fn clear_partition_graphs(&mut self) {
        self.partition_graphs.clear();
    }

    // Param is passed by value, moved
    pub fn set_partition_graphs(&mut self, v: ::protobuf::RepeatedField<super::graph::GraphDef>) {
        self.partition_graphs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_partition_graphs(&mut self) -> &mut ::protobuf::RepeatedField<super::graph::GraphDef> {
        &mut self.partition_graphs
    }

    // Take field
    pub fn take_partition_graphs(&mut self) -> ::protobuf::RepeatedField<super::graph::GraphDef> {
        ::std::mem::replace(&mut self.partition_graphs, ::protobuf::RepeatedField::new())
    }

    pub fn get_partition_graphs(&self) -> &[super::graph::GraphDef] {
        &self.partition_graphs
    }

    fn get_partition_graphs_for_reflect(&self) -> &::protobuf::RepeatedField<super::graph::GraphDef> {
        &self.partition_graphs
    }

    fn mut_partition_graphs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::graph::GraphDef> {
        &mut self.partition_graphs
    }
}

impl ::protobuf::Message for RunMetadata {
    fn is_initialized(&self) -> bool {
        for v in &self.step_stats {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cost_graph {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.partition_graphs {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.step_stats)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cost_graph)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.partition_graphs)?;
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
        if let Some(ref v) = self.step_stats.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.cost_graph.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.partition_graphs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.step_stats.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.cost_graph.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.partition_graphs {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RunMetadata {
    fn new() -> RunMetadata {
        RunMetadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<RunMetadata>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::step_stats::StepStats>>(
                    "step_stats",
                    RunMetadata::get_step_stats_for_reflect,
                    RunMetadata::mut_step_stats_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::cost_graph::CostGraphDef>>(
                    "cost_graph",
                    RunMetadata::get_cost_graph_for_reflect,
                    RunMetadata::mut_cost_graph_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::graph::GraphDef>>(
                    "partition_graphs",
                    RunMetadata::get_partition_graphs_for_reflect,
                    RunMetadata::mut_partition_graphs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RunMetadata>(
                    "RunMetadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RunMetadata {
    fn clear(&mut self) {
        self.clear_step_stats();
        self.clear_cost_graph();
        self.clear_partition_graphs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RunMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RunMetadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%tensorflow/core/protobuf/config.proto\x12\ntensorflow\x1a*tensorflow/\
    core/framework/cost_graph.proto\x1a%tensorflow/core/framework/graph.prot\
    o\x1a*tensorflow/core/framework/step_stats.proto\x1a$tensorflow/core/pro\
    tobuf/debug.proto\x1a&tensorflow/core/protobuf/cluster.proto\x1a.tensorf\
    low/core/protobuf/rewriter_config.proto\"\xb4\x03\n\nGPUOptions\x12D\n\
    \x1fper_process_gpu_memory_fraction\x18\x01\x20\x01(\x01R\x1bperProcessG\
    puMemoryFraction\x12%\n\x0eallocator_type\x18\x02\x20\x01(\tR\rallocator\
    Type\x126\n\x17deferred_deletion_bytes\x18\x03\x20\x01(\x03R\x15deferred\
    DeletionBytes\x12!\n\x0callow_growth\x18\x04\x20\x01(\x08R\x0ballowGrowt\
    h\x12.\n\x13visible_device_list\x18\x05\x20\x01(\tR\x11visibleDeviceList\
    \x12;\n\x1apolling_active_delay_usecs\x18\x06\x20\x01(\x05R\x17pollingAc\
    tiveDelayUsecs\x12?\n\x1cpolling_inactive_delay_msecs\x18\x07\x20\x01(\
    \x05R\x19pollingInactiveDelayMsecs\x120\n\x14force_gpu_compatible\x18\
    \x08\x20\x01(\x08R\x12forceGpuCompatible\"\xc2\x03\n\x10OptimizerOptions\
    \x12M\n#do_common_subexpression_elimination\x18\x01\x20\x01(\x08R\x20doC\
    ommonSubexpressionElimination\x12.\n\x13do_constant_folding\x18\x02\x20\
    \x01(\x08R\x11doConstantFolding\x120\n\x14do_function_inlining\x18\x04\
    \x20\x01(\x08R\x12doFunctionInlining\x12?\n\topt_level\x18\x03\x20\x01(\
    \x0e2\".tensorflow.OptimizerOptions.LevelR\x08optLevel\x12U\n\x10global_\
    jit_level\x18\x05\x20\x01(\x0e2+.tensorflow.OptimizerOptions.GlobalJitLe\
    velR\x0eglobalJitLevel\"\x20\n\x05Level\x12\x06\n\x02L1\x10\0\x12\x0f\n\
    \x02L0\x10\xff\xff\xff\xff\xff\xff\xff\xff\xff\x01\"C\n\x0eGlobalJitLeve\
    l\x12\x0b\n\x07DEFAULT\x10\0\x12\x10\n\x03OFF\x10\xff\xff\xff\xff\xff\
    \xff\xff\xff\xff\x01\x12\x08\n\x04ON_1\x10\x01\x12\x08\n\x04ON_2\x10\x02\
    \"\x90\x04\n\x0cGraphOptions\x124\n\x16enable_recv_scheduling\x18\x02\
    \x20\x01(\x08R\x14enableRecvScheduling\x12I\n\x11optimizer_options\x18\
    \x03\x20\x01(\x0b2\x1c.tensorflow.OptimizerOptionsR\x10optimizerOptions\
    \x12(\n\x10build_cost_model\x18\x04\x20\x01(\x03R\x0ebuildCostModel\x123\
    \n\x16build_cost_model_after\x18\t\x20\x01(\x03R\x13buildCostModelAfter\
    \x12!\n\x0cinfer_shapes\x18\x05\x20\x01(\x08R\x0binferShapes\x12,\n\x12p\
    lace_pruned_graph\x18\x06\x20\x01(\x08R\x10placePrunedGraph\x128\n\x18en\
    able_bfloat16_sendrecv\x18\x07\x20\x01(\x08R\x16enableBfloat16Sendrecv\
    \x12#\n\rtimeline_step\x18\x08\x20\x01(\x05R\x0ctimelineStep\x12C\n\x0fr\
    ewrite_options\x18\n\x20\x01(\x0b2\x1a.tensorflow.RewriterConfigR\x0erew\
    riteOptionsJ\x04\x08\x01\x10\x02R%skip_common_subexpression_elimination\
    \"Y\n\x15ThreadPoolOptionProto\x12\x1f\n\x0bnum_threads\x18\x01\x20\x01(\
    \x05R\nnumThreads\x12\x1f\n\x0bglobal_name\x18\x02\x20\x01(\tR\nglobalNa\
    me\"L\n\nRPCOptions\x12>\n\x1cuse_rpc_for_inprocess_master\x18\x01\x20\
    \x01(\x08R\x18useRpcForInprocessMaster\"\x8d\x07\n\x0bConfigProto\x12K\n\
    \x0cdevice_count\x18\x01\x20\x03(\x0b2(.tensorflow.ConfigProto.DeviceCou\
    ntEntryR\x0bdeviceCount\x12?\n\x1cintra_op_parallelism_threads\x18\x02\
    \x20\x01(\x05R\x19intraOpParallelismThreads\x12?\n\x1cinter_op_paralleli\
    sm_threads\x18\x05\x20\x01(\x05R\x19interOpParallelismThreads\x125\n\x17\
    use_per_session_threads\x18\t\x20\x01(\x08R\x14usePerSessionThreads\x12a\
    \n\x1csession_inter_op_thread_pool\x18\x0c\x20\x03(\x0b2!.tensorflow.Thr\
    eadPoolOptionProtoR\x18sessionInterOpThreadPool\x12)\n\x10placement_peri\
    od\x18\x03\x20\x01(\x05R\x0fplacementPeriod\x12%\n\x0edevice_filters\x18\
    \x04\x20\x03(\tR\rdeviceFilters\x127\n\x0bgpu_options\x18\x06\x20\x01(\
    \x0b2\x16.tensorflow.GPUOptionsR\ngpuOptions\x120\n\x14allow_soft_placem\
    ent\x18\x07\x20\x01(\x08R\x12allowSoftPlacement\x120\n\x14log_device_pla\
    cement\x18\x08\x20\x01(\x08R\x12logDevicePlacement\x12=\n\rgraph_options\
    \x18\n\x20\x01(\x0b2\x18.tensorflow.GraphOptionsR\x0cgraphOptions\x125\n\
    \x17operation_timeout_in_ms\x18\x0b\x20\x01(\x03R\x14operationTimeoutInM\
    s\x127\n\x0brpc_options\x18\r\x20\x01(\x0b2\x16.tensorflow.RPCOptionsR\n\
    rpcOptions\x127\n\x0bcluster_def\x18\x0e\x20\x01(\x0b2\x16.tensorflow.Cl\
    usterDefR\nclusterDef\x1a>\n\x10DeviceCountEntry\x12\x10\n\x03key\x18\
    \x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\x05R\x05valu\
    e:\x028\x01\"\xf6\x02\n\nRunOptions\x12B\n\x0btrace_level\x18\x01\x20\
    \x01(\x0e2!.tensorflow.RunOptions.TraceLevelR\ntraceLevel\x12\"\n\rtimeo\
    ut_in_ms\x18\x02\x20\x01(\x03R\x0btimeoutInMs\x12/\n\x14inter_op_thread_\
    pool\x18\x03\x20\x01(\x05R\x11interOpThreadPool\x126\n\x17output_partiti\
    on_graphs\x18\x05\x20\x01(\x08R\x15outputPartitionGraphs\x12=\n\rdebug_o\
    ptions\x18\x06\x20\x01(\x0b2\x18.tensorflow.DebugOptionsR\x0cdebugOption\
    s\"R\n\nTraceLevel\x12\x0c\n\x08NO_TRACE\x10\0\x12\x12\n\x0eSOFTWARE_TRA\
    CE\x10\x01\x12\x12\n\x0eHARDWARE_TRACE\x10\x02\x12\x0e\n\nFULL_TRACE\x10\
    \x03J\x04\x08\x04\x10\x05\"\xbd\x01\n\x0bRunMetadata\x124\n\nstep_stats\
    \x18\x01\x20\x01(\x0b2\x15.tensorflow.StepStatsR\tstepStats\x127\n\ncost\
    _graph\x18\x02\x20\x01(\x0b2\x18.tensorflow.CostGraphDefR\tcostGraph\x12\
    ?\n\x10partition_graphs\x18\x03\x20\x03(\x0b2\x14.tensorflow.GraphDefR\
    \x0fpartitionGraphsB-\n\x18org.tensorflow.frameworkB\x0cConfigProtosP\
    \x01\xf8\x01\x01J\xfcu\n\x07\x12\x05\0\0\xd5\x02\x01\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\n\x08\n\x01\x08\x12\x03\
    \x03\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\n\x0c\n\x05\x08\
    \xe7\x07\0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\
    \x03\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x03\x07\x17\n\
    \x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\x01\x08\x12\x03\
    \x04\0-\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\0-\n\x0c\n\x05\x08\xe7\
    \x07\x01\x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\
    \x04\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x04\x07\x1b\n\
    \x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e,\n\x08\n\x01\x08\x12\x03\
    \x05\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\x0c\n\x05\x08\xe7\
    \x07\x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\
    \x05\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x05\x07\x1a\n\
    \x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\n\x01\x08\x12\x03\
    \x06\01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\x0c\n\x05\x08\xe7\
    \x07\x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\
    \x06\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x06\x07\x13\n\
    \x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\t\n\x02\x03\0\x12\x03\
    \x08\x073\n\t\n\x02\x03\x01\x12\x03\t\x07.\n\t\n\x02\x03\x02\x12\x03\n\
    \x073\n\t\n\x02\x03\x03\x12\x03\x0b\x07-\n\t\n\x02\x03\x04\x12\x03\x0c\
    \x07/\n\t\n\x02\x03\x05\x12\x03\r\x077\n\n\n\x02\x04\0\x12\x04\x0f\0O\
    \x01\n\n\n\x03\x04\0\x01\x12\x03\x0f\x08\x12\n\xf7\x01\n\x04\x04\0\x02\0\
    \x12\x03\x14\x02-\x1a\xe9\x01\x20A\x20value\x20between\x200\x20and\x201\
    \x20that\x20indicates\x20what\x20fraction\x20of\x20the\n\x20available\
    \x20GPU\x20memory\x20to\x20pre-allocate\x20for\x20each\x20process.\x20\
    \x201\x20means\n\x20to\x20pre-allocate\x20all\x20of\x20the\x20GPU\x20mem\
    ory,\x200.5\x20means\x20the\x20process\n\x20allocates\x20~50%\x20of\x20t\
    he\x20available\x20GPU\x20memory.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\
    \x14\x02\x0f\x14\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x14\x02\x08\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\x14\t(\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\
    \x14+,\n\x8b\x02\n\x04\x04\0\x02\x01\x12\x03\x1e\x02\x1c\x1a\xfd\x01\x20\
    The\x20type\x20of\x20GPU\x20allocation\x20strategy\x20to\x20use.\n\n\x20\
    Allowed\x20values:\n\x20\"\":\x20The\x20empty\x20string\x20(default)\x20\
    uses\x20a\x20system-chosen\x20default\n\x20\x20\x20\x20\x20which\x20may\
    \x20change\x20over\x20time.\n\n\x20\"BFC\":\x20A\x20\"Best-fit\x20with\
    \x20coalescing\"\x20algorithm,\x20simplified\x20from\x20a\n\x20\x20\x20\
    \x20\x20\x20\x20\x20version\x20of\x20dlmalloc.\n\n\r\n\x05\x04\0\x02\x01\
    \x04\x12\x04\x1e\x02\x14-\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x1e\x02\
    \x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x1e\t\x17\n\x0c\n\x05\x04\0\
    \x02\x01\x03\x12\x03\x1e\x1a\x1b\n\xb2\x01\n\x04\x04\0\x02\x02\x12\x03#\
    \x02$\x1a\xa4\x01\x20Delay\x20deletion\x20of\x20up\x20to\x20this\x20many\
    \x20bytes\x20to\x20reduce\x20the\x20number\x20of\n\x20interactions\x20wi\
    th\x20gpu\x20driver\x20code.\x20\x20If\x200,\x20the\x20system\x20chooses\
    \n\x20a\x20reasonable\x20default\x20(several\x20MBs).\n\n\r\n\x05\x04\0\
    \x02\x02\x04\x12\x04#\x02\x1e\x1c\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03#\
    \x02\x07\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03#\x08\x1f\n\x0c\n\x05\x04\
    \0\x02\x02\x03\x12\x03#\"#\n\x93\x01\n\x04\x04\0\x02\x03\x12\x03'\x02\
    \x18\x1a\x85\x01\x20If\x20true,\x20the\x20allocator\x20does\x20not\x20pr\
    e-allocate\x20the\x20entire\x20specified\n\x20GPU\x20memory\x20region,\
    \x20instead\x20starting\x20small\x20and\x20growing\x20as\x20needed.\n\n\
    \r\n\x05\x04\0\x02\x03\x04\x12\x04'\x02#$\n\x0c\n\x05\x04\0\x02\x03\x05\
    \x12\x03'\x02\x06\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03'\x07\x13\n\x0c\n\
    \x05\x04\0\x02\x03\x03\x12\x03'\x16\x17\n\x96\x07\n\x04\x04\0\x02\x04\
    \x12\x038\x02!\x1a\x88\x07\x20A\x20comma-separated\x20list\x20of\x20GPU\
    \x20ids\x20that\x20determines\x20the\x20'visible'\n\x20to\x20'virtual'\
    \x20mapping\x20of\x20GPU\x20devices.\x20\x20For\x20example,\x20if\x20Ten\
    sorFlow\n\x20can\x20see\x208\x20GPU\x20devices\x20in\x20the\x20process,\
    \x20and\x20one\x20wanted\x20to\x20map\n\x20visible\x20GPU\x20devices\x20\
    5\x20and\x203\x20as\x20\"/device:GPU:0\",\x20and\x20\"/device:GPU:1\",\
    \x20then\x20one\n\x20would\x20specify\x20this\x20field\x20as\x20\"5,3\".\
    \x20\x20This\x20field\x20is\x20similar\x20in\n\x20spirit\x20to\x20the\
    \x20CUDA_VISIBLE_DEVICES\x20environment\x20variable,\x20except\n\x20it\
    \x20applies\x20to\x20the\x20visible\x20GPU\x20devices\x20in\x20the\x20pr\
    ocess.\n\n\x20NOTE:\x20The\x20GPU\x20driver\x20provides\x20the\x20proces\
    s\x20with\x20the\x20visible\x20GPUs\n\x20in\x20an\x20order\x20which\x20i\
    s\x20not\x20guaranteed\x20to\x20have\x20any\x20correlation\x20to\n\x20th\
    e\x20*physical*\x20GPU\x20id\x20in\x20the\x20machine.\x20\x20This\x20fie\
    ld\x20is\x20used\x20for\n\x20remapping\x20\"visible\"\x20to\x20\"virtual\
    \",\x20which\x20means\x20this\x20operates\x20only\n\x20after\x20the\x20p\
    rocess\x20starts.\x20\x20Users\x20are\x20required\x20to\x20use\x20vendor\
    \n\x20specific\x20mechanisms\x20(e.g.,\x20CUDA_VISIBLE_DEVICES)\x20to\
    \x20control\x20the\n\x20physical\x20to\x20visible\x20device\x20mapping\
    \x20prior\x20to\x20invoking\x20TensorFlow.\n\n\r\n\x05\x04\0\x02\x04\x04\
    \x12\x048\x02'\x18\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x038\x02\x08\n\x0c\
    \n\x05\x04\0\x02\x04\x01\x12\x038\t\x1c\n\x0c\n\x05\x04\0\x02\x04\x03\
    \x12\x038\x1f\x20\n\xc1\x01\n\x04\x04\0\x02\x05\x12\x03=\x02'\x1a\xb3\
    \x01\x20In\x20the\x20event\x20polling\x20loop\x20sleep\x20this\x20many\
    \x20microseconds\x20between\n\x20PollEvents\x20calls,\x20when\x20the\x20\
    queue\x20is\x20not\x20empty.\x20\x20If\x20value\x20is\x20not\n\x20set\
    \x20or\x20set\x20to\x200,\x20gets\x20set\x20to\x20a\x20non-zero\x20defau\
    lt.\n\n\r\n\x05\x04\0\x02\x05\x04\x12\x04=\x028!\n\x0c\n\x05\x04\0\x02\
    \x05\x05\x12\x03=\x02\x07\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03=\x08\"\n\
    \x0c\n\x05\x04\0\x02\x05\x03\x12\x03=%&\n\xbc\x01\n\x04\x04\0\x02\x06\
    \x12\x03B\x02)\x1a\xae\x01\x20In\x20the\x20event\x20polling\x20loop\x20s\
    leep\x20this\x20many\x20millisconds\x20between\n\x20PollEvents\x20calls,\
    \x20when\x20the\x20queue\x20is\x20empty.\x20\x20If\x20value\x20is\x20not\
    \n\x20set\x20or\x20set\x20to\x200,\x20gets\x20set\x20to\x20a\x20non-zero\
    \x20default.\n\n\r\n\x05\x04\0\x02\x06\x04\x12\x04B\x02='\n\x0c\n\x05\
    \x04\0\x02\x06\x05\x12\x03B\x02\x07\n\x0c\n\x05\x04\0\x02\x06\x01\x12\
    \x03B\x08$\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03B'(\n\x9a\x05\n\x04\x04\
    \0\x02\x07\x12\x03N\x02\x20\x1a\x8c\x05\x20Force\x20all\x20tensors\x20to\
    \x20be\x20gpu_compatible.\x20On\x20a\x20GPU-enabled\x20TensorFlow,\n\x20\
    enabling\x20this\x20option\x20forces\x20all\x20CPU\x20tensors\x20to\x20b\
    e\x20allocated\x20with\x20Cuda\n\x20pinned\x20memory.\x20Normally,\x20Te\
    nsorFlow\x20will\x20infer\x20which\x20tensors\x20should\x20be\n\x20alloc\
    ated\x20as\x20the\x20pinned\x20memory.\x20But\x20in\x20case\x20where\x20\
    the\x20inference\x20is\n\x20incomplete,\x20this\x20option\x20can\x20sign\
    ificantly\x20speed\x20up\x20the\x20cross-device\x20memory\n\x20copy\x20p\
    erformance\x20as\x20long\x20as\x20it\x20fits\x20the\x20memory.\n\x20Note\
    \x20that\x20this\x20option\x20is\x20not\x20something\x20that\x20should\
    \x20be\n\x20enabled\x20by\x20default\x20for\x20unknown\x20or\x20very\x20\
    large\x20models,\x20since\x20all\x20Cuda\x20pinned\n\x20memory\x20is\x20\
    unpageable,\x20having\x20too\x20much\x20pinned\x20memory\x20might\x20neg\
    atively\x20impact\n\x20the\x20overall\x20host\x20system\x20performance.\
    \n\n\r\n\x05\x04\0\x02\x07\x04\x12\x04N\x02B)\n\x0c\n\x05\x04\0\x02\x07\
    \x05\x12\x03N\x02\x06\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03N\x07\x1b\n\
    \x0c\n\x05\x04\0\x02\x07\x03\x12\x03N\x1e\x1f\n3\n\x02\x04\x01\x12\x04R\
    \0x\x01\x1a'\x20Options\x20passed\x20to\x20the\x20graph\x20optimizer\n\n\
    \n\n\x03\x04\x01\x01\x12\x03R\x08\x18\nR\n\x04\x04\x01\x02\0\x12\x03T\
    \x02/\x1aE\x20If\x20true,\x20optimize\x20the\x20graph\x20using\x20common\
    \x20subexpression\x20elimination.\n\n\r\n\x05\x04\x01\x02\0\x04\x12\x04T\
    \x02R\x1a\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03T\x02\x06\n\x0c\n\x05\x04\
    \x01\x02\0\x01\x12\x03T\x07*\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03T-.\nK\
    \n\x04\x04\x01\x02\x01\x12\x03W\x02\x1f\x1a>\x20If\x20true,\x20perform\
    \x20constant\x20folding\x20optimization\x20on\x20the\x20graph.\n\n\r\n\
    \x05\x04\x01\x02\x01\x04\x12\x04W\x02T/\n\x0c\n\x05\x04\x01\x02\x01\x05\
    \x12\x03W\x02\x06\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03W\x07\x1a\n\x0c\
    \n\x05\x04\x01\x02\x01\x03\x12\x03W\x1d\x1e\n?\n\x04\x04\x01\x02\x02\x12\
    \x03Z\x02\x20\x1a2\x20If\x20true,\x20perform\x20function\x20inlining\x20\
    on\x20the\x20graph.\n\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04Z\x02W\x1f\n\
    \x0c\n\x05\x04\x01\x02\x02\x05\x12\x03Z\x02\x06\n\x0c\n\x05\x04\x01\x02\
    \x02\x01\x12\x03Z\x07\x1b\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03Z\x1e\
    \x1f\n\"\n\x04\x04\x01\x04\0\x12\x04]\x02f\x03\x1a\x14\x20Optimization\
    \x20level\n\n\x0c\n\x05\x04\x01\x04\0\x01\x12\x03]\x07\x0c\n\x83\x01\n\
    \x06\x04\x01\x04\0\x02\0\x12\x03b\x04\x0b\x1at\x20L1\x20is\x20the\x20def\
    ault\x20level.\n\x20Optimization\x20performed\x20at\x20L1\x20:\n\x201.\
    \x20Common\x20subexpression\x20elimination\n\x202.\x20Constant\x20foldin\
    g\n\n\x0e\n\x07\x04\x01\x04\0\x02\0\x01\x12\x03b\x04\x06\n\x0e\n\x07\x04\
    \x01\x04\0\x02\0\x02\x12\x03b\t\n\n!\n\x06\x04\x01\x04\0\x02\x01\x12\x03\
    e\x04\x0c\x1a\x12\x20No\x20optimizations\n\n\x0e\n\x07\x04\x01\x04\0\x02\
    \x01\x01\x12\x03e\x04\x06\n\x0e\n\x07\x04\x01\x04\0\x02\x01\x02\x12\x03e\
    \t\x0b\n\xa4\x01\n\x04\x04\x01\x02\x03\x12\x03j\x02\x16\x1a\x96\x01\x20O\
    verall\x20optimization\x20level.\x20The\x20actual\x20optimizations\x20ap\
    plied\x20will\x20be\x20the\n\x20logical\x20OR\x20of\x20the\x20flags\x20t\
    hat\x20this\x20level\x20implies\x20and\x20any\x20flags\x20already\x20set\
    .\n\n\r\n\x05\x04\x01\x02\x03\x04\x12\x04j\x02f\x03\n\x0c\n\x05\x04\x01\
    \x02\x03\x06\x12\x03j\x02\x07\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03j\
    \x08\x11\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03j\x14\x15\nC\n\x04\x04\
    \x01\x04\x01\x12\x04m\x02v\x03\x1a5\x20Control\x20the\x20use\x20of\x20th\
    e\x20compiler/jit.\x20\x20Experimental.\n\n\x0c\n\x05\x04\x01\x04\x01\
    \x01\x12\x03m\x07\x15\nK\n\x06\x04\x01\x04\x01\x02\0\x12\x03n\x04\x10\"<\
    \x20Default\x20setting\x20(\"off\"\x20now,\x20but\x20later\x20expected\
    \x20to\x20be\x20\"on\")\n\n\x0e\n\x07\x04\x01\x04\x01\x02\0\x01\x12\x03n\
    \x04\x0b\n\x0e\n\x07\x04\x01\x04\x01\x02\0\x02\x12\x03n\x0e\x0f\n\r\n\
    \x06\x04\x01\x04\x01\x02\x01\x12\x03o\x04\r\n\x0e\n\x07\x04\x01\x04\x01\
    \x02\x01\x01\x12\x03o\x04\x07\n\x0e\n\x07\x04\x01\x04\x01\x02\x01\x02\
    \x12\x03o\n\x0c\n\x83\x02\n\x06\x04\x01\x04\x01\x02\x02\x12\x03t\x04\r\
    \x1a\xf3\x01\x20The\x20following\x20settings\x20turn\x20on\x20compilatio\
    n,\x20with\x20higher\x20values\x20being\n\x20more\x20aggressive.\x20\x20\
    Higher\x20values\x20may\x20reduce\x20opportunities\x20for\x20parallelism\
    \n\x20and\x20may\x20use\x20more\x20memory.\x20\x20(At\x20present,\x20the\
    re\x20is\x20no\x20distinction,\x20but\x20this\n\x20is\x20expected\x20to\
    \x20change.)\n\n\x0e\n\x07\x04\x01\x04\x01\x02\x02\x01\x12\x03t\x04\x08\
    \n\x0e\n\x07\x04\x01\x04\x01\x02\x02\x02\x12\x03t\x0b\x0c\n\r\n\x06\x04\
    \x01\x04\x01\x02\x03\x12\x03u\x04\r\n\x0e\n\x07\x04\x01\x04\x01\x02\x03\
    \x01\x12\x03u\x04\x08\n\x0e\n\x07\x04\x01\x04\x01\x02\x03\x02\x12\x03u\
    \x0b\x0c\n\x0b\n\x04\x04\x01\x02\x04\x12\x03w\x02&\n\r\n\x05\x04\x01\x02\
    \x04\x04\x12\x04w\x02v\x03\n\x0c\n\x05\x04\x01\x02\x04\x06\x12\x03w\x02\
    \x10\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03w\x11!\n\x0c\n\x05\x04\x01\
    \x02\x04\x03\x12\x03w$%\n\x0b\n\x02\x04\x02\x12\x05z\0\xa7\x01\x01\n\n\n\
    \x03\x04\x02\x01\x12\x03z\x08\x14\n3\n\x03\x04\x02\n\x12\x03|\x0b3\x1a'\
    \x20Removed,\x20use\x20optimizer_options\x20below.\n\n\x0b\n\x04\x04\x02\
    \n\0\x12\x03|\x0b2\n\n\n\x03\x04\x02\t\x12\x03}\x0b\r\n\x0b\n\x04\x04\
    \x02\t\0\x12\x03}\x0b\x0c\n\x0c\n\x05\x04\x02\t\0\x01\x12\x03}\x0b\x0c\n\
    \x0c\n\x05\x04\x02\t\0\x02\x12\x03}\x0b\x0c\ni\n\x04\x04\x02\x02\0\x12\
    \x04\x81\x01\x02\"\x1a[\x20If\x20true,\x20use\x20control\x20flow\x20to\
    \x20schedule\x20the\x20activation\x20of\x20Recv\x20nodes.\n\x20(Currentl\
    y\x20ignored.)\n\n\x0e\n\x05\x04\x02\x02\0\x04\x12\x05\x81\x01\x02}\r\n\
    \r\n\x05\x04\x02\x02\0\x05\x12\x04\x81\x01\x02\x06\n\r\n\x05\x04\x02\x02\
    \0\x01\x12\x04\x81\x01\x07\x1d\n\r\n\x05\x04\x02\x02\0\x03\x12\x04\x81\
    \x01\x20!\n;\n\x04\x04\x02\x02\x01\x12\x04\x84\x01\x02)\x1a-\x20Options\
    \x20controlling\x20how\x20graph\x20is\x20optimized.\n\n\x0f\n\x05\x04\
    \x02\x02\x01\x04\x12\x06\x84\x01\x02\x81\x01\"\n\r\n\x05\x04\x02\x02\x01\
    \x06\x12\x04\x84\x01\x02\x12\n\r\n\x05\x04\x02\x02\x01\x01\x12\x04\x84\
    \x01\x13$\n\r\n\x05\x04\x02\x02\x01\x03\x12\x04\x84\x01'(\n\xa8\x01\n\
    \x04\x04\x02\x02\x02\x12\x04\x89\x01\x02\x1d\x1a\x99\x01\x20The\x20numbe\
    r\x20of\x20steps\x20to\x20run\x20before\x20returning\x20a\x20cost\x20mod\
    el\x20detailing\n\x20the\x20memory\x20usage\x20and\x20performance\x20of\
    \x20each\x20node\x20of\x20the\x20graph.\x200\x20means\n\x20no\x20cost\
    \x20model.\n\n\x0f\n\x05\x04\x02\x02\x02\x04\x12\x06\x89\x01\x02\x84\x01\
    )\n\r\n\x05\x04\x02\x02\x02\x05\x12\x04\x89\x01\x02\x07\n\r\n\x05\x04\
    \x02\x02\x02\x01\x12\x04\x89\x01\x08\x18\n\r\n\x05\x04\x02\x02\x02\x03\
    \x12\x04\x89\x01\x1b\x1c\n]\n\x04\x04\x02\x02\x03\x12\x04\x8d\x01\x02#\
    \x1aO\x20The\x20number\x20of\x20steps\x20to\x20skip\x20before\x20collect\
    ing\x20statistics\x20for\x20the\n\x20cost\x20model.\n\n\x0f\n\x05\x04\
    \x02\x02\x03\x04\x12\x06\x8d\x01\x02\x89\x01\x1d\n\r\n\x05\x04\x02\x02\
    \x03\x05\x12\x04\x8d\x01\x02\x07\n\r\n\x05\x04\x02\x02\x03\x01\x12\x04\
    \x8d\x01\x08\x1e\n\r\n\x05\x04\x02\x02\x03\x03\x12\x04\x8d\x01!\"\nk\n\
    \x04\x04\x02\x02\x04\x12\x04\x91\x01\x02\x18\x1a]\x20Annotate\x20each\
    \x20Node\x20with\x20Op\x20output\x20shape\x20data,\x20to\x20the\x20exten\
    t\x20it\x20can\n\x20be\x20statically\x20inferred.\n\n\x0f\n\x05\x04\x02\
    \x02\x04\x04\x12\x06\x91\x01\x02\x8d\x01#\n\r\n\x05\x04\x02\x02\x04\x05\
    \x12\x04\x91\x01\x02\x06\n\r\n\x05\x04\x02\x02\x04\x01\x12\x04\x91\x01\
    \x07\x13\n\r\n\x05\x04\x02\x02\x04\x03\x12\x04\x91\x01\x16\x17\n\xee\x02\
    \n\x04\x04\x02\x02\x05\x12\x04\x9a\x01\x02\x1e\x1a\xdf\x02\x20Only\x20pl\
    ace\x20the\x20subgraphs\x20that\x20are\x20run,\x20rather\x20than\x20the\
    \x20entire\x20graph.\n\n\x20This\x20is\x20useful\x20for\x20interactive\
    \x20graph\x20building,\x20where\x20one\x20might\n\x20produce\x20graphs\
    \x20that\x20cannot\x20be\x20placed\x20during\x20the\x20debugging\n\x20pr\
    ocess.\x20\x20In\x20particular,\x20it\x20allows\x20the\x20client\x20to\
    \x20continue\x20work\x20in\n\x20a\x20session\x20after\x20adding\x20a\x20\
    node\x20to\x20a\x20graph\x20whose\x20placement\n\x20constraints\x20are\
    \x20unsatisfiable.\n\n\x0f\n\x05\x04\x02\x02\x05\x04\x12\x06\x9a\x01\x02\
    \x91\x01\x18\n\r\n\x05\x04\x02\x02\x05\x05\x12\x04\x9a\x01\x02\x06\n\r\n\
    \x05\x04\x02\x02\x05\x01\x12\x04\x9a\x01\x07\x19\n\r\n\x05\x04\x02\x02\
    \x05\x03\x12\x04\x9a\x01\x1c\x1d\nM\n\x04\x04\x02\x02\x06\x12\x04\x9d\
    \x01\x02$\x1a?\x20If\x20true,\x20transfer\x20float\x20values\x20between\
    \x20processes\x20as\x20bfloat16.\n\n\x0f\n\x05\x04\x02\x02\x06\x04\x12\
    \x06\x9d\x01\x02\x9a\x01\x1e\n\r\n\x05\x04\x02\x02\x06\x05\x12\x04\x9d\
    \x01\x02\x06\n\r\n\x05\x04\x02\x02\x06\x01\x12\x04\x9d\x01\x07\x1f\n\r\n\
    \x05\x04\x02\x02\x06\x03\x12\x04\x9d\x01\"#\n~\n\x04\x04\x02\x02\x07\x12\
    \x04\xa1\x01\x02\x1a\x1ap\x20If\x20>\x200,\x20record\x20a\x20timeline\
    \x20every\x20this\x20many\x20steps.\n\x20EXPERIMENTAL:\x20This\x20curren\
    tly\x20has\x20no\x20effect\x20in\x20MasterSession.\n\n\x0f\n\x05\x04\x02\
    \x02\x07\x04\x12\x06\xa1\x01\x02\x9d\x01$\n\r\n\x05\x04\x02\x02\x07\x05\
    \x12\x04\xa1\x01\x02\x07\n\r\n\x05\x04\x02\x02\x07\x01\x12\x04\xa1\x01\
    \x08\x15\n\r\n\x05\x04\x02\x02\x07\x03\x12\x04\xa1\x01\x18\x19\n\xd8\x01\
    \n\x04\x04\x02\x02\x08\x12\x04\xa6\x01\x02&\x1a\xc9\x01\x20Options\x20th\
    at\x20control\x20the\x20type\x20and\x20amount\x20of\x20graph\x20rewritin\
    g.\n\x20Not\x20currently\x20configurable\x20via\x20the\x20public\x20Pyth\
    on\x20API\x20(i.e.\x20there\x20is\x20no\x20API\n\x20stability\x20guarant\
    ee\x20if\x20you\x20import\x20RewriterConfig\x20explicitly).\n\n\x0f\n\
    \x05\x04\x02\x02\x08\x04\x12\x06\xa6\x01\x02\xa1\x01\x1a\n\r\n\x05\x04\
    \x02\x02\x08\x06\x12\x04\xa6\x01\x02\x10\n\r\n\x05\x04\x02\x02\x08\x01\
    \x12\x04\xa6\x01\x11\x20\n\r\n\x05\x04\x02\x02\x08\x03\x12\x04\xa6\x01#%\
    \n\x0c\n\x02\x04\x03\x12\x06\xa9\x01\0\xc0\x01\x01\n\x0b\n\x03\x04\x03\
    \x01\x12\x04\xa9\x01\x08\x1d\n\xbb\x01\n\x04\x04\x03\x02\0\x12\x04\xae\
    \x01\x02\x18\x1a\xac\x01\x20The\x20number\x20of\x20threads\x20in\x20the\
    \x20pool.\n\n\x200\x20means\x20the\x20system\x20picks\x20a\x20value\x20b\
    ased\x20on\x20where\x20this\x20option\x20proto\x20is\x20used\n\x20(see\
    \x20the\x20declaration\x20of\x20the\x20specific\x20field\x20for\x20more\
    \x20info).\n\n\x0f\n\x05\x04\x03\x02\0\x04\x12\x06\xae\x01\x02\xa9\x01\
    \x1f\n\r\n\x05\x04\x03\x02\0\x05\x12\x04\xae\x01\x02\x07\n\r\n\x05\x04\
    \x03\x02\0\x01\x12\x04\xae\x01\x08\x13\n\r\n\x05\x04\x03\x02\0\x03\x12\
    \x04\xae\x01\x16\x17\n\xf7\x05\n\x04\x04\x03\x02\x01\x12\x04\xbf\x01\x02\
    \x19\x1a\xe8\x05\x20The\x20global\x20name\x20of\x20the\x20threadpool.\n\
    \n\x20If\x20empty,\x20then\x20the\x20threadpool\x20is\x20made\x20and\x20\
    used\x20according\x20to\x20the\x20scope\x20it's\n\x20in\x20-\x20e.g.,\
    \x20for\x20a\x20session\x20threadpool,\x20it\x20is\x20used\x20by\x20that\
    \x20session\x20only.\n\n\x20If\x20non-empty,\x20then:\n\x20-\x20a\x20glo\
    bal\x20threadpool\x20associated\x20with\x20this\x20name\x20is\x20looked\
    \n\x20\x20\x20up\x20or\x20created.\x20This\x20allows,\x20for\x20example,\
    \x20sharing\x20one\x20threadpool\x20across\n\x20\x20\x20many\x20sessions\
    \x20(e.g.,\x20like\x20the\x20default\x20behavior,\x20if\n\x20\x20\x20int\
    er_op_parallelism_threads\x20is\x20not\x20configured),\x20but\x20still\
    \x20partitioning\n\x20\x20\x20into\x20a\x20large\x20and\x20small\x20pool\
    .\n\x20-\x20if\x20the\x20threadpool\x20for\x20this\x20global_name\x20alr\
    eady\x20exists,\x20then\x20it\x20is\x20an\n\x20\x20\x20error\x20if\x20th\
    e\x20existing\x20pool\x20was\x20created\x20using\x20a\x20different\x20nu\
    m_threads\n\x20\x20\x20value\x20as\x20is\x20specified\x20on\x20this\x20c\
    all.\n\x20-\x20threadpools\x20created\x20this\x20way\x20are\x20never\x20\
    garbage\x20collected.\n\n\x0f\n\x05\x04\x03\x02\x01\x04\x12\x06\xbf\x01\
    \x02\xae\x01\x18\n\r\n\x05\x04\x03\x02\x01\x05\x12\x04\xbf\x01\x02\x08\n\
    \r\n\x05\x04\x03\x02\x01\x01\x12\x04\xbf\x01\t\x14\n\r\n\x05\x04\x03\x02\
    \x01\x03\x12\x04\xbf\x01\x17\x18\n\x0c\n\x02\x04\x04\x12\x06\xc2\x01\0\
    \xc9\x01\x01\n\x0b\n\x03\x04\x04\x01\x12\x04\xc2\x01\x08\x12\n\x88\x02\n\
    \x04\x04\x04\x02\0\x12\x04\xc8\x01\x02(\x1a\xf9\x01\x20If\x20true,\x20al\
    ways\x20use\x20RPC\x20to\x20contact\x20the\x20session\x20target.\n\n\x20\
    If\x20false\x20(the\x20default\x20option),\x20TensorFlow\x20may\x20use\
    \x20an\x20optimized\n\x20transport\x20for\x20client-master\x20communicat\
    ion\x20that\x20avoids\x20the\x20RPC\n\x20stack.\x20This\x20option\x20is\
    \x20primarily\x20for\x20used\x20testing\x20the\x20RPC\x20stack.\n\n\x0f\
    \n\x05\x04\x04\x02\0\x04\x12\x06\xc8\x01\x02\xc2\x01\x14\n\r\n\x05\x04\
    \x04\x02\0\x05\x12\x04\xc8\x01\x02\x06\n\r\n\x05\x04\x04\x02\0\x01\x12\
    \x04\xc8\x01\x07#\n\r\n\x05\x04\x04\x02\0\x03\x12\x04\xc8\x01&'\ns\n\x02\
    \x04\x05\x12\x06\xcd\x01\0\xab\x02\x01\x1ae\x20Session\x20configuration\
    \x20parameters.\n\x20The\x20system\x20picks\x20appropriate\x20values\x20\
    for\x20fields\x20that\x20are\x20not\x20set.\n\n\x0b\n\x03\x04\x05\x01\
    \x12\x04\xcd\x01\x08\x13\n\xd5\x01\n\x04\x04\x05\x02\0\x12\x04\xd2\x01\
    \x02&\x1a\xc6\x01\x20Map\x20from\x20device\x20type\x20name\x20(e.g.,\x20\
    \"CPU\"\x20or\x20\"GPU\"\x20)\x20to\x20maximum\n\x20number\x20of\x20devi\
    ces\x20of\x20that\x20type\x20to\x20use.\x20\x20If\x20a\x20particular\x20\
    device\n\x20type\x20is\x20not\x20found\x20in\x20the\x20map,\x20the\x20sy\
    stem\x20picks\x20an\x20appropriate\n\x20number.\n\n\x0f\n\x05\x04\x05\
    \x02\0\x04\x12\x06\xd2\x01\x02\xcd\x01\x15\n\r\n\x05\x04\x05\x02\0\x06\
    \x12\x04\xd2\x01\x02\x14\n\r\n\x05\x04\x05\x02\0\x01\x12\x04\xd2\x01\x15\
    !\n\r\n\x05\x04\x05\x02\0\x03\x12\x04\xd2\x01$%\n\xb7\x01\n\x04\x04\x05\
    \x02\x01\x12\x04\xd7\x01\x02)\x1a\xa8\x01\x20The\x20execution\x20of\x20a\
    n\x20individual\x20op\x20(for\x20some\x20op\x20types)\x20can\x20be\n\x20\
    parallelized\x20on\x20a\x20pool\x20of\x20intra_op_parallelism_threads.\n\
    \x200\x20means\x20the\x20system\x20picks\x20an\x20appropriate\x20number.\
    \n\n\x0f\n\x05\x04\x05\x02\x01\x04\x12\x06\xd7\x01\x02\xd2\x01&\n\r\n\
    \x05\x04\x05\x02\x01\x05\x12\x04\xd7\x01\x02\x07\n\r\n\x05\x04\x05\x02\
    \x01\x01\x12\x04\xd7\x01\x08$\n\r\n\x05\x04\x05\x02\x01\x03\x12\x04\xd7\
    \x01'(\n\xfc\x02\n\x04\x04\x05\x02\x02\x12\x04\xe1\x01\x02)\x1a\xed\x02\
    \x20Nodes\x20that\x20perform\x20blocking\x20operations\x20are\x20enqueue\
    d\x20on\x20a\x20pool\x20of\n\x20inter_op_parallelism_threads\x20availabl\
    e\x20in\x20each\x20process.\n\n\x200\x20means\x20the\x20system\x20picks\
    \x20an\x20appropriate\x20number.\n\n\x20Note\x20that\x20the\x20first\x20\
    Session\x20created\x20in\x20the\x20process\x20sets\x20the\n\x20number\
    \x20of\x20threads\x20for\x20all\x20future\x20sessions\x20unless\x20use_p\
    er_session_threads\x20is\n\x20true\x20or\x20session_inter_op_thread_pool\
    \x20is\x20configured.\n\n\x0f\n\x05\x04\x05\x02\x02\x04\x12\x06\xe1\x01\
    \x02\xd7\x01)\n\r\n\x05\x04\x05\x02\x02\x05\x12\x04\xe1\x01\x02\x07\n\r\
    \n\x05\x04\x05\x02\x02\x01\x12\x04\xe1\x01\x08$\n\r\n\x05\x04\x05\x02\
    \x02\x03\x12\x04\xe1\x01'(\n\xd0\x03\n\x04\x04\x05\x02\x03\x12\x04\xec\
    \x01\x02#\x1a\xc1\x03\x20If\x20true,\x20use\x20a\x20new\x20set\x20of\x20\
    threads\x20for\x20this\x20session\x20rather\x20than\x20the\x20global\n\
    \x20pool\x20of\x20threads.\x20Only\x20supported\x20by\x20direct\x20sessi\
    ons.\n\n\x20If\x20false,\x20use\x20the\x20global\x20threads\x20created\
    \x20by\x20the\x20first\x20session,\x20or\x20the\n\x20per-session\x20thre\
    ad\x20pools\x20configured\x20by\x20session_inter_op_thread_pool.\n\n\x20\
    This\x20option\x20is\x20deprecated.\x20The\x20same\x20effect\x20can\x20b\
    e\x20achieved\x20by\x20setting\n\x20session_inter_op_thread_pool\x20to\
    \x20have\x20one\x20element,\x20whose\x20num_threads\x20equals\n\x20inter\
    _op_parallelism_threads.\n\n\x0f\n\x05\x04\x05\x02\x03\x04\x12\x06\xec\
    \x01\x02\xe1\x01)\n\r\n\x05\x04\x05\x02\x03\x05\x12\x04\xec\x01\x02\x06\
    \n\r\n\x05\x04\x05\x02\x03\x01\x12\x04\xec\x01\x07\x1e\n\r\n\x05\x04\x05\
    \x02\x03\x03\x12\x04\xec\x01!\"\n\x91\x08\n\x04\x04\x05\x02\x04\x12\x04\
    \x81\x02\x02C\x1a\x82\x08\x20This\x20option\x20is\x20experimental\x20-\
    \x20it\x20may\x20be\x20replaced\x20with\x20a\x20different\x20mechanism\n\
    \x20in\x20the\x20future.\n\n\x20Configures\x20session\x20thread\x20pools\
    .\x20If\x20this\x20is\x20configured,\x20then\x20RunOptions\x20for\n\x20a\
    \x20Run\x20call\x20can\x20select\x20the\x20thread\x20pool\x20to\x20use.\
    \n\n\x20The\x20intended\x20use\x20is\x20for\x20when\x20some\x20session\
    \x20invocations\x20need\x20to\x20run\x20in\x20a\n\x20background\x20pool\
    \x20limited\x20to\x20a\x20small\x20number\x20of\x20threads:\n\x20-\x20Fo\
    r\x20example,\x20a\x20session\x20may\x20be\x20configured\x20to\x20have\
    \x20one\x20large\x20pool\x20(for\n\x20regular\x20compute)\x20and\x20one\
    \x20small\x20pool\x20(for\x20periodic,\x20low\x20priority\x20work);\n\
    \x20using\x20the\x20small\x20pool\x20is\x20currently\x20the\x20mechanism\
    \x20for\x20limiting\x20the\x20inter-op\n\x20parallelism\x20of\x20the\x20\
    low\x20priority\x20work.\x20\x20Note\x20that\x20it\x20does\x20not\x20lim\
    it\x20the\n\x20parallelism\x20of\x20work\x20spawned\x20by\x20a\x20single\
    \x20op\x20kernel\x20implementation.\n\x20-\x20Using\x20this\x20setting\
    \x20is\x20normally\x20not\x20needed\x20in\x20training,\x20but\x20may\x20\
    help\x20some\n\x20serving\x20use\x20cases.\n\x20-\x20It\x20is\x20also\
    \x20generally\x20recommended\x20to\x20set\x20the\x20global_name\x20field\
    \x20of\x20this\n\x20proto,\x20to\x20avoid\x20creating\x20multiple\x20lar\
    ge\x20pools.\x20It\x20is\x20typically\x20better\x20to\n\x20run\x20the\
    \x20non-low-priority\x20work,\x20even\x20across\x20sessions,\x20in\x20a\
    \x20single\x20large\n\x20pool.\n\n\r\n\x05\x04\x05\x02\x04\x04\x12\x04\
    \x81\x02\x02\n\n\r\n\x05\x04\x05\x02\x04\x06\x12\x04\x81\x02\x0b\x20\n\r\
    \n\x05\x04\x05\x02\x04\x01\x12\x04\x81\x02!=\n\r\n\x05\x04\x05\x02\x04\
    \x03\x12\x04\x81\x02@B\n\xbd\x01\n\x04\x04\x05\x02\x05\x12\x04\x86\x02\
    \x02\x1d\x1a\xae\x01\x20Assignment\x20of\x20Nodes\x20to\x20Devices\x20is\
    \x20recomputed\x20every\x20placement_period\n\x20steps\x20until\x20the\
    \x20system\x20warms\x20up\x20(at\x20which\x20point\x20the\x20recomputati\
    on\n\x20typically\x20slows\x20down\x20automatically).\n\n\x0f\n\x05\x04\
    \x05\x02\x05\x04\x12\x06\x86\x02\x02\x81\x02C\n\r\n\x05\x04\x05\x02\x05\
    \x05\x12\x04\x86\x02\x02\x07\n\r\n\x05\x04\x05\x02\x05\x01\x12\x04\x86\
    \x02\x08\x18\n\r\n\x05\x04\x05\x02\x05\x03\x12\x04\x86\x02\x1b\x1c\n\xc5\
    \x01\n\x04\x04\x05\x02\x06\x12\x04\x8b\x02\x02%\x1a\xb6\x01\x20When\x20a\
    ny\x20filters\x20are\x20present\x20sessions\x20will\x20ignore\x20all\x20\
    devices\x20which\x20do\x20not\n\x20match\x20the\x20filters.\x20Each\x20f\
    ilter\x20can\x20be\x20partially\x20specified,\x20e.g.\x20\"/job:ps\"\n\
    \x20\"/job:worker/replica:3\",\x20etc.\n\n\r\n\x05\x04\x05\x02\x06\x04\
    \x12\x04\x8b\x02\x02\n\n\r\n\x05\x04\x05\x02\x06\x05\x12\x04\x8b\x02\x0b\
    \x11\n\r\n\x05\x04\x05\x02\x06\x01\x12\x04\x8b\x02\x12\x20\n\r\n\x05\x04\
    \x05\x02\x06\x03\x12\x04\x8b\x02#$\n/\n\x04\x04\x05\x02\x07\x12\x04\x8e\
    \x02\x02\x1d\x1a!\x20Options\x20that\x20apply\x20to\x20all\x20GPUs.\n\n\
    \x0f\n\x05\x04\x05\x02\x07\x04\x12\x06\x8e\x02\x02\x8b\x02%\n\r\n\x05\
    \x04\x05\x02\x07\x06\x12\x04\x8e\x02\x02\x0c\n\r\n\x05\x04\x05\x02\x07\
    \x01\x12\x04\x8e\x02\r\x18\n\r\n\x05\x04\x05\x02\x07\x03\x12\x04\x8e\x02\
    \x1b\x1c\n\x9a\x02\n\x04\x04\x05\x02\x08\x12\x04\x97\x02\x02\x20\x1a\x8b\
    \x02\x20Whether\x20soft\x20placement\x20is\x20allowed.\x20If\x20allow_so\
    ft_placement\x20is\x20true,\n\x20an\x20op\x20will\x20be\x20placed\x20on\
    \x20CPU\x20if\n\x20\x20\x201.\x20there's\x20no\x20GPU\x20implementation\
    \x20for\x20the\x20OP\n\x20or\n\x20\x20\x202.\x20no\x20GPU\x20devices\x20\
    are\x20known\x20or\x20registered\n\x20or\n\x20\x20\x203.\x20need\x20to\
    \x20co-locate\x20with\x20reftype\x20input(s)\x20which\x20are\x20from\x20\
    CPU.\n\n\x0f\n\x05\x04\x05\x02\x08\x04\x12\x06\x97\x02\x02\x8e\x02\x1d\n\
    \r\n\x05\x04\x05\x02\x08\x05\x12\x04\x97\x02\x02\x06\n\r\n\x05\x04\x05\
    \x02\x08\x01\x12\x04\x97\x02\x07\x1b\n\r\n\x05\x04\x05\x02\x08\x03\x12\
    \x04\x97\x02\x1e\x1f\n;\n\x04\x04\x05\x02\t\x12\x04\x9a\x02\x02\x20\x1a-\
    \x20Whether\x20device\x20placements\x20should\x20be\x20logged.\n\n\x0f\n\
    \x05\x04\x05\x02\t\x04\x12\x06\x9a\x02\x02\x97\x02\x20\n\r\n\x05\x04\x05\
    \x02\t\x05\x12\x04\x9a\x02\x02\x06\n\r\n\x05\x04\x05\x02\t\x01\x12\x04\
    \x9a\x02\x07\x1b\n\r\n\x05\x04\x05\x02\t\x03\x12\x04\x9a\x02\x1e\x1f\n1\
    \n\x04\x04\x05\x02\n\x12\x04\x9d\x02\x02\"\x1a#\x20Options\x20that\x20ap\
    ply\x20to\x20all\x20graphs.\n\n\x0f\n\x05\x04\x05\x02\n\x04\x12\x06\x9d\
    \x02\x02\x9a\x02\x20\n\r\n\x05\x04\x05\x02\n\x06\x12\x04\x9d\x02\x02\x0e\
    \n\r\n\x05\x04\x05\x02\n\x01\x12\x04\x9d\x02\x0f\x1c\n\r\n\x05\x04\x05\
    \x02\n\x03\x12\x04\x9d\x02\x1f!\n\xce\x01\n\x04\x04\x05\x02\x0b\x12\x04\
    \xa2\x02\x02%\x1a\xbf\x01\x20Global\x20timeout\x20for\x20all\x20blocking\
    \x20operations\x20in\x20this\x20session.\x20\x20If\x20non-zero,\n\x20and\
    \x20not\x20overridden\x20on\x20a\x20per-operation\x20basis,\x20this\x20v\
    alue\x20will\x20be\x20used\x20as\x20the\n\x20deadline\x20for\x20all\x20b\
    locking\x20operations.\n\n\x0f\n\x05\x04\x05\x02\x0b\x04\x12\x06\xa2\x02\
    \x02\x9d\x02\"\n\r\n\x05\x04\x05\x02\x0b\x05\x12\x04\xa2\x02\x02\x07\n\r\
    \n\x05\x04\x05\x02\x0b\x01\x12\x04\xa2\x02\x08\x1f\n\r\n\x05\x04\x05\x02\
    \x0b\x03\x12\x04\xa2\x02\"$\nR\n\x04\x04\x05\x02\x0c\x12\x04\xa5\x02\x02\
    \x1e\x1aD\x20Options\x20that\x20apply\x20when\x20this\x20session\x20uses\
    \x20the\x20distributed\x20runtime.\n\n\x0f\n\x05\x04\x05\x02\x0c\x04\x12\
    \x06\xa5\x02\x02\xa2\x02%\n\r\n\x05\x04\x05\x02\x0c\x06\x12\x04\xa5\x02\
    \x02\x0c\n\r\n\x05\x04\x05\x02\x0c\x01\x12\x04\xa5\x02\r\x18\n\r\n\x05\
    \x04\x05\x02\x0c\x03\x12\x04\xa5\x02\x1b\x1d\nD\n\x04\x04\x05\x02\r\x12\
    \x04\xa8\x02\x02\x1e\x1a6\x20Optional\x20list\x20of\x20all\x20workers\
    \x20to\x20use\x20in\x20this\x20session.\n\n\x0f\n\x05\x04\x05\x02\r\x04\
    \x12\x06\xa8\x02\x02\xa5\x02\x1e\n\r\n\x05\x04\x05\x02\r\x06\x12\x04\xa8\
    \x02\x02\x0c\n\r\n\x05\x04\x05\x02\r\x01\x12\x04\xa8\x02\r\x18\n\r\n\x05\
    \x04\x05\x02\r\x03\x12\x04\xa8\x02\x1b\x1d\n0\n\x02\x04\x06\x12\x06\xae\
    \x02\0\xc7\x02\x01\x1a\"\x20Options\x20for\x20a\x20single\x20Run()\x20ca\
    ll.\n\n\x0b\n\x03\x04\x06\x01\x12\x04\xae\x02\x08\x12\n\x84\x01\n\x04\
    \x04\x06\x04\0\x12\x06\xb1\x02\x02\xb6\x02\x03\x1at\x20TODO(pbar)\x20Tur\
    n\x20this\x20into\x20a\x20TraceOptions\x20proto\x20which\x20allows\n\x20\
    tracing\x20to\x20be\x20controlled\x20in\x20a\x20more\x20orthogonal\x20ma\
    nner?\n\n\r\n\x05\x04\x06\x04\0\x01\x12\x04\xb1\x02\x07\x11\n\x0e\n\x06\
    \x04\x06\x04\0\x02\0\x12\x04\xb2\x02\x04\x11\n\x0f\n\x07\x04\x06\x04\0\
    \x02\0\x01\x12\x04\xb2\x02\x04\x0c\n\x0f\n\x07\x04\x06\x04\0\x02\0\x02\
    \x12\x04\xb2\x02\x0f\x10\n\x0e\n\x06\x04\x06\x04\0\x02\x01\x12\x04\xb3\
    \x02\x04\x17\n\x0f\n\x07\x04\x06\x04\0\x02\x01\x01\x12\x04\xb3\x02\x04\
    \x12\n\x0f\n\x07\x04\x06\x04\0\x02\x01\x02\x12\x04\xb3\x02\x15\x16\n\x0e\
    \n\x06\x04\x06\x04\0\x02\x02\x12\x04\xb4\x02\x04\x17\n\x0f\n\x07\x04\x06\
    \x04\0\x02\x02\x01\x12\x04\xb4\x02\x04\x12\n\x0f\n\x07\x04\x06\x04\0\x02\
    \x02\x02\x12\x04\xb4\x02\x15\x16\n\x0e\n\x06\x04\x06\x04\0\x02\x03\x12\
    \x04\xb5\x02\x04\x13\n\x0f\n\x07\x04\x06\x04\0\x02\x03\x01\x12\x04\xb5\
    \x02\x04\x0e\n\x0f\n\x07\x04\x06\x04\0\x02\x03\x02\x12\x04\xb5\x02\x11\
    \x12\n\x0c\n\x04\x04\x06\x02\0\x12\x04\xb7\x02\x02\x1d\n\x0f\n\x05\x04\
    \x06\x02\0\x04\x12\x06\xb7\x02\x02\xb6\x02\x03\n\r\n\x05\x04\x06\x02\0\
    \x06\x12\x04\xb7\x02\x02\x0c\n\r\n\x05\x04\x06\x02\0\x01\x12\x04\xb7\x02\
    \r\x18\n\r\n\x05\x04\x06\x02\0\x03\x12\x04\xb7\x02\x1b\x1c\nG\n\x04\x04\
    \x06\x02\x01\x12\x04\xba\x02\x02\x1a\x1a9\x20Time\x20to\x20wait\x20for\
    \x20operation\x20to\x20complete\x20in\x20milliseconds.\n\n\x0f\n\x05\x04\
    \x06\x02\x01\x04\x12\x06\xba\x02\x02\xb7\x02\x1d\n\r\n\x05\x04\x06\x02\
    \x01\x05\x12\x04\xba\x02\x02\x07\n\r\n\x05\x04\x06\x02\x01\x01\x12\x04\
    \xba\x02\x08\x15\n\r\n\x05\x04\x06\x02\x01\x03\x12\x04\xba\x02\x18\x19\n\
    V\n\x04\x04\x06\x02\x02\x12\x04\xbd\x02\x02!\x1aH\x20The\x20thread\x20po\
    ol\x20to\x20use,\x20if\x20session_inter_op_thread_pool\x20is\x20configur\
    ed.\n\n\x0f\n\x05\x04\x06\x02\x02\x04\x12\x06\xbd\x02\x02\xba\x02\x1a\n\
    \r\n\x05\x04\x06\x02\x02\x05\x12\x04\xbd\x02\x02\x07\n\r\n\x05\x04\x06\
    \x02\x02\x01\x12\x04\xbd\x02\x08\x1c\n\r\n\x05\x04\x06\x02\x02\x03\x12\
    \x04\xbd\x02\x1f\x20\np\n\x04\x04\x06\x02\x03\x12\x04\xc1\x02\x02#\x1ab\
    \x20Whether\x20the\x20partition\x20graph(s)\x20executed\x20by\x20the\x20\
    executor(s)\x20should\x20be\n\x20outputted\x20via\x20RunMetadata.\n\n\
    \x0f\n\x05\x04\x06\x02\x03\x04\x12\x06\xc1\x02\x02\xbd\x02!\n\r\n\x05\
    \x04\x06\x02\x03\x05\x12\x04\xc1\x02\x02\x06\n\r\n\x05\x04\x06\x02\x03\
    \x01\x12\x04\xc1\x02\x07\x1e\n\r\n\x05\x04\x06\x02\x03\x03\x12\x04\xc1\
    \x02!\"\nT\n\x04\x04\x06\x02\x04\x12\x04\xc4\x02\x02!\x1aF\x20EXPERIMENT\
    AL.\x20\x20Options\x20used\x20to\x20initialize\x20DebuggerState,\x20if\
    \x20enabled.\n\n\x0f\n\x05\x04\x06\x02\x04\x04\x12\x06\xc4\x02\x02\xc1\
    \x02#\n\r\n\x05\x04\x06\x02\x04\x06\x12\x04\xc4\x02\x02\x0e\n\r\n\x05\
    \x04\x06\x02\x04\x01\x12\x04\xc4\x02\x0f\x1c\n\r\n\x05\x04\x06\x02\x04\
    \x03\x12\x04\xc4\x02\x1f\x20\n\x0b\n\x03\x04\x06\t\x12\x04\xc6\x02\x0b\r\
    \n\x0c\n\x04\x04\x06\t\0\x12\x04\xc6\x02\x0b\x0c\n\r\n\x05\x04\x06\t\0\
    \x01\x12\x04\xc6\x02\x0b\x0c\n\r\n\x05\x04\x06\t\0\x02\x12\x04\xc6\x02\
    \x0b\x0c\nK\n\x02\x04\x07\x12\x06\xca\x02\0\xd5\x02\x01\x1a=\x20Metadata\
    \x20output\x20(i.e.,\x20non-Tensor)\x20for\x20a\x20single\x20Run()\x20ca\
    ll.\n\n\x0b\n\x03\x04\x07\x01\x12\x04\xca\x02\x08\x13\n\xbb\x01\n\x04\
    \x04\x07\x02\0\x12\x04\xce\x02\x02\x1b\x1a\xac\x01\x20Statistics\x20trac\
    ed\x20for\x20this\x20step.\x20Populated\x20if\x20tracing\x20is\x20turned\
    \x20on\x20via\x20the\n\x20\"RunOptions\"\x20proto.\n\x20EXPERIMENTAL:\
    \x20The\x20format\x20and\x20set\x20of\x20events\x20may\x20change\x20in\
    \x20future\x20versions.\n\n\x0f\n\x05\x04\x07\x02\0\x04\x12\x06\xce\x02\
    \x02\xca\x02\x15\n\r\n\x05\x04\x07\x02\0\x06\x12\x04\xce\x02\x02\x0b\n\r\
    \n\x05\x04\x07\x02\0\x01\x12\x04\xce\x02\x0c\x16\n\r\n\x05\x04\x07\x02\0\
    \x03\x12\x04\xce\x02\x19\x1a\nK\n\x04\x04\x07\x02\x01\x12\x04\xd1\x02\
    \x02\x1e\x1a=\x20The\x20cost\x20graph\x20for\x20the\x20computation\x20de\
    fined\x20by\x20the\x20run\x20call.\n\n\x0f\n\x05\x04\x07\x02\x01\x04\x12\
    \x06\xd1\x02\x02\xce\x02\x1b\n\r\n\x05\x04\x07\x02\x01\x06\x12\x04\xd1\
    \x02\x02\x0e\n\r\n\x05\x04\x07\x02\x01\x01\x12\x04\xd1\x02\x0f\x19\n\r\n\
    \x05\x04\x07\x02\x01\x03\x12\x04\xd1\x02\x1c\x1d\n?\n\x04\x04\x07\x02\
    \x02\x12\x04\xd4\x02\x02)\x1a1\x20Graphs\x20of\x20the\x20partitions\x20e\
    xecuted\x20by\x20executors.\n\n\r\n\x05\x04\x07\x02\x02\x04\x12\x04\xd4\
    \x02\x02\n\n\r\n\x05\x04\x07\x02\x02\x06\x12\x04\xd4\x02\x0b\x13\n\r\n\
    \x05\x04\x07\x02\x02\x01\x12\x04\xd4\x02\x14$\n\r\n\x05\x04\x07\x02\x02\
    \x03\x12\x04\xd4\x02'(b\x06proto3\
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
