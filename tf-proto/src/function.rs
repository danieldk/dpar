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
pub struct FunctionDefLibrary {
    // message fields
    pub function: ::protobuf::RepeatedField<FunctionDef>,
    pub gradient: ::protobuf::RepeatedField<GradientDef>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FunctionDefLibrary {}

impl FunctionDefLibrary {
    pub fn new() -> FunctionDefLibrary {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FunctionDefLibrary {
        static mut instance: ::protobuf::lazy::Lazy<FunctionDefLibrary> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FunctionDefLibrary,
        };
        unsafe {
            instance.get(FunctionDefLibrary::new)
        }
    }

    // repeated .tensorflow.FunctionDef function = 1;

    pub fn clear_function(&mut self) {
        self.function.clear();
    }

    // Param is passed by value, moved
    pub fn set_function(&mut self, v: ::protobuf::RepeatedField<FunctionDef>) {
        self.function = v;
    }

    // Mutable pointer to the field.
    pub fn mut_function(&mut self) -> &mut ::protobuf::RepeatedField<FunctionDef> {
        &mut self.function
    }

    // Take field
    pub fn take_function(&mut self) -> ::protobuf::RepeatedField<FunctionDef> {
        ::std::mem::replace(&mut self.function, ::protobuf::RepeatedField::new())
    }

    pub fn get_function(&self) -> &[FunctionDef] {
        &self.function
    }

    fn get_function_for_reflect(&self) -> &::protobuf::RepeatedField<FunctionDef> {
        &self.function
    }

    fn mut_function_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<FunctionDef> {
        &mut self.function
    }

    // repeated .tensorflow.GradientDef gradient = 2;

    pub fn clear_gradient(&mut self) {
        self.gradient.clear();
    }

    // Param is passed by value, moved
    pub fn set_gradient(&mut self, v: ::protobuf::RepeatedField<GradientDef>) {
        self.gradient = v;
    }

    // Mutable pointer to the field.
    pub fn mut_gradient(&mut self) -> &mut ::protobuf::RepeatedField<GradientDef> {
        &mut self.gradient
    }

    // Take field
    pub fn take_gradient(&mut self) -> ::protobuf::RepeatedField<GradientDef> {
        ::std::mem::replace(&mut self.gradient, ::protobuf::RepeatedField::new())
    }

    pub fn get_gradient(&self) -> &[GradientDef] {
        &self.gradient
    }

    fn get_gradient_for_reflect(&self) -> &::protobuf::RepeatedField<GradientDef> {
        &self.gradient
    }

