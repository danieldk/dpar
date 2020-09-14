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
pub struct AutoParallelOptions {
    // message fields
    pub enable: bool,
    pub num_replicas: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AutoParallelOptions {}

impl AutoParallelOptions {
    pub fn new() -> AutoParallelOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AutoParallelOptions {
        static mut instance: ::protobuf::lazy::Lazy<AutoParallelOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AutoParallelOptions,
        };
        unsafe {
            instance.get(AutoParallelOptions::new)
        }
    }

    // bool enable = 1;

    pub fn clear_enable(&mut self) {
        self.enable = false;
    }

    // Param is passed by value, moved
    pub fn set_enable(&mut self, v: bool) {
        self.enable = v;
    }

    pub fn get_enable(&self) -> bool {
        self.enable
    }

    fn get_enable_for_reflect(&self) -> &bool {
        &self.enable
    }

    fn mut_enable_for_reflect(&mut self) -> &mut bool {
        &mut self.enable
    }

    // int32 num_replicas = 2;

    pub fn clear_num_replicas(&mut self) {
        self.num_replicas = 0;
    }

    // Param is passed by value, moved
    pub fn set_num_replicas(&mut self, v: i32) {
        self.num_replicas = v;
    }

    pub fn get_num_replicas(&self) -> i32 {
        self.num_replicas
    }

    fn get_num_replicas_for_reflect(&self) -> &i32 {
        &self.num_replicas
    }

    fn mut_num_replicas_for_reflect(&mut self) -> &mut i32 {
        &mut self.num_replicas
    }
}

impl ::protobuf::Message for AutoParallelOptions {
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
                    self.enable = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.num_replicas = tmp;
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
        if self.enable != false {
            my_size += 2;
        }
        if self.num_replicas != 0 {
            my_size += ::protobuf::rt::value_size(2, self.num_replicas, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.enable != false {
            os.write_bool(1, self.enable)?;
        }
        if self.num_replicas != 0 {
            os.write_int32(2, self.num_replicas)?;
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

impl ::protobuf::MessageStatic for AutoParallelOptions {
    fn new() -> AutoParallelOptions {
        AutoParallelOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<AutoParallelOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "enable",
                    AutoParallelOptions::get_enable_for_reflect,
                    AutoParallelOptions::mut_enable_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_replicas",
                    AutoParallelOptions::get_num_replicas_for_reflect,
                    AutoParallelOptions::mut_num_replicas_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AutoParallelOptions>(
                    "AutoParallelOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AutoParallelOptions {
    fn clear(&mut self) {
        self.clear_enable();
        self.clear_num_replicas();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AutoParallelOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AutoParallelOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RewriterConfig {
    // message fields
    pub optimize_tensor_layout: bool,
    pub constant_folding: RewriterConfig_Toggle,
    pub arithmetic_optimization: RewriterConfig_Toggle,
    pub disable_model_pruning: bool,
    pub memory_optimization: RewriterConfig_MemOptType,
    pub memory_optimizer_target_node_name_prefix: ::std::string::String,
    pub auto_parallel: ::protobuf::SingularPtrField<AutoParallelOptions>,
    pub optimizers: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RewriterConfig {}

impl RewriterConfig {
    pub fn new() -> RewriterConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RewriterConfig {
        static mut instance: ::protobuf::lazy::Lazy<RewriterConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RewriterConfig,
        };
        unsafe {
            instance.get(RewriterConfig::new)
        }
    }

    // bool optimize_tensor_layout = 1;

    pub fn clear_optimize_tensor_layout(&mut self) {
        self.optimize_tensor_layout = false;
    }

    // Param is passed by value, moved
    pub fn set_optimize_tensor_layout(&mut self, v: bool) {
        self.optimize_tensor_layout = v;
    }

    pub fn get_optimize_tensor_layout(&self) -> bool {
        self.optimize_tensor_layout
    }

    fn get_optimize_tensor_layout_for_reflect(&self) -> &bool {
        &self.optimize_tensor_layout
    }

    fn mut_optimize_tensor_layout_for_reflect(&mut self) -> &mut bool {
        &mut self.optimize_tensor_layout
    }

    // .tensorflow.RewriterConfig.Toggle constant_folding = 3;

    pub fn clear_constant_folding(&mut self) {
        self.constant_folding = RewriterConfig_Toggle::DEFAULT;
    }

    // Param is passed by value, moved
    pub fn set_constant_folding(&mut self, v: RewriterConfig_Toggle) {
        self.constant_folding = v;
    }

    pub fn get_constant_folding(&self) -> RewriterConfig_Toggle {
        self.constant_folding
    }

    fn get_constant_folding_for_reflect(&self) -> &RewriterConfig_Toggle {
        &self.constant_folding
    }

    fn mut_constant_folding_for_reflect(&mut self) -> &mut RewriterConfig_Toggle {
        &mut self.constant_folding
    }

    // .tensorflow.RewriterConfig.Toggle arithmetic_optimization = 7;

    pub fn clear_arithmetic_optimization(&mut self) {
        self.arithmetic_optimization = RewriterConfig_Toggle::DEFAULT;
    }

    // Param is passed by value, moved
    pub fn set_arithmetic_optimization(&mut self, v: RewriterConfig_Toggle) {
        self.arithmetic_optimization = v;
    }

    pub fn get_arithmetic_optimization(&self) -> RewriterConfig_Toggle {
        self.arithmetic_optimization
    }

    fn get_arithmetic_optimization_for_reflect(&self) -> &RewriterConfig_Toggle {
        &self.arithmetic_optimization
    }

    fn mut_arithmetic_optimization_for_reflect(&mut self) -> &mut RewriterConfig_Toggle {
        &mut self.arithmetic_optimization
    }

    // bool disable_model_pruning = 2;

    pub fn clear_disable_model_pruning(&mut self) {
        self.disable_model_pruning = false;
    }

    // Param is passed by value, moved
    pub fn set_disable_model_pruning(&mut self, v: bool) {
        self.disable_model_pruning = v;
    }

    pub fn get_disable_model_pruning(&self) -> bool {
        self.disable_model_pruning
    }

    fn get_disable_model_pruning_for_reflect(&self) -> &bool {
        &self.disable_model_pruning
    }

    fn mut_disable_model_pruning_for_reflect(&mut self) -> &mut bool {
        &mut self.disable_model_pruning
    }

    // .tensorflow.RewriterConfig.MemOptType memory_optimization = 4;

    pub fn clear_memory_optimization(&mut self) {
        self.memory_optimization = RewriterConfig_MemOptType::DEFAULT_MEM_OPT;
    }

    // Param is passed by value, moved
    pub fn set_memory_optimization(&mut self, v: RewriterConfig_MemOptType) {
        self.memory_optimization = v;
    }

    pub fn get_memory_optimization(&self) -> RewriterConfig_MemOptType {
        self.memory_optimization
    }

    fn get_memory_optimization_for_reflect(&self) -> &RewriterConfig_MemOptType {
        &self.memory_optimization
    }

    fn mut_memory_optimization_for_reflect(&mut self) -> &mut RewriterConfig_MemOptType {
        &mut self.memory_optimization
    }

    // string memory_optimizer_target_node_name_prefix = 6;

    pub fn clear_memory_optimizer_target_node_name_prefix(&mut self) {
        self.memory_optimizer_target_node_name_prefix.clear();
    }

    // Param is passed by value, moved
    pub fn set_memory_optimizer_target_node_name_prefix(&mut self, v: ::std::string::String) {
        self.memory_optimizer_target_node_name_prefix = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_memory_optimizer_target_node_name_prefix(&mut self) -> &mut ::std::string::String {
        &mut self.memory_optimizer_target_node_name_prefix
    }

    // Take field
    pub fn take_memory_optimizer_target_node_name_prefix(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.memory_optimizer_target_node_name_prefix, ::std::string::String::new())
    }

    pub fn get_memory_optimizer_target_node_name_prefix(&self) -> &str {
        &self.memory_optimizer_target_node_name_prefix
    }

    fn get_memory_optimizer_target_node_name_prefix_for_reflect(&self) -> &::std::string::String {
        &self.memory_optimizer_target_node_name_prefix
    }

    fn mut_memory_optimizer_target_node_name_prefix_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.memory_optimizer_target_node_name_prefix
    }

    // .tensorflow.AutoParallelOptions auto_parallel = 5;

    pub fn clear_auto_parallel(&mut self) {
        self.auto_parallel.clear();
    }

    pub fn has_auto_parallel(&self) -> bool {
        self.auto_parallel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_auto_parallel(&mut self, v: AutoParallelOptions) {
        self.auto_parallel = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_auto_parallel(&mut self) -> &mut AutoParallelOptions {
        if self.auto_parallel.is_none() {
            self.auto_parallel.set_default();
        }
        self.auto_parallel.as_mut().unwrap()
    }

    // Take field
    pub fn take_auto_parallel(&mut self) -> AutoParallelOptions {
        self.auto_parallel.take().unwrap_or_else(|| AutoParallelOptions::new())
    }

    pub fn get_auto_parallel(&self) -> &AutoParallelOptions {
        self.auto_parallel.as_ref().unwrap_or_else(|| AutoParallelOptions::default_instance())
    }

    fn get_auto_parallel_for_reflect(&self) -> &::protobuf::SingularPtrField<AutoParallelOptions> {
        &self.auto_parallel
    }

    fn mut_auto_parallel_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AutoParallelOptions> {
        &mut self.auto_parallel
    }

    // repeated string optimizers = 100;

    pub fn clear_optimizers(&mut self) {
        self.optimizers.clear();
    }

    // Param is passed by value, moved
    pub fn set_optimizers(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.optimizers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_optimizers(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.optimizers
    }

    // Take field
    pub fn take_optimizers(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.optimizers, ::protobuf::RepeatedField::new())
    }

    pub fn get_optimizers(&self) -> &[::std::string::String] {
        &self.optimizers
    }

    fn get_optimizers_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.optimizers
    }

    fn mut_optimizers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.optimizers
    }
}

impl ::protobuf::Message for RewriterConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.auto_parallel {
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
                    let tmp = is.read_bool()?;
                    self.optimize_tensor_layout = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.constant_folding = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.arithmetic_optimization = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.disable_model_pruning = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.memory_optimization = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.memory_optimizer_target_node_name_prefix)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.auto_parallel)?;
                },
                100 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.optimizers)?;
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
        if self.optimize_tensor_layout != false {
            my_size += 2;
        }
        if self.constant_folding != RewriterConfig_Toggle::DEFAULT {
            my_size += ::protobuf::rt::enum_size(3, self.constant_folding);
        }
        if self.arithmetic_optimization != RewriterConfig_Toggle::DEFAULT {
            my_size += ::protobuf::rt::enum_size(7, self.arithmetic_optimization);
        }
        if self.disable_model_pruning != false {
            my_size += 2;
        }
        if self.memory_optimization != RewriterConfig_MemOptType::DEFAULT_MEM_OPT {
            my_size += ::protobuf::rt::enum_size(4, self.memory_optimization);
        }
        if !self.memory_optimizer_target_node_name_prefix.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.memory_optimizer_target_node_name_prefix);
        }
        if let Some(ref v) = self.auto_parallel.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.optimizers {
            my_size += ::protobuf::rt::string_size(100, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.optimize_tensor_layout != false {
            os.write_bool(1, self.optimize_tensor_layout)?;
        }
        if self.constant_folding != RewriterConfig_Toggle::DEFAULT {
            os.write_enum(3, self.constant_folding.value())?;
        }
        if self.arithmetic_optimization != RewriterConfig_Toggle::DEFAULT {
            os.write_enum(7, self.arithmetic_optimization.value())?;
        }
        if self.disable_model_pruning != false {
            os.write_bool(2, self.disable_model_pruning)?;
        }
        if self.memory_optimization != RewriterConfig_MemOptType::DEFAULT_MEM_OPT {
            os.write_enum(4, self.memory_optimization.value())?;
        }
        if !self.memory_optimizer_target_node_name_prefix.is_empty() {
            os.write_string(6, &self.memory_optimizer_target_node_name_prefix)?;
        }
        if let Some(ref v) = self.auto_parallel.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.optimizers {
            os.write_string(100, &v)?;
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

impl ::protobuf::MessageStatic for RewriterConfig {
    fn new() -> RewriterConfig {
        RewriterConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<RewriterConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "optimize_tensor_layout",
                    RewriterConfig::get_optimize_tensor_layout_for_reflect,
                    RewriterConfig::mut_optimize_tensor_layout_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RewriterConfig_Toggle>>(
                    "constant_folding",
                    RewriterConfig::get_constant_folding_for_reflect,
                    RewriterConfig::mut_constant_folding_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RewriterConfig_Toggle>>(
                    "arithmetic_optimization",
                    RewriterConfig::get_arithmetic_optimization_for_reflect,
                    RewriterConfig::mut_arithmetic_optimization_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "disable_model_pruning",
                    RewriterConfig::get_disable_model_pruning_for_reflect,
                    RewriterConfig::mut_disable_model_pruning_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RewriterConfig_MemOptType>>(
                    "memory_optimization",
                    RewriterConfig::get_memory_optimization_for_reflect,
                    RewriterConfig::mut_memory_optimization_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "memory_optimizer_target_node_name_prefix",
                    RewriterConfig::get_memory_optimizer_target_node_name_prefix_for_reflect,
                    RewriterConfig::mut_memory_optimizer_target_node_name_prefix_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AutoParallelOptions>>(
                    "auto_parallel",
                    RewriterConfig::get_auto_parallel_for_reflect,
                    RewriterConfig::mut_auto_parallel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "optimizers",
                    RewriterConfig::get_optimizers_for_reflect,
                    RewriterConfig::mut_optimizers_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RewriterConfig>(
                    "RewriterConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RewriterConfig {
    fn clear(&mut self) {
        self.clear_optimize_tensor_layout();
        self.clear_constant_folding();
        self.clear_arithmetic_optimization();
        self.clear_disable_model_pruning();
        self.clear_memory_optimization();
        self.clear_memory_optimizer_target_node_name_prefix();
        self.clear_auto_parallel();
        self.clear_optimizers();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RewriterConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RewriterConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RewriterConfig_Toggle {
    DEFAULT = 0,
    ON = 1,
    OFF = 2,
}

impl ::protobuf::ProtobufEnum for RewriterConfig_Toggle {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RewriterConfig_Toggle> {
        match value {
            0 => ::std::option::Option::Some(RewriterConfig_Toggle::DEFAULT),
            1 => ::std::option::Option::Some(RewriterConfig_Toggle::ON),
            2 => ::std::option::Option::Some(RewriterConfig_Toggle::OFF),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RewriterConfig_Toggle] = &[
            RewriterConfig_Toggle::DEFAULT,
            RewriterConfig_Toggle::ON,
            RewriterConfig_Toggle::OFF,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RewriterConfig_Toggle>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RewriterConfig_Toggle", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RewriterConfig_Toggle {
}

impl ::std::default::Default for RewriterConfig_Toggle {
    fn default() -> Self {
        RewriterConfig_Toggle::DEFAULT
    }
}

impl ::protobuf::reflect::ProtobufValue for RewriterConfig_Toggle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RewriterConfig_MemOptType {
    DEFAULT_MEM_OPT = 0,
    NO_MEM_OPT = 1,
    MANUAL = 2,
    HEURISTICS = 3,
}

impl ::protobuf::ProtobufEnum for RewriterConfig_MemOptType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RewriterConfig_MemOptType> {
        match value {
            0 => ::std::option::Option::Some(RewriterConfig_MemOptType::DEFAULT_MEM_OPT),
            1 => ::std::option::Option::Some(RewriterConfig_MemOptType::NO_MEM_OPT),
            2 => ::std::option::Option::Some(RewriterConfig_MemOptType::MANUAL),
            3 => ::std::option::Option::Some(RewriterConfig_MemOptType::HEURISTICS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RewriterConfig_MemOptType] = &[
            RewriterConfig_MemOptType::DEFAULT_MEM_OPT,
            RewriterConfig_MemOptType::NO_MEM_OPT,
            RewriterConfig_MemOptType::MANUAL,
            RewriterConfig_MemOptType::HEURISTICS,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RewriterConfig_MemOptType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RewriterConfig_MemOptType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RewriterConfig_MemOptType {
}

impl ::std::default::Default for RewriterConfig_MemOptType {
    fn default() -> Self {
        RewriterConfig_MemOptType::DEFAULT_MEM_OPT
    }
}

impl ::protobuf::reflect::ProtobufValue for RewriterConfig_MemOptType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n.tensorflow/core/protobuf/rewriter_config.proto\x12\ntensorflow\"P\n\
    \x13AutoParallelOptions\x12\x16\n\x06enable\x18\x01\x20\x01(\x08R\x06ena\
    ble\x12!\n\x0cnum_replicas\x18\x02\x20\x01(\x05R\x0bnumReplicas\"\xb0\
    \x05\n\x0eRewriterConfig\x124\n\x16optimize_tensor_layout\x18\x01\x20\
    \x01(\x08R\x14optimizeTensorLayout\x12L\n\x10constant_folding\x18\x03\
    \x20\x01(\x0e2!.tensorflow.RewriterConfig.ToggleR\x0fconstantFolding\x12\
    Z\n\x17arithmetic_optimization\x18\x07\x20\x01(\x0e2!.tensorflow.Rewrite\
    rConfig.ToggleR\x16arithmeticOptimization\x122\n\x15disable_model_prunin\
    g\x18\x02\x20\x01(\x08R\x13disableModelPruning\x12V\n\x13memory_optimiza\
    tion\x18\x04\x20\x01(\x0e2%.tensorflow.RewriterConfig.MemOptTypeR\x12mem\
    oryOptimization\x12U\n(memory_optimizer_target_node_name_prefix\x18\x06\
    \x20\x01(\tR#memoryOptimizerTargetNodeNamePrefix\x12D\n\rauto_parallel\
    \x18\x05\x20\x01(\x0b2\x1f.tensorflow.AutoParallelOptionsR\x0cautoParall\
    el\x12\x1e\n\noptimizers\x18d\x20\x03(\tR\noptimizers\"&\n\x06Toggle\x12\
    \x0b\n\x07DEFAULT\x10\0\x12\x06\n\x02ON\x10\x01\x12\x07\n\x03OFF\x10\x02\
    \"M\n\nMemOptType\x12\x13\n\x0fDEFAULT_MEM_OPT\x10\0\x12\x0e\n\nNO_MEM_O\
    PT\x10\x01\x12\n\n\x06MANUAL\x10\x02\x12\x0e\n\nHEURISTICS\x10\x03B5\n\
    \x18org.tensorflow.frameworkB\x14RewriterConfigProtosP\x01\xf8\x01\x01J\
    \x96\x1d\n\x06\x12\x04\0\0N\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\
    \x01\x02\x12\x03\x02\x08\x12\n\x08\n\x01\x08\x12\x03\x03\0\x1f\n\x0b\n\
    \x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\
    \x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x03\x07\x17\n\x0e\
    \n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x03\x07\x17\n\x0c\n\x05\x08\xe7\
    \x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\x01\x08\x12\x03\x04\05\n\x0b\n\
    \x04\x08\xe7\x07\x01\x12\x03\x04\05\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\
    \x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x04\x07\x1b\n\
    \x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x04\x07\x1b\n\x0c\n\x05\x08\
    \xe7\x07\x01\x07\x12\x03\x04\x1e4\n\x08\n\x01\x08\x12\x03\x05\0\"\n\x0b\
    \n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\x0c\n\x05\x08\xe7\x07\x02\x02\
    \x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x05\x07\x1a\
    \n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x05\x07\x1a\n\x0c\n\x05\
    \x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\n\x01\x08\x12\x03\x06\01\n\
    \x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\x0c\n\x05\x08\xe7\x07\x03\
    \x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x06\x07\
    \x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x06\x07\x13\n\x0c\n\
    \x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\n\n\x02\x04\0\x12\x04\x08\0\
    \x0b\x01\n\n\n\x03\x04\0\x01\x12\x03\x08\x08\x1b\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\t\x02\x12\n\r\n\x05\x04\0\x02\0\x04\x12\x04\t\x02\x08\x1d\n\x0c\
    \n\x05\x04\0\x02\0\x05\x12\x03\t\x02\x06\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\t\x07\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\t\x10\x11\n\x0b\n\x04\
    \x04\0\x02\x01\x12\x03\n\x02\x19\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\n\
    \x02\t\x12\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\n\x02\x07\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\n\x08\x14\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\n\x17\x18\nr\n\x02\x04\x01\x12\x04\r\0N\x01\"f\x20Graph\x20rewritin\
    g\x20is\x20experimental\x20and\x20subject\x20to\x20change,\x20not\x20cov\
    ered\x20by\x20any\n\x20API\x20stability\x20guarantees.\n\n\n\n\x03\x04\
    \x01\x01\x12\x03\r\x08\x16\n\xc6\x01\n\x04\x04\x01\x04\0\x12\x04\x15\x02\
    \x19\x032\xb7\x01\x20Configuration\x20options\x20for\x20the\x20meta-opti\
    mizer.\x20Unless\x20otherwise\x20noted,\x20these\n\x20configuration\x20o\
    ptions\x20do\x20not\x20apply\x20to\x20explicitly\x20triggered\x20optimiz\
    ation\n\x20passes\x20in\x20the\x20optimizers\x20field.\n\n\x0c\n\x05\x04\
    \x01\x04\0\x01\x12\x03\x15\x07\r\n\r\n\x06\x04\x01\x04\0\x02\0\x12\x03\
    \x16\x04\x10\n\x0e\n\x07\x04\x01\x04\0\x02\0\x01\x12\x03\x16\x04\x0b\n\
    \x0e\n\x07\x04\x01\x04\0\x02\0\x02\x12\x03\x16\x0e\x0f\n\r\n\x06\x04\x01\
    \x04\0\x02\x01\x12\x03\x17\x04\x0b\n\x0e\n\x07\x04\x01\x04\0\x02\x01\x01\
    \x12\x03\x17\x04\x06\n\x0e\n\x07\x04\x01\x04\0\x02\x01\x02\x12\x03\x17\t\
    \n\n\r\n\x06\x04\x01\x04\0\x02\x02\x12\x03\x18\x04\x0c\n\x0e\n\x07\x04\
    \x01\x04\0\x02\x02\x01\x12\x03\x18\x04\x07\n\x0e\n\x07\x04\x01\x04\0\x02\
    \x02\x02\x12\x03\x18\n\x0b\n&\n\x04\x04\x01\x02\0\x12\x03\x1c\x02\"\x1a\
    \x19\x20Optimize\x20tensor\x20layouts\n\n\r\n\x05\x04\x01\x02\0\x04\x12\
    \x04\x1c\x02\x19\x03\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x1c\x02\x06\n\
    \x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x1c\x07\x1d\n\x0c\n\x05\x04\x01\x02\
    \0\x03\x12\x03\x1c\x20!\n-\n\x04\x04\x01\x02\x01\x12\x03\x1e\x02\x1e\x1a\
    \x20\x20Fold\x20constants\x20(default\x20is\x20ON)\n\n\r\n\x05\x04\x01\
    \x02\x01\x04\x12\x04\x1e\x02\x1c\"\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\
    \x03\x1e\x02\x08\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x1e\t\x19\n\x0c\
    \n\x05\x04\x01\x02\x01\x03\x12\x03\x1e\x1c\x1d\n7\n\x04\x04\x01\x02\x02\
    \x12\x03\x20\x02%\x1a*\x20Arithmetic\x20optimizations\x20(default\x20is\
    \x20ON)\n\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04\x20\x02\x1e\x1e\n\x0c\n\
    \x05\x04\x01\x02\x02\x06\x12\x03\x20\x02\x08\n\x0c\n\x05\x04\x01\x02\x02\
    \x01\x12\x03\x20\t\x20\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x20#$\nC\
    \n\x04\x04\x01\x02\x03\x12\x03\"\x02!\x1a6\x20If\x20true,\x20don't\x20re\
    move\x20unnecessary\x20ops\x20from\x20the\x20graph\n\n\r\n\x05\x04\x01\
    \x02\x03\x04\x12\x04\"\x02\x20%\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\
    \"\x02\x06\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\"\x07\x1c\n\x0c\n\x05\
    \x04\x01\x02\x03\x03\x12\x03\"\x1f\x20\n\x0c\n\x04\x04\x01\x04\x01\x12\
    \x04$\x020\x03\n\x0c\n\x05\x04\x01\x04\x01\x01\x12\x03$\x07\x11\n9\n\x06\
    \x04\x01\x04\x01\x02\0\x12\x03&\x04\x18\x1a*\x20The\x20default\x20settin\
    g\x20(currently\x20disabled)\n\n\x0e\n\x07\x04\x01\x04\x01\x02\0\x01\x12\
    \x03&\x04\x13\n\x0e\n\x07\x04\x01\x04\x01\x02\0\x02\x12\x03&\x16\x17\n0\
    \n\x06\x04\x01\x04\x01\x02\x01\x12\x03(\x04\x13\x1a!\x20Disabled\x20in\
    \x20the\x20meta-optimizer.\n\n\x0e\n\x07\x04\x01\x04\x01\x02\x01\x01\x12\
    \x03(\x04\x0e\n\x0e\n\x07\x04\x01\x04\x01\x02\x01\x02\x12\x03(\x11\x12\n\
    7\n\x06\x04\x01\x04\x01\x02\x02\x12\x03*\x04\x0f\x1a(\x20Driven\x20by\
    \x20manual\x20op-level\x20annotations.\n\n\x0e\n\x07\x04\x01\x04\x01\x02\
    \x02\x01\x12\x03*\x04\n\n\x0e\n\x07\x04\x01\x04\x01\x02\x02\x02\x12\x03*\
    \r\x0e\n\xf0\x01\n\x06\x04\x01\x04\x01\x02\x03\x12\x03/\x04\x13\x1a\xe0\
    \x01\x20Driven\x20by\x20heuristics.\x20The\x20behavior\x20of\x20these\
    \x20heuristics\x20is\x20subject\x20to\n\x20change.\x20Currently\x20inclu\
    des\x20an\x20experimental\x20recomputation\n\x20heuristic.\x20Manual\x20\
    annotations\x20are\x20respected,\x20but\x20additional\x20nodes\x20are\n\
    \x20selected\x20automatically.\n\n\x0e\n\x07\x04\x01\x04\x01\x02\x03\x01\
    \x12\x03/\x04\x0e\n\x0e\n\x07\x04\x01\x04\x01\x02\x03\x02\x12\x03/\x11\
    \x12\n\xab\x01\n\x04\x04\x01\x02\x04\x12\x034\x02%\x1a\x9d\x01\x20Config\
    ures\x20memory\x20optimization\x20passes\x20through\x20the\x20meta-optim\
    izer.\x20Has\x20no\n\x20effect\x20on\x20manually\x20requested\x20memory\
    \x20optimization\x20passes\x20in\x20the\x20optimizers\n\x20field.\n\n\r\
    \n\x05\x04\x01\x02\x04\x04\x12\x044\x020\x03\n\x0c\n\x05\x04\x01\x02\x04\
    \x06\x12\x034\x02\x0c\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x034\r\x20\n\
    \x0c\n\x05\x04\x01\x02\x04\x03\x12\x034#$\n\xf4\x04\n\x04\x04\x01\x02\
    \x05\x12\x03>\x026\x1a\xe6\x04\x20The\x20prefix\x20for\x20nodes\x20which\
    \x20are\x20valid\x20outputs\x20of\x20recomputations.\x20Inputs\x20to\n\
    \x20nodes\x20with\x20this\x20name\x20prefix\x20may\x20be\x20recomputed\
    \x20(subject\x20either\x20to\x20manual\n\x20annotation\x20of\x20those\
    \x20input\x20nodes\x20or\x20to\x20manual\x20annotation\x20and\x20heurist\
    ics\n\x20depending\x20on\x20memory_optimization),\x20but\x20the\x20prefi\
    xed\x20nodes\x20themselves\x20will\n\x20not\x20be\x20recomputed.\x20Typi\
    cally\x20this\x20will\x20be\x20\"gradients/\",\x20indicating\x20that\n\
    \x20activations\x20from\x20the\x20forward\x20pass\x20of\x20a\x20graph\
    \x20may\x20be\x20recomputed\x20as\x20inputs\x20to\n\x20gradients,\x20but\
    \x20may\x20be\x20adjusted\x20if\x20gradients\x20are\x20inside\x20a\x20na\
    me\x20scope\x20or\x20if\n\x20inputs\x20to\x20non-gradients\x20should\x20\
    be\x20recomputed.\x20Defaults\x20to\x20\"gradients/\"\x20if\n\x20empty\
    \x20or\x20not\x20set.\n\n\r\n\x05\x04\x01\x02\x05\x04\x12\x04>\x024%\n\
    \x0c\n\x05\x04\x01\x02\x05\x05\x12\x03>\x02\x08\n\x0c\n\x05\x04\x01\x02\
    \x05\x01\x12\x03>\t1\n\x0c\n\x05\x04\x01\x02\x05\x03\x12\x03>45\n\x97\
    \x01\n\x04\x04\x01\x02\x06\x12\x03B\x02(\x1a\x89\x01\x20Configures\x20Au\
    toParallel\x20optimization\x20passes\x20either\x20through\x20the\n\x20me\
    ta-optimizer\x20or\x20when\x20manually\x20specified\x20through\x20the\
    \x20optimizers\x20field.\n\n\r\n\x05\x04\x01\x02\x06\x04\x12\x04B\x02>6\
    \n\x0c\n\x05\x04\x01\x02\x06\x06\x12\x03B\x02\x15\n\x0c\n\x05\x04\x01\
    \x02\x06\x01\x12\x03B\x16#\n\x0c\n\x05\x04\x01\x02\x06\x03\x12\x03B&'\n\
    \x97\x04\n\x04\x04\x01\x02\x07\x12\x03M\x02#\x1a\x89\x04\x20If\x20non-em\
    pty,\x20will\x20use\x20this\x20as\x20an\x20alternative\x20way\x20to\x20s\
    pecify\x20a\x20list\x20of\n\x20optimizations\x20to\x20turn\x20on\x20and\
    \x20the\x20order\x20of\x20the\x20optimizations\x20(replacing\x20the\n\
    \x20meta-optimizer).\n\n\x20Of\x20the\x20RewriterConfig\x20options,\x20o\
    nly\x20the\x20AutoParallel\x20configuration\x20options\n\x20(the\x20auto\
    _parallel\x20field)\x20apply\x20to\x20manually\x20requested\x20optimizat\
    ion\x20passes\n\x20(\"autoparallel\").\x20Memory\x20optimization\x20pass\
    es\x20(\"memory\")\x20invoked\x20here\x20are\n\x20not\x20configurable\
    \x20(in\x20contrast\x20to\x20memory\x20optimization\x20passes\x20through\
    \x20the\n\x20meta-optimizer)\x20and\x20act\x20only\x20on\x20manual\x20op\
    \x20annotations.\n\n\x0c\n\x05\x04\x01\x02\x07\x04\x12\x03M\x02\n\n\x0c\
    \n\x05\x04\x01\x02\x07\x05\x12\x03M\x0b\x11\n\x0c\n\x05\x04\x01\x02\x07\
    \x01\x12\x03M\x12\x1c\n\x0c\n\x05\x04\x01\x02\x07\x03\x12\x03M\x1f\"b\
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