    fn mut_gradient_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<GradientDef> {
        &mut self.gradient
    }
}

impl ::protobuf::Message for FunctionDefLibrary {
    fn is_initialized(&self) -> bool {
        for v in &self.function {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.gradient {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.function)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.gradient)?;
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
        for value in &self.function {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.gradient {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.function {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.gradient {
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

impl ::protobuf::MessageStatic for FunctionDefLibrary {
    fn new() -> FunctionDefLibrary {
        FunctionDefLibrary::new()
    }

    fn descriptor_static(_: ::std::option::Option<FunctionDefLibrary>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FunctionDef>>(
                    "function",
                    FunctionDefLibrary::get_function_for_reflect,
                    FunctionDefLibrary::mut_function_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GradientDef>>(
                    "gradient",
                    FunctionDefLibrary::get_gradient_for_reflect,
                    FunctionDefLibrary::mut_gradient_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FunctionDefLibrary>(
                    "FunctionDefLibrary",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FunctionDefLibrary {
    fn clear(&mut self) {
        self.clear_function();
        self.clear_gradient();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FunctionDefLibrary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FunctionDefLibrary {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FunctionDef {
    // message fields
    pub signature: ::protobuf::SingularPtrField<super::op_def::OpDef>,
    pub attr: ::std::collections::HashMap<::std::string::String, super::attr_value::AttrValue>,
    pub node_def: ::protobuf::RepeatedField<super::node_def::NodeDef>,
    pub ret: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FunctionDef {}

impl FunctionDef {
    pub fn new() -> FunctionDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FunctionDef {
        static mut instance: ::protobuf::lazy::Lazy<FunctionDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FunctionDef,
        };
        unsafe {
            instance.get(FunctionDef::new)
        }
    }

    // .tensorflow.OpDef signature = 1;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: super::op_def::OpDef) {
        self.signature = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut super::op_def::OpDef {
        if self.signature.is_none() {
            self.signature.set_default();
        }
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> super::op_def::OpDef {
        self.signature.take().unwrap_or_else(|| super::op_def::OpDef::new())
    }

    pub fn get_signature(&self) -> &super::op_def::OpDef {
        self.signature.as_ref().unwrap_or_else(|| super::op_def::OpDef::default_instance())
    }

    fn get_signature_for_reflect(&self) -> &::protobuf::SingularPtrField<super::op_def::OpDef> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::op_def::OpDef> {
        &mut self.signature
    }

    // repeated .tensorflow.FunctionDef.AttrEntry attr = 5;

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

    // repeated .tensorflow.NodeDef node_def = 3;

    pub fn clear_node_def(&mut self) {
        self.node_def.clear();
    }

    // Param is passed by value, moved
    pub fn set_node_def(&mut self, v: ::protobuf::RepeatedField<super::node_def::NodeDef>) {
        self.node_def = v;
    }

    // Mutable pointer to the field.
    pub fn mut_node_def(&mut self) -> &mut ::protobuf::RepeatedField<super::node_def::NodeDef> {
        &mut self.node_def
    }

    // Take field
    pub fn take_node_def(&mut self) -> ::protobuf::RepeatedField<super::node_def::NodeDef> {
        ::std::mem::replace(&mut self.node_def, ::protobuf::RepeatedField::new())
    }

    pub fn get_node_def(&self) -> &[super::node_def::NodeDef] {
        &self.node_def
    }

    fn get_node_def_for_reflect(&self) -> &::protobuf::RepeatedField<super::node_def::NodeDef> {
        &self.node_def
    }

    fn mut_node_def_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::node_def::NodeDef> {
        &mut self.node_def
    }

    // repeated .tensorflow.FunctionDef.RetEntry ret = 4;

    pub fn clear_ret(&mut self) {
        self.ret.clear();
    }

    // Param is passed by value, moved
    pub fn set_ret(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.ret = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ret(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.ret
    }

    // Take field
    pub fn take_ret(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.ret, ::std::collections::HashMap::new())
    }

    pub fn get_ret(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.ret
    }

    fn get_ret_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.ret
    }

    fn mut_ret_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.ret
    }
}

impl ::protobuf::Message for FunctionDef {
    fn is_initialized(&self) -> bool {
        for v in &self.signature {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.node_def {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.signature)?;
                },
                5 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<super::attr_value::AttrValue>>(wire_type, is, &mut self.attr)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.node_def)?;
                },
                4 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.ret)?;
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
        if let Some(ref v) = self.signature.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<super::attr_value::AttrValue>>(5, &self.attr);
        for value in &self.node_def {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(4, &self.ret);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.signature.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<super::attr_value::AttrValue>>(5, &self.attr, os)?;
        for v in &self.node_def {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(4, &self.ret, os)?;
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

impl ::protobuf::MessageStatic for FunctionDef {
    fn new() -> FunctionDef {
        FunctionDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<FunctionDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::op_def::OpDef>>(
                    "signature",
                    FunctionDef::get_signature_for_reflect,
                    FunctionDef::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<super::attr_value::AttrValue>>(
                    "attr",
                    FunctionDef::get_attr_for_reflect,
                    FunctionDef::mut_attr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::node_def::NodeDef>>(
                    "node_def",
                    FunctionDef::get_node_def_for_reflect,
                    FunctionDef::mut_node_def_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                    "ret",
                    FunctionDef::get_ret_for_reflect,
                    FunctionDef::mut_ret_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FunctionDef>(
                    "FunctionDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FunctionDef {
    fn clear(&mut self) {
        self.clear_signature();
        self.clear_attr();
        self.clear_node_def();
        self.clear_ret();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FunctionDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FunctionDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GradientDef {
    // message fields
    pub function_name: ::std::string::String,
    pub gradient_func: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GradientDef {}

impl GradientDef {
    pub fn new() -> GradientDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GradientDef {
        static mut instance: ::protobuf::lazy::Lazy<GradientDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GradientDef,
        };
        unsafe {
            instance.get(GradientDef::new)
        }
    }

    // string function_name = 1;

    pub fn clear_function_name(&mut self) {
        self.function_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_function_name(&mut self, v: ::std::string::String) {
        self.function_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_function_name(&mut self) -> &mut ::std::string::String {
        &mut self.function_name
    }

    // Take field
    pub fn take_function_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.function_name, ::std::string::String::new())
    }

    pub fn get_function_name(&self) -> &str {
        &self.function_name
    }

    fn get_function_name_for_reflect(&self) -> &::std::string::String {
        &self.function_name
    }

    fn mut_function_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.function_name
    }

    // string gradient_func = 2;

    pub fn clear_gradient_func(&mut self) {
        self.gradient_func.clear();
    }

    // Param is passed by value, moved
    pub fn set_gradient_func(&mut self, v: ::std::string::String) {
        self.gradient_func = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gradient_func(&mut self) -> &mut ::std::string::String {
        &mut self.gradient_func
    }

    // Take field
    pub fn take_gradient_func(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.gradient_func, ::std::string::String::new())
    }

    pub fn get_gradient_func(&self) -> &str {
        &self.gradient_func
    }

    fn get_gradient_func_for_reflect(&self) -> &::std::string::String {
        &self.gradient_func
    }

    fn mut_gradient_func_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.gradient_func
    }
}

impl ::protobuf::Message for GradientDef {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.function_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.gradient_func)?;
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
        if !self.function_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.function_name);
        }
        if !self.gradient_func.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.gradient_func);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.function_name.is_empty() {
            os.write_string(1, &self.function_name)?;
        }
        if !self.gradient_func.is_empty() {
            os.write_string(2, &self.gradient_func)?;
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

impl ::protobuf::MessageStatic for GradientDef {
    fn new() -> GradientDef {
        GradientDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<GradientDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "function_name",
                    GradientDef::get_function_name_for_reflect,
                    GradientDef::mut_function_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "gradient_func",
                    GradientDef::get_gradient_func_for_reflect,
                    GradientDef::mut_gradient_func_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GradientDef>(
                    "GradientDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GradientDef {
    fn clear(&mut self) {
        self.clear_function_name();
        self.clear_gradient_func();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GradientDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GradientDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(tensorflow/core/framework/function.proto\x12\ntensorflow\x1a*tensorfl\
    ow/core/framework/attr_value.proto\x1a(tensorflow/core/framework/node_de\
    f.proto\x1a&tensorflow/core/framework/op_def.proto\"~\n\x12FunctionDefLi\
    brary\x123\n\x08function\x18\x01\x20\x03(\x0b2\x17.tensorflow.FunctionDe\
    fR\x08function\x123\n\x08gradient\x18\x02\x20\x03(\x0b2\x17.tensorflow.G\
    radientDefR\x08gradient\"\xe1\x02\n\x0bFunctionDef\x12/\n\tsignature\x18\
    \x01\x20\x01(\x0b2\x11.tensorflow.OpDefR\tsignature\x125\n\x04attr\x18\
    \x05\x20\x03(\x0b2!.tensorflow.FunctionDef.AttrEntryR\x04attr\x12.\n\x08\
    node_def\x18\x03\x20\x03(\x0b2\x13.tensorflow.NodeDefR\x07nodeDef\x122\n\
    \x03ret\x18\x04\x20\x03(\x0b2\x20.tensorflow.FunctionDef.RetEntryR\x03re\
    t\x1aN\n\tAttrEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12+\n\
    \x05value\x18\x02\x20\x01(\x0b2\x15.tensorflow.AttrValueR\x05value:\x028\
    \x01\x1a6\n\x08RetEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\
    \x14\n\x05value\x18\x02\x20\x01(\tR\x05value:\x028\x01\"W\n\x0bGradientD\
    ef\x12#\n\rfunction_name\x18\x01\x20\x01(\tR\x0cfunctionName\x12#\n\rgra\
    dient_func\x18\x02\x20\x01(\tR\x0cgradientFuncB/\n\x18org.tensorflow.fra\
    meworkB\x0eFunctionProtosP\x01\xf8\x01\x01J\xf0!\n\x06\x12\x04\0\0d\x01\
    \n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\n\
    \x08\n\x01\x08\x12\x03\x03\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\
    \x1f\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\
    \x07\0\x02\0\x12\x03\x03\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\
    \x03\x03\x07\x17\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\
    \n\x01\x08\x12\x03\x04\0/\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\0/\n\
    \x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\
    \x01\x02\0\x12\x03\x04\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\
    \x03\x04\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e.\n\x08\
    \n\x01\x08\x12\x03\x05\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\
    \x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\
    \x02\x02\0\x12\x03\x05\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\
    \x03\x05\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\
    \n\x01\x08\x12\x03\x06\01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\
    \x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\
    \x03\x02\0\x12\x03\x06\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\
    \x03\x06\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\t\n\
    \x02\x03\0\x12\x03\x08\x073\n\t\n\x02\x03\x01\x12\x03\t\x071\n\t\n\x02\
    \x03\x02\x12\x03\n\x07/\n4\n\x02\x04\0\x12\x04\r\0\x10\x01\x1a(\x20A\x20\
    library\x20is\x20a\x20set\x20of\x20named\x20functions.\n\n\n\n\x03\x04\0\
    \x01\x12\x03\r\x08\x1a\n\x0b\n\x04\x04\0\x02\0\x12\x03\x0e\x02$\n\x0c\n\
    \x05\x04\0\x02\0\x04\x12\x03\x0e\x02\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\
    \x03\x0e\x0b\x16\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0e\x17\x1f\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03\x0e\"#\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \x0f\x02$\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x0f\x02\n\n\x0c\n\x05\
    \x04\0\x02\x01\x06\x12\x03\x0f\x0b\x16\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03\x0f\x17\x1f\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0f\"#\n\xf3\x01\
    \n\x02\x04\x01\x12\x04\x18\0M\x01\x1a\xe6\x01\x20A\x20function\x20can\
    \x20be\x20instantiated\x20when\x20the\x20runtime\x20can\x20bind\x20every\
    \x20attr\n\x20with\x20a\x20value.\x20When\x20a\x20GraphDef\x20has\x20a\
    \x20call\x20to\x20a\x20function,\x20it\x20must\n\x20have\x20binding\x20f\
    or\x20every\x20attr\x20defined\x20in\x20the\x20signature.\n\n\x20TODO(zh\
    ifengc):\n\x20\x20\x20*\x20device\x20spec,\x20etc.\n\n\n\n\x03\x04\x01\
    \x01\x12\x03\x18\x08\x13\n[\n\x04\x04\x01\x02\0\x12\x03\x1b\x02\x16\x1aN\
    \x20The\x20definition\x20of\x20the\x20function's\x20name,\x20arguments,\
    \x20return\x20values,\n\x20attrs\x20etc.\n\n\r\n\x05\x04\x01\x02\0\x04\
    \x12\x04\x1b\x02\x18\x15\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x1b\x02\
    \x07\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x1b\x08\x11\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\x1b\x14\x15\n?\n\x04\x04\x01\x02\x01\x12\x03\x1e\
    \x02\"\x1a2\x20Attributes\x20specific\x20to\x20this\x20function\x20defin\
    ition.\n\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x1e\x02\x1b\x16\n\x0c\n\
    \x05\x04\x01\x02\x01\x06\x12\x03\x1e\x02\x18\n\x0c\n\x05\x04\x01\x02\x01\
    \x01\x12\x03\x1e\x19\x1d\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x1e\x20\
    !\n\xbd\x0f\n\x04\x04\x01\x02\x02\x12\x03H\x02\x20\x1a\x97\x01\x20By\x20\
    convention,\x20\"op\"\x20in\x20node_def\x20is\x20resolved\x20by\x20consu\
    lting\x20with\x20a\n\x20user-defined\x20library\x20first.\x20If\x20not\
    \x20resolved,\x20\"func\"\x20is\x20assumed\x20to\n\x20be\x20a\x20builtin\
    \x20op.\n2@\x20NOTE:\x20field\x20id\x202\x20deleted\x20on\x20Jan\x2011,\
    \x202016,\x20GraphDef\x20version\x2021.\n2\xad\x0c\x20In\x20both\x20of\
    \x20the\x20following\x20fields,\x20there\x20is\x20the\x20need\x20to\x20s\
    pecify\x20an\n\x20output\x20that\x20is\x20used\x20as\x20either\x20the\
    \x20input\x20to\x20another\x20node\x20(in\n\x20`node_def`)\x20or\x20as\
    \x20a\x20return\x20value\x20of\x20the\x20function\x20(in\x20`ret`).\n\
    \x20Unlike\x20the\x20NodeDefs\x20in\x20GraphDef,\x20we\x20need\x20to\x20\
    be\x20able\x20to\x20specify\x20a\n\x20list\x20in\x20some\x20cases\x20(in\
    stead\x20of\x20just\x20single\x20outputs).\x20\x20Also,\x20we\n\x20need\
    \x20to\x20be\x20able\x20to\x20deal\x20with\x20lists\x20of\x20unknown\x20\
    length\x20(so\x20the\n\x20output\x20index\x20may\x20not\x20be\x20known\
    \x20at\x20function\x20definition\x20time).\x20\x20So\n\x20we\x20use\x20t\
    he\x20following\x20format\x20instead:\n\x20*\x20\"fun_in\"\x20where\x20\
    \"fun_in\"\x20is\x20the\x20name\x20of\x20a\x20function\x20input\x20arg\
    \x20in\n\x20\x20\x20the\x20`signature`\x20field\x20above.\x20\x20This\
    \x20represents\x20that\x20input,\x20whether\n\x20\x20\x20it\x20is\x20a\
    \x20single\x20tensor\x20or\x20a\x20list.\n\x20*\x20\"fun_in:0\"\x20gives\
    \x20the\x20first\x20element\x20of\x20a\x20function\x20input\x20arg\x20(a\
    \n\x20\x20\x20non-list\x20input\x20is\x20considered\x20a\x20list\x20of\
    \x20length\x201\x20for\x20these\n\x20\x20\x20purposes).\n\x20*\x20\"node\
    :out\"\x20where\x20\"node\"\x20is\x20the\x20name\x20of\x20a\x20node\x20i\
    n\x20`node_def`\x20and\n\x20\x20\x20\"out\"\x20is\x20the\x20name\x20one\
    \x20of\x20its\x20op's\x20output\x20arguments\x20(the\x20name\n\x20\x20\
    \x20comes\x20from\x20the\x20OpDef\x20of\x20the\x20node's\x20op).\x20This\
    \x20represents\x20that\n\x20\x20\x20node's\x20output,\x20whether\x20it\
    \x20is\x20a\x20single\x20tensor\x20or\x20a\x20list.\n\x20\x20\x20Note:\
    \x20We\x20enforce\x20that\x20an\x20op's\x20output\x20arguments\x20are\
    \x20never\n\x20\x20\x20renamed\x20in\x20the\x20backwards-compatibility\
    \x20test.\n\x20*\x20\"node:out:0\"\x20gives\x20the\x20first\x20element\
    \x20of\x20a\x20node\x20output\x20arg\x20(a\n\x20\x20\x20non-list\x20outp\
    ut\x20is\x20considered\x20a\x20list\x20of\x20length\x201\x20for\x20these\
    \n\x20\x20\x20purposes).\n\n\x20NOT\x20CURRENTLY\x20SUPPORTED\x20(but\
    \x20may\x20be\x20in\x20the\x20future):\n\x20*\x20\"node:out:-1\"\x20give\
    s\x20last\x20element\x20in\x20a\x20node\x20output\x20list\n\x20*\x20\"no\
    de:out:1:\"\x20gives\x20a\x20list\x20with\x20all\x20but\x20the\x20first\
    \x20element\x20in\x20a\n\x20\x20\x20node\x20output\x20list\n\x20*\x20\"n\
    ode:out::-1\"\x20gives\x20a\x20list\x20with\x20all\x20but\x20the\x20last\
    \x20element\x20in\x20a\n\x20\x20\x20node\x20output\x20list\n2\xa3\x01\
    \x20The\x20body\x20of\x20the\x20function.\x20\x20Unlike\x20the\x20NodeDe\
    fs\x20in\x20a\x20GraphDef,\x20attrs\n\x20may\x20have\x20values\x20of\x20\
    type\x20`placeholder`\x20and\x20the\x20`input`\x20field\x20uses\n\x20the\
    \x20\"output\"\x20format\x20above.\n\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\
    \x03H\x02\n\n\x0c\n\x05\x04\x01\x02\x02\x06\x12\x03H\x0b\x12\n\x0c\n\x05\
    \x04\x01\x02\x02\x01\x12\x03H\x13\x1b\n\x0c\n\x05\x04\x01\x02\x02\x03\
    \x12\x03H\x1e\x1f\n\x8c\x01\n\x04\x04\x01\x02\x03\x12\x03L\x02\x1e\x1a\
    \x7f\x20A\x20mapping\x20from\x20the\x20output\x20arg\x20names\x20from\
    \x20`signature`\x20to\x20the\n\x20outputs\x20from\x20`node_def`\x20that\
    \x20should\x20be\x20returned\x20by\x20the\x20function.\n\n\r\n\x05\x04\
    \x01\x02\x03\x04\x12\x04L\x02H\x20\n\x0c\n\x05\x04\x01\x02\x03\x06\x12\
    \x03L\x02\x15\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03L\x16\x19\n\x0c\n\
    \x05\x04\x01\x02\x03\x03\x12\x03L\x1c\x1d\n\xfb\x05\n\x02\x04\x02\x12\
    \x04a\0d\x01\x1a\xee\x05\x20GradientDef\x20defines\x20the\x20gradient\
    \x20function\x20of\x20a\x20function\x20defined\x20in\n\x20a\x20function\
    \x20library.\n\n\x20A\x20gradient\x20function\x20g\x20(specified\x20by\
    \x20gradient_func)\x20for\x20a\x20function\x20f\n\x20(specified\x20by\
    \x20function_name)\x20must\x20follow\x20the\x20following:\n\n\x20The\x20\
    function\x20'f'\x20must\x20be\x20a\x20numerical\x20function\x20which\x20\
    takes\x20N\x20inputs\n\x20and\x20produces\x20M\x20outputs.\x20Its\x20gra\
    dient\x20function\x20'g',\x20which\x20is\x20a\n\x20function\x20taking\
    \x20N\x20+\x20M\x20inputs\x20and\x20produces\x20N\x20outputs.\n\n\x20I.e\
    .\x20if\x20we\x20have\n\x20\x20\x20\x20(y1,\x20y2,\x20...,\x20y_M)\x20=\
    \x20f(x1,\x20x2,\x20...,\x20x_N),\n\x20then,\x20g\x20is\n\x20\x20\x20\
    \x20(dL/dx1,\x20dL/dx2,\x20...,\x20dL/dx_N)\x20=\x20g(x1,\x20x2,\x20...,\
    \x20x_N,\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20dL/dy1,\x20dL/dy2,\x20...,\x20dL/dy_M),\n\x20where\
    \x20L\x20is\x20a\x20scalar-value\x20function\x20of\x20(x1,\x20x2,\x20...\
    ,\x20xN)\x20(e.g.,\x20the\n\x20loss\x20function).\x20dL/dx_i\x20is\x20th\
    e\x20partial\x20derivative\x20of\x20L\x20with\x20respect\n\x20to\x20x_i.\
    \n\n\n\n\x03\x04\x02\x01\x12\x03a\x08\x13\n!\n\x04\x04\x02\x02\0\x12\x03\
    b\x02\x1b\"\x14\x20The\x20function\x20name.\n\n\r\n\x05\x04\x02\x02\0\
    \x04\x12\x04b\x02a\x15\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03b\x02\x08\n\
    \x0c\n\x05\x04\x02\x02\0\x01\x12\x03b\t\x16\n\x0c\n\x05\x04\x02\x02\0\
    \x03\x12\x03b\x19\x1a\n,\n\x04\x04\x02\x02\x01\x12\x03c\x02\x1b\"\x1f\
    \x20The\x20gradient\x20function's\x20name.\n\n\r\n\x05\x04\x02\x02\x01\
    \x04\x12\x04c\x02b\x1b\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03c\x02\x08\
    \n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03c\t\x16\n\x0c\n\x05\x04\x02\x02\
    \x01\x03\x12\x03c\x19\x1ab\x06proto3\
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
