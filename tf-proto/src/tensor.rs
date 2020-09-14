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
pub struct TensorProto {
    // message fields
    pub dtype: super::types::DataType,
    pub tensor_shape: ::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto>,
    pub version_number: i32,
    pub tensor_content: ::std::vec::Vec<u8>,
    pub half_val: ::std::vec::Vec<i32>,
    pub float_val: ::std::vec::Vec<f32>,
    pub double_val: ::std::vec::Vec<f64>,
    pub int_val: ::std::vec::Vec<i32>,
    pub string_val: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub scomplex_val: ::std::vec::Vec<f32>,
    pub int64_val: ::std::vec::Vec<i64>,
    pub bool_val: ::std::vec::Vec<bool>,
    pub dcomplex_val: ::std::vec::Vec<f64>,
    pub resource_handle_val: ::protobuf::RepeatedField<super::resource_handle::ResourceHandleProto>,
    pub variant_val: ::protobuf::RepeatedField<VariantTensorDataProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TensorProto {}

impl TensorProto {
    pub fn new() -> TensorProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TensorProto {
        static mut instance: ::protobuf::lazy::Lazy<TensorProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TensorProto,
        };
        unsafe {
            instance.get(TensorProto::new)
        }
    }

    // .tensorflow.DataType dtype = 1;

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

    // .tensorflow.TensorShapeProto tensor_shape = 2;

    pub fn clear_tensor_shape(&mut self) {
        self.tensor_shape.clear();
    }

    pub fn has_tensor_shape(&self) -> bool {
        self.tensor_shape.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tensor_shape(&mut self, v: super::tensor_shape::TensorShapeProto) {
        self.tensor_shape = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tensor_shape(&mut self) -> &mut super::tensor_shape::TensorShapeProto {
        if self.tensor_shape.is_none() {
            self.tensor_shape.set_default();
        }
        self.tensor_shape.as_mut().unwrap()
    }

    // Take field
    pub fn take_tensor_shape(&mut self) -> super::tensor_shape::TensorShapeProto {
        self.tensor_shape.take().unwrap_or_else(|| super::tensor_shape::TensorShapeProto::new())
    }

    pub fn get_tensor_shape(&self) -> &super::tensor_shape::TensorShapeProto {
        self.tensor_shape.as_ref().unwrap_or_else(|| super::tensor_shape::TensorShapeProto::default_instance())
    }

    fn get_tensor_shape_for_reflect(&self) -> &::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto> {
        &self.tensor_shape
    }

    fn mut_tensor_shape_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::tensor_shape::TensorShapeProto> {
        &mut self.tensor_shape
    }

    // int32 version_number = 3;

    pub fn clear_version_number(&mut self) {
        self.version_number = 0;
    }

    // Param is passed by value, moved
    pub fn set_version_number(&mut self, v: i32) {
        self.version_number = v;
    }

    pub fn get_version_number(&self) -> i32 {
        self.version_number
    }

    fn get_version_number_for_reflect(&self) -> &i32 {
        &self.version_number
    }

    fn mut_version_number_for_reflect(&mut self) -> &mut i32 {
        &mut self.version_number
    }

    // bytes tensor_content = 4;

    pub fn clear_tensor_content(&mut self) {
        self.tensor_content.clear();
    }

    // Param is passed by value, moved
    pub fn set_tensor_content(&mut self, v: ::std::vec::Vec<u8>) {
        self.tensor_content = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tensor_content(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tensor_content
    }

    // Take field
    pub fn take_tensor_content(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.tensor_content, ::std::vec::Vec::new())
    }

    pub fn get_tensor_content(&self) -> &[u8] {
        &self.tensor_content
    }

    fn get_tensor_content_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.tensor_content
    }

    fn mut_tensor_content_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tensor_content
    }

    // repeated int32 half_val = 13;

    pub fn clear_half_val(&mut self) {
        self.half_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_half_val(&mut self, v: ::std::vec::Vec<i32>) {
        self.half_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_half_val(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.half_val
    }

    // Take field
    pub fn take_half_val(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.half_val, ::std::vec::Vec::new())
    }

    pub fn get_half_val(&self) -> &[i32] {
        &self.half_val
    }

    fn get_half_val_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.half_val
    }

    fn mut_half_val_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.half_val
    }

    // repeated float float_val = 5;

    pub fn clear_float_val(&mut self) {
        self.float_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_float_val(&mut self, v: ::std::vec::Vec<f32>) {
        self.float_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_float_val(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.float_val
    }

    // Take field
    pub fn take_float_val(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.float_val, ::std::vec::Vec::new())
    }

    pub fn get_float_val(&self) -> &[f32] {
        &self.float_val
    }

    fn get_float_val_for_reflect(&self) -> &::std::vec::Vec<f32> {
        &self.float_val
    }

    fn mut_float_val_for_reflect(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.float_val
    }

    // repeated double double_val = 6;

    pub fn clear_double_val(&mut self) {
        self.double_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_double_val(&mut self, v: ::std::vec::Vec<f64>) {
        self.double_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_double_val(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.double_val
    }

    // Take field
    pub fn take_double_val(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.double_val, ::std::vec::Vec::new())
    }

    pub fn get_double_val(&self) -> &[f64] {
        &self.double_val
    }

    fn get_double_val_for_reflect(&self) -> &::std::vec::Vec<f64> {
        &self.double_val
    }

    fn mut_double_val_for_reflect(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.double_val
    }

    // repeated int32 int_val = 7;

    pub fn clear_int_val(&mut self) {
        self.int_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_int_val(&mut self, v: ::std::vec::Vec<i32>) {
        self.int_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_int_val(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.int_val
    }

    // Take field
    pub fn take_int_val(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.int_val, ::std::vec::Vec::new())
    }

    pub fn get_int_val(&self) -> &[i32] {
        &self.int_val
    }

    fn get_int_val_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.int_val
    }

    fn mut_int_val_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.int_val
    }

    // repeated bytes string_val = 8;

    pub fn clear_string_val(&mut self) {
        self.string_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_string_val(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.string_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_string_val(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.string_val
    }

    // Take field
    pub fn take_string_val(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.string_val, ::protobuf::RepeatedField::new())
    }

    pub fn get_string_val(&self) -> &[::std::vec::Vec<u8>] {
        &self.string_val
    }

    fn get_string_val_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.string_val
    }

    fn mut_string_val_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.string_val
    }

    // repeated float scomplex_val = 9;

    pub fn clear_scomplex_val(&mut self) {
        self.scomplex_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_scomplex_val(&mut self, v: ::std::vec::Vec<f32>) {
        self.scomplex_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_scomplex_val(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.scomplex_val
    }

    // Take field
    pub fn take_scomplex_val(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.scomplex_val, ::std::vec::Vec::new())
    }

    pub fn get_scomplex_val(&self) -> &[f32] {
        &self.scomplex_val
    }

    fn get_scomplex_val_for_reflect(&self) -> &::std::vec::Vec<f32> {
        &self.scomplex_val
    }

    fn mut_scomplex_val_for_reflect(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.scomplex_val
    }

    // repeated int64 int64_val = 10;

    pub fn clear_int64_val(&mut self) {
        self.int64_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_int64_val(&mut self, v: ::std::vec::Vec<i64>) {
        self.int64_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_int64_val(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.int64_val
    }

    // Take field
    pub fn take_int64_val(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.int64_val, ::std::vec::Vec::new())
    }

    pub fn get_int64_val(&self) -> &[i64] {
        &self.int64_val
    }

    fn get_int64_val_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.int64_val
    }

    fn mut_int64_val_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.int64_val
    }

    // repeated bool bool_val = 11;

    pub fn clear_bool_val(&mut self) {
        self.bool_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_bool_val(&mut self, v: ::std::vec::Vec<bool>) {
        self.bool_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bool_val(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.bool_val
    }

    // Take field
    pub fn take_bool_val(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.bool_val, ::std::vec::Vec::new())
    }

    pub fn get_bool_val(&self) -> &[bool] {
        &self.bool_val
    }

    fn get_bool_val_for_reflect(&self) -> &::std::vec::Vec<bool> {
        &self.bool_val
    }

    fn mut_bool_val_for_reflect(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.bool_val
    }

    // repeated double dcomplex_val = 12;

    pub fn clear_dcomplex_val(&mut self) {
        self.dcomplex_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_dcomplex_val(&mut self, v: ::std::vec::Vec<f64>) {
        self.dcomplex_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_dcomplex_val(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.dcomplex_val
    }

    // Take field
    pub fn take_dcomplex_val(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.dcomplex_val, ::std::vec::Vec::new())
    }

    pub fn get_dcomplex_val(&self) -> &[f64] {
        &self.dcomplex_val
    }

    fn get_dcomplex_val_for_reflect(&self) -> &::std::vec::Vec<f64> {
        &self.dcomplex_val
    }

    fn mut_dcomplex_val_for_reflect(&mut self) -> &mut ::std::vec::Vec<f64> {
        &mut self.dcomplex_val
    }

    // repeated .tensorflow.ResourceHandleProto resource_handle_val = 14;

    pub fn clear_resource_handle_val(&mut self) {
        self.resource_handle_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_resource_handle_val(&mut self, v: ::protobuf::RepeatedField<super::resource_handle::ResourceHandleProto>) {
        self.resource_handle_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resource_handle_val(&mut self) -> &mut ::protobuf::RepeatedField<super::resource_handle::ResourceHandleProto> {
        &mut self.resource_handle_val
    }

    // Take field
    pub fn take_resource_handle_val(&mut self) -> ::protobuf::RepeatedField<super::resource_handle::ResourceHandleProto> {
        ::std::mem::replace(&mut self.resource_handle_val, ::protobuf::RepeatedField::new())
    }

    pub fn get_resource_handle_val(&self) -> &[super::resource_handle::ResourceHandleProto] {
        &self.resource_handle_val
    }

    fn get_resource_handle_val_for_reflect(&self) -> &::protobuf::RepeatedField<super::resource_handle::ResourceHandleProto> {
        &self.resource_handle_val
    }

    fn mut_resource_handle_val_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::resource_handle::ResourceHandleProto> {
        &mut self.resource_handle_val
    }

    // repeated .tensorflow.VariantTensorDataProto variant_val = 15;

    pub fn clear_variant_val(&mut self) {
        self.variant_val.clear();
    }

    // Param is passed by value, moved
    pub fn set_variant_val(&mut self, v: ::protobuf::RepeatedField<VariantTensorDataProto>) {
        self.variant_val = v;
    }

    // Mutable pointer to the field.
    pub fn mut_variant_val(&mut self) -> &mut ::protobuf::RepeatedField<VariantTensorDataProto> {
        &mut self.variant_val
    }

    // Take field
    pub fn take_variant_val(&mut self) -> ::protobuf::RepeatedField<VariantTensorDataProto> {
        ::std::mem::replace(&mut self.variant_val, ::protobuf::RepeatedField::new())
    }

    pub fn get_variant_val(&self) -> &[VariantTensorDataProto] {
        &self.variant_val
    }

    fn get_variant_val_for_reflect(&self) -> &::protobuf::RepeatedField<VariantTensorDataProto> {
        &self.variant_val
    }

    fn mut_variant_val_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<VariantTensorDataProto> {
        &mut self.variant_val
    }
}

impl ::protobuf::Message for TensorProto {
    fn is_initialized(&self) -> bool {
        for v in &self.tensor_shape {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.resource_handle_val {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.variant_val {
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
                    self.dtype = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tensor_shape)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.version_number = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.tensor_content)?;
                },
                13 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.half_val)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.float_val)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.double_val)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.int_val)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.string_val)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.scomplex_val)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.int64_val)?;
                },
                11 => {
                    ::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.bool_val)?;
                },
                12 => {
                    ::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.dcomplex_val)?;
                },
                14 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.resource_handle_val)?;
                },
                15 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.variant_val)?;
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
        if self.dtype != super::types::DataType::DT_INVALID {
            my_size += ::protobuf::rt::enum_size(1, self.dtype);
        }
        if let Some(ref v) = self.tensor_shape.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.version_number != 0 {
            my_size += ::protobuf::rt::value_size(3, self.version_number, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.tensor_content.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.tensor_content);
        }
        if !self.half_val.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(13, &self.half_val);
        }
        if !self.float_val.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.float_val.len() as u32) + (self.float_val.len() * 4) as u32;
        }
        if !self.double_val.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.double_val.len() as u32) + (self.double_val.len() * 8) as u32;
        }
        if !self.int_val.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(7, &self.int_val);
        }
        for value in &self.string_val {
            my_size += ::protobuf::rt::bytes_size(8, &value);
        };
        if !self.scomplex_val.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.scomplex_val.len() as u32) + (self.scomplex_val.len() * 4) as u32;
        }
        if !self.int64_val.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(10, &self.int64_val);
        }
        if !self.bool_val.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.bool_val.len() as u32) + (self.bool_val.len() * 1) as u32;
        }
        if !self.dcomplex_val.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.dcomplex_val.len() as u32) + (self.dcomplex_val.len() * 8) as u32;
        }
        for value in &self.resource_handle_val {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.variant_val {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.dtype != super::types::DataType::DT_INVALID {
            os.write_enum(1, self.dtype.value())?;
        }
        if let Some(ref v) = self.tensor_shape.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.version_number != 0 {
            os.write_int32(3, self.version_number)?;
        }
        if !self.tensor_content.is_empty() {
            os.write_bytes(4, &self.tensor_content)?;
        }
        if !self.half_val.is_empty() {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.half_val))?;
            for v in &self.half_val {
                os.write_int32_no_tag(*v)?;
            };
        }
        if !self.float_val.is_empty() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.float_val.len() * 4) as u32)?;
            for v in &self.float_val {
                os.write_float_no_tag(*v)?;
            };
        }
        if !self.double_val.is_empty() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.double_val.len() * 8) as u32)?;
            for v in &self.double_val {
                os.write_double_no_tag(*v)?;
            };
        }
        if !self.int_val.is_empty() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.int_val))?;
            for v in &self.int_val {
                os.write_int32_no_tag(*v)?;
            };
        }
        for v in &self.string_val {
            os.write_bytes(8, &v)?;
        };
        if !self.scomplex_val.is_empty() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.scomplex_val.len() * 4) as u32)?;
            for v in &self.scomplex_val {
                os.write_float_no_tag(*v)?;
            };
        }
        if !self.int64_val.is_empty() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.int64_val))?;
            for v in &self.int64_val {
                os.write_int64_no_tag(*v)?;
            };
        }
        if !self.bool_val.is_empty() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.bool_val.len() * 1) as u32)?;
            for v in &self.bool_val {
                os.write_bool_no_tag(*v)?;
            };
        }
        if !self.dcomplex_val.is_empty() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.dcomplex_val.len() * 8) as u32)?;
            for v in &self.dcomplex_val {
                os.write_double_no_tag(*v)?;
            };
        }
        for v in &self.resource_handle_val {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.variant_val {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for TensorProto {
    fn new() -> TensorProto {
        TensorProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<TensorProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::types::DataType>>(
                    "dtype",
                    TensorProto::get_dtype_for_reflect,
                    TensorProto::mut_dtype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tensor_shape::TensorShapeProto>>(
                    "tensor_shape",
                    TensorProto::get_tensor_shape_for_reflect,
                    TensorProto::mut_tensor_shape_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "version_number",
                    TensorProto::get_version_number_for_reflect,
                    TensorProto::mut_version_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "tensor_content",
                    TensorProto::get_tensor_content_for_reflect,
                    TensorProto::mut_tensor_content_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "half_val",
                    TensorProto::get_half_val_for_reflect,
                    TensorProto::mut_half_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "float_val",
                    TensorProto::get_float_val_for_reflect,
                    TensorProto::mut_float_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "double_val",
                    TensorProto::get_double_val_for_reflect,
                    TensorProto::mut_double_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "int_val",
                    TensorProto::get_int_val_for_reflect,
                    TensorProto::mut_int_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "string_val",
                    TensorProto::get_string_val_for_reflect,
                    TensorProto::mut_string_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "scomplex_val",
                    TensorProto::get_scomplex_val_for_reflect,
                    TensorProto::mut_scomplex_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "int64_val",
                    TensorProto::get_int64_val_for_reflect,
                    TensorProto::mut_int64_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "bool_val",
                    TensorProto::get_bool_val_for_reflect,
                    TensorProto::mut_bool_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "dcomplex_val",
                    TensorProto::get_dcomplex_val_for_reflect,
                    TensorProto::mut_dcomplex_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::resource_handle::ResourceHandleProto>>(
                    "resource_handle_val",
                    TensorProto::get_resource_handle_val_for_reflect,
                    TensorProto::mut_resource_handle_val_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VariantTensorDataProto>>(
                    "variant_val",
                    TensorProto::get_variant_val_for_reflect,
                    TensorProto::mut_variant_val_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TensorProto>(
                    "TensorProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TensorProto {
    fn clear(&mut self) {
        self.clear_dtype();
        self.clear_tensor_shape();
        self.clear_version_number();
        self.clear_tensor_content();
        self.clear_half_val();
        self.clear_float_val();
        self.clear_double_val();
        self.clear_int_val();
        self.clear_string_val();
        self.clear_scomplex_val();
        self.clear_int64_val();
        self.clear_bool_val();
        self.clear_dcomplex_val();
        self.clear_resource_handle_val();
        self.clear_variant_val();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TensorProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TensorProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VariantTensorDataProto {
    // message fields
    pub type_name: ::std::string::String,
    pub metadata: ::std::vec::Vec<u8>,
    pub tensors: ::protobuf::RepeatedField<TensorProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VariantTensorDataProto {}

impl VariantTensorDataProto {
    pub fn new() -> VariantTensorDataProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VariantTensorDataProto {
        static mut instance: ::protobuf::lazy::Lazy<VariantTensorDataProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VariantTensorDataProto,
        };
        unsafe {
            instance.get(VariantTensorDataProto::new)
        }
    }

    // string type_name = 1;

    pub fn clear_type_name(&mut self) {
        self.type_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_type_name(&mut self, v: ::std::string::String) {
        self.type_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_type_name(&mut self) -> &mut ::std::string::String {
        &mut self.type_name
    }

    // Take field
    pub fn take_type_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.type_name, ::std::string::String::new())
    }

    pub fn get_type_name(&self) -> &str {
        &self.type_name
    }

    fn get_type_name_for_reflect(&self) -> &::std::string::String {
        &self.type_name
    }

    fn mut_type_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.type_name
    }

    // bytes metadata = 2;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: ::std::vec::Vec<u8>) {
        self.metadata = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.metadata
    }

    // Take field
    pub fn take_metadata(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.metadata, ::std::vec::Vec::new())
    }

    pub fn get_metadata(&self) -> &[u8] {
        &self.metadata
    }

    fn get_metadata_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.metadata
    }

    // repeated .tensorflow.TensorProto tensors = 3;

    pub fn clear_tensors(&mut self) {
        self.tensors.clear();
    }

    // Param is passed by value, moved
    pub fn set_tensors(&mut self, v: ::protobuf::RepeatedField<TensorProto>) {
        self.tensors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tensors(&mut self) -> &mut ::protobuf::RepeatedField<TensorProto> {
        &mut self.tensors
    }

    // Take field
    pub fn take_tensors(&mut self) -> ::protobuf::RepeatedField<TensorProto> {
        ::std::mem::replace(&mut self.tensors, ::protobuf::RepeatedField::new())
    }

    pub fn get_tensors(&self) -> &[TensorProto] {
        &self.tensors
    }

    fn get_tensors_for_reflect(&self) -> &::protobuf::RepeatedField<TensorProto> {
        &self.tensors
    }

    fn mut_tensors_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<TensorProto> {
        &mut self.tensors
    }
}

impl ::protobuf::Message for VariantTensorDataProto {
    fn is_initialized(&self) -> bool {
        for v in &self.tensors {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.type_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.metadata)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tensors)?;
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
        if !self.type_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.type_name);
        }
        if !self.metadata.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.metadata);
        }
        for value in &self.tensors {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.type_name.is_empty() {
            os.write_string(1, &self.type_name)?;
        }
        if !self.metadata.is_empty() {
            os.write_bytes(2, &self.metadata)?;
        }
        for v in &self.tensors {
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

impl ::protobuf::MessageStatic for VariantTensorDataProto {
    fn new() -> VariantTensorDataProto {
        VariantTensorDataProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<VariantTensorDataProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type_name",
                    VariantTensorDataProto::get_type_name_for_reflect,
                    VariantTensorDataProto::mut_type_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "metadata",
                    VariantTensorDataProto::get_metadata_for_reflect,
                    VariantTensorDataProto::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TensorProto>>(
                    "tensors",
                    VariantTensorDataProto::get_tensors_for_reflect,
                    VariantTensorDataProto::mut_tensors_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VariantTensorDataProto>(
                    "VariantTensorDataProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VariantTensorDataProto {
    fn clear(&mut self) {
        self.clear_type_name();
        self.clear_metadata();
        self.clear_tensors();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VariantTensorDataProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VariantTensorDataProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&tensorflow/core/framework/tensor.proto\x12\ntensorflow\x1a/tensorflow\
    /core/framework/resource_handle.proto\x1a,tensorflow/core/framework/tens\
    or_shape.proto\x1a%tensorflow/core/framework/types.proto\"\x8b\x05\n\x0b\
    TensorProto\x12*\n\x05dtype\x18\x01\x20\x01(\x0e2\x14.tensorflow.DataTyp\
    eR\x05dtype\x12?\n\x0ctensor_shape\x18\x02\x20\x01(\x0b2\x1c.tensorflow.\
    TensorShapeProtoR\x0btensorShape\x12%\n\x0eversion_number\x18\x03\x20\
    \x01(\x05R\rversionNumber\x12%\n\x0etensor_content\x18\x04\x20\x01(\x0cR\
    \rtensorContent\x12\x1d\n\x08half_val\x18\r\x20\x03(\x05R\x07halfValB\
    \x02\x10\x01\x12\x1f\n\tfloat_val\x18\x05\x20\x03(\x02R\x08floatValB\x02\
    \x10\x01\x12!\n\ndouble_val\x18\x06\x20\x03(\x01R\tdoubleValB\x02\x10\
    \x01\x12\x1b\n\x07int_val\x18\x07\x20\x03(\x05R\x06intValB\x02\x10\x01\
    \x12\x1d\n\nstring_val\x18\x08\x20\x03(\x0cR\tstringVal\x12%\n\x0cscompl\
    ex_val\x18\t\x20\x03(\x02R\x0bscomplexValB\x02\x10\x01\x12\x1f\n\tint64_\
    val\x18\n\x20\x03(\x03R\x08int64ValB\x02\x10\x01\x12\x1d\n\x08bool_val\
    \x18\x0b\x20\x03(\x08R\x07boolValB\x02\x10\x01\x12%\n\x0cdcomplex_val\
    \x18\x0c\x20\x03(\x01R\x0bdcomplexValB\x02\x10\x01\x12O\n\x13resource_ha\
    ndle_val\x18\x0e\x20\x03(\x0b2\x1f.tensorflow.ResourceHandleProtoR\x11re\
    sourceHandleVal\x12C\n\x0bvariant_val\x18\x0f\x20\x03(\x0b2\".tensorflow\
    .VariantTensorDataProtoR\nvariantVal\"\x84\x01\n\x16VariantTensorDataPro\
    to\x12\x1b\n\ttype_name\x18\x01\x20\x01(\tR\x08typeName\x12\x1a\n\x08met\
    adata\x18\x02\x20\x01(\x0cR\x08metadata\x121\n\x07tensors\x18\x03\x20\
    \x03(\x0b2\x17.tensorflow.TensorProtoR\x07tensorsB-\n\x18org.tensorflow.\
    frameworkB\x0cTensorProtosP\x01\xf8\x01\x01J\xa9\"\n\x06\x12\x04\0\0W\
    \x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\
    \n\x08\n\x01\x08\x12\x03\x03\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\
    \0\x1f\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\
    \xe7\x07\0\x02\0\x12\x03\x03\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\
    \x12\x03\x03\x07\x17\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\
    \x08\n\x01\x08\x12\x03\x04\0-\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\0-\
    \n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\
    \x07\x01\x02\0\x12\x03\x04\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\
    \x12\x03\x04\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e,\n\
    \x08\n\x01\x08\x12\x03\x05\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\
    \"\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\
    \x07\x02\x02\0\x12\x03\x05\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\
    \x12\x03\x05\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\
    \x08\n\x01\x08\x12\x03\x06\01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\
    \n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\
    \x07\x03\x02\0\x12\x03\x06\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\
    \x12\x03\x06\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\
    \t\n\x02\x03\0\x12\x03\x08\x078\n\t\n\x02\x03\x01\x12\x03\t\x075\n\t\n\
    \x02\x03\x02\x12\x03\n\x07.\n4\n\x02\x04\0\x12\x04\r\0M\x01\x1a(\x20Prot\
    ocol\x20buffer\x20representing\x20a\x20tensor.\n\n\n\n\x03\x04\0\x01\x12\
    \x03\r\x08\x13\n\x0b\n\x04\x04\0\x02\0\x12\x03\x0e\x02\x15\n\r\n\x05\x04\
    \0\x02\0\x04\x12\x04\x0e\x02\r\x15\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\
    \x0e\x02\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0e\x0b\x10\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x0e\x13\x14\nM\n\x04\x04\0\x02\x01\x12\x03\x11\
    \x02$\x1a@\x20Shape\x20of\x20the\x20tensor.\x20\x20TODO(touts):\x20sort\
    \x20out\x20the\x200-rank\x20issues.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\
    \x04\x11\x02\x0e\x15\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x11\x02\x12\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x11\x13\x1f\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03\x11\"#\n\xc9\x03\n\x04\x04\0\x02\x02\x12\x03\x1c\x02\
    \x1b\x1a\xd7\x01\x20Version\x20number.\n\n\x20In\x20version\x200,\x20if\
    \x20the\x20\"repeated\x20xxx\"\x20representations\x20contain\x20only\x20\
    one\n\x20element,\x20that\x20element\x20is\x20repeated\x20to\x20fill\x20\
    the\x20shape.\x20\x20This\x20makes\x20it\x20easy\n\x20to\x20represent\
    \x20a\x20constant\x20Tensor\x20with\x20a\x20single\x20value.\n2\xe1\x01\
    \x20Only\x20one\x20of\x20the\x20representations\x20below\x20is\x20set,\
    \x20one\x20of\x20\"tensor_contents\"\x20and\n\x20the\x20\"xxx_val\"\x20a\
    ttributes.\x20\x20We\x20are\x20not\x20using\x20oneof\x20because\x20as\
    \x20oneofs\x20cannot\n\x20contain\x20repeated\x20fields\x20it\x20would\
    \x20require\x20another\x20extra\x20set\x20of\x20messages.\n\n\r\n\x05\
    \x04\0\x02\x02\x04\x12\x04\x1c\x02\x11$\n\x0c\n\x05\x04\0\x02\x02\x05\
    \x12\x03\x1c\x02\x07\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x1c\x08\x16\n\
    \x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x1c\x19\x1a\n\xd8\x02\n\x04\x04\0\
    \x02\x03\x12\x03#\x02\x1b\x1a\xca\x02\x20Serialized\x20raw\x20tensor\x20\
    content\x20from\x20either\x20Tensor::AsProtoTensorContent\x20or\n\x20mem\
    cpy\x20in\x20tensorflow::grpc::EncodeTensorToByteBuffer.\x20This\x20repr\
    esentation\n\x20can\x20be\x20used\x20for\x20all\x20tensor\x20types.\x20T\
    he\x20purpose\x20of\x20this\x20representation\x20is\x20to\n\x20reduce\
    \x20serialization\x20overhead\x20during\x20RPC\x20call\x20by\x20avoiding\
    \x20serialization\x20of\n\x20many\x20repeated\x20small\x20items.\n\n\r\n\
    \x05\x04\0\x02\x03\x04\x12\x04#\x02\x1c\x1b\n\x0c\n\x05\x04\0\x02\x03\
    \x05\x12\x03#\x02\x07\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03#\x08\x16\n\
    \x0c\n\x05\x04\0\x02\x03\x03\x12\x03#\x19\x1a\n\xef\x02\n\x04\x04\0\x02\
    \x04\x12\x03,\x02/\x1as\x20DT_HALF.\x20Note\x20that\x20since\x20protobuf\
    \x20has\x20no\x20int16\x20type,\x20we'll\x20have\x20some\n\x20pointless\
    \x20zero\x20padding\x20for\x20each\x20value\x20here.\n2\xec\x01\x20Type\
    \x20specific\x20representations\x20that\x20make\x20it\x20easy\x20to\x20c\
    reate\x20tensor\x20protos\x20in\n\x20all\x20languages.\x20\x20Only\x20th\
    e\x20representation\x20corresponding\x20to\x20\"dtype\"\x20can\n\x20be\
    \x20set.\x20\x20The\x20values\x20hold\x20the\x20flattened\x20representat\
    ion\x20of\x20the\x20tensor\x20in\n\x20row\x20major\x20order.\n\n\x0c\n\
    \x05\x04\0\x02\x04\x04\x12\x03,\x02\n\n\x0c\n\x05\x04\0\x02\x04\x05\x12\
    \x03,\x0b\x10\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03,\x11\x19\n\x0c\n\x05\
    \x04\0\x02\x04\x03\x12\x03,\x1c\x1e\n\x0c\n\x05\x04\0\x02\x04\x08\x12\
    \x03,\x1f.\n\x0f\n\x08\x04\0\x02\x04\x08\xe7\x07\0\x12\x03,\x20-\n\x10\n\
    \t\x04\0\x02\x04\x08\xe7\x07\0\x02\x12\x03,\x20&\n\x11\n\n\x04\0\x02\x04\
    \x08\xe7\x07\0\x02\0\x12\x03,\x20&\n\x12\n\x0b\x04\0\x02\x04\x08\xe7\x07\
    \0\x02\0\x01\x12\x03,\x20&\n\x10\n\t\x04\0\x02\x04\x08\xe7\x07\0\x03\x12\
    \x03,)-\n\x18\n\x04\x04\0\x02\x05\x12\x03/\x02/\x1a\x0b\x20DT_FLOAT.\n\n\
    \x0c\n\x05\x04\0\x02\x05\x04\x12\x03/\x02\n\n\x0c\n\x05\x04\0\x02\x05\
    \x05\x12\x03/\x0b\x10\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03/\x11\x1a\n\
    \x0c\n\x05\x04\0\x02\x05\x03\x12\x03/\x1d\x1e\n\x0c\n\x05\x04\0\x02\x05\
    \x08\x12\x03/\x1f.\n\x0f\n\x08\x04\0\x02\x05\x08\xe7\x07\0\x12\x03/\x20-\
    \n\x10\n\t\x04\0\x02\x05\x08\xe7\x07\0\x02\x12\x03/\x20&\n\x11\n\n\x04\0\
    \x02\x05\x08\xe7\x07\0\x02\0\x12\x03/\x20&\n\x12\n\x0b\x04\0\x02\x05\x08\
    \xe7\x07\0\x02\0\x01\x12\x03/\x20&\n\x10\n\t\x04\0\x02\x05\x08\xe7\x07\0\
    \x03\x12\x03/)-\n\x19\n\x04\x04\0\x02\x06\x12\x032\x021\x1a\x0c\x20DT_DO\
    UBLE.\n\n\x0c\n\x05\x04\0\x02\x06\x04\x12\x032\x02\n\n\x0c\n\x05\x04\0\
    \x02\x06\x05\x12\x032\x0b\x11\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x032\x12\
    \x1c\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x032\x1f\x20\n\x0c\n\x05\x04\0\
    \x02\x06\x08\x12\x032!0\n\x0f\n\x08\x04\0\x02\x06\x08\xe7\x07\0\x12\x032\
    \"/\n\x10\n\t\x04\0\x02\x06\x08\xe7\x07\0\x02\x12\x032\"(\n\x11\n\n\x04\
    \0\x02\x06\x08\xe7\x07\0\x02\0\x12\x032\"(\n\x12\n\x0b\x04\0\x02\x06\x08\
    \xe7\x07\0\x02\0\x01\x12\x032\"(\n\x10\n\t\x04\0\x02\x06\x08\xe7\x07\0\
    \x03\x12\x032+/\n5\n\x04\x04\0\x02\x07\x12\x035\x02-\x1a(\x20DT_INT32,\
    \x20DT_INT16,\x20DT_INT8,\x20DT_UINT8.\n\n\x0c\n\x05\x04\0\x02\x07\x04\
    \x12\x035\x02\n\n\x0c\n\x05\x04\0\x02\x07\x05\x12\x035\x0b\x10\n\x0c\n\
    \x05\x04\0\x02\x07\x01\x12\x035\x11\x18\n\x0c\n\x05\x04\0\x02\x07\x03\
    \x12\x035\x1b\x1c\n\x0c\n\x05\x04\0\x02\x07\x08\x12\x035\x1d,\n\x0f\n\
    \x08\x04\0\x02\x07\x08\xe7\x07\0\x12\x035\x1e+\n\x10\n\t\x04\0\x02\x07\
    \x08\xe7\x07\0\x02\x12\x035\x1e$\n\x11\n\n\x04\0\x02\x07\x08\xe7\x07\0\
    \x02\0\x12\x035\x1e$\n\x12\n\x0b\x04\0\x02\x07\x08\xe7\x07\0\x02\0\x01\
    \x12\x035\x1e$\n\x10\n\t\x04\0\x02\x07\x08\xe7\x07\0\x03\x12\x035'+\n\
    \x18\n\x04\x04\0\x02\x08\x12\x038\x02\x20\x1a\x0b\x20DT_STRING\n\n\x0c\n\
    \x05\x04\0\x02\x08\x04\x12\x038\x02\n\n\x0c\n\x05\x04\0\x02\x08\x05\x12\
    \x038\x0b\x10\n\x0c\n\x05\x04\0\x02\x08\x01\x12\x038\x11\x1b\n\x0c\n\x05\
    \x04\0\x02\x08\x03\x12\x038\x1e\x1f\n\x86\x01\n\x04\x04\0\x02\t\x12\x03<\
    \x022\x1ay\x20DT_COMPLEX64.\x20scomplex_val(2*i)\x20and\x20scomplex_val(\
    2*i+1)\x20are\x20real\n\x20and\x20imaginary\x20parts\x20of\x20i-th\x20si\
    ngle\x20precision\x20complex.\n\n\x0c\n\x05\x04\0\x02\t\x04\x12\x03<\x02\
    \n\n\x0c\n\x05\x04\0\x02\t\x05\x12\x03<\x0b\x10\n\x0c\n\x05\x04\0\x02\t\
    \x01\x12\x03<\x11\x1d\n\x0c\n\x05\x04\0\x02\t\x03\x12\x03<\x20!\n\x0c\n\
    \x05\x04\0\x02\t\x08\x12\x03<\"1\n\x0f\n\x08\x04\0\x02\t\x08\xe7\x07\0\
    \x12\x03<#0\n\x10\n\t\x04\0\x02\t\x08\xe7\x07\0\x02\x12\x03<#)\n\x11\n\n\
    \x04\0\x02\t\x08\xe7\x07\0\x02\0\x12\x03<#)\n\x12\n\x0b\x04\0\x02\t\x08\
    \xe7\x07\0\x02\0\x01\x12\x03<#)\n\x10\n\t\x04\0\x02\t\x08\xe7\x07\0\x03\
    \x12\x03<,0\n\x17\n\x04\x04\0\x02\n\x12\x03?\x020\x1a\n\x20DT_INT64\n\n\
    \x0c\n\x05\x04\0\x02\n\x04\x12\x03?\x02\n\n\x0c\n\x05\x04\0\x02\n\x05\
    \x12\x03?\x0b\x10\n\x0c\n\x05\x04\0\x02\n\x01\x12\x03?\x11\x1a\n\x0c\n\
    \x05\x04\0\x02\n\x03\x12\x03?\x1d\x1f\n\x0c\n\x05\x04\0\x02\n\x08\x12\
    \x03?\x20/\n\x0f\n\x08\x04\0\x02\n\x08\xe7\x07\0\x12\x03?!.\n\x10\n\t\
    \x04\0\x02\n\x08\xe7\x07\0\x02\x12\x03?!'\n\x11\n\n\x04\0\x02\n\x08\xe7\
    \x07\0\x02\0\x12\x03?!'\n\x12\n\x0b\x04\0\x02\n\x08\xe7\x07\0\x02\0\x01\
    \x12\x03?!'\n\x10\n\t\x04\0\x02\n\x08\xe7\x07\0\x03\x12\x03?*.\n\x16\n\
    \x04\x04\0\x02\x0b\x12\x03B\x02.\x1a\t\x20DT_BOOL\n\n\x0c\n\x05\x04\0\
    \x02\x0b\x04\x12\x03B\x02\n\n\x0c\n\x05\x04\0\x02\x0b\x05\x12\x03B\x0b\
    \x0f\n\x0c\n\x05\x04\0\x02\x0b\x01\x12\x03B\x10\x18\n\x0c\n\x05\x04\0\
    \x02\x0b\x03\x12\x03B\x1b\x1d\n\x0c\n\x05\x04\0\x02\x0b\x08\x12\x03B\x1e\
    -\n\x0f\n\x08\x04\0\x02\x0b\x08\xe7\x07\0\x12\x03B\x1f,\n\x10\n\t\x04\0\
    \x02\x0b\x08\xe7\x07\0\x02\x12\x03B\x1f%\n\x11\n\n\x04\0\x02\x0b\x08\xe7\
    \x07\0\x02\0\x12\x03B\x1f%\n\x12\n\x0b\x04\0\x02\x0b\x08\xe7\x07\0\x02\0\
    \x01\x12\x03B\x1f%\n\x10\n\t\x04\0\x02\x0b\x08\xe7\x07\0\x03\x12\x03B(,\
    \n\x87\x01\n\x04\x04\0\x02\x0c\x12\x03F\x024\x1az\x20DT_COMPLEX128.\x20d\
    complex_val(2*i)\x20and\x20dcomplex_val(2*i+1)\x20are\x20real\n\x20and\
    \x20imaginary\x20parts\x20of\x20i-th\x20double\x20precision\x20complex.\
    \n\n\x0c\n\x05\x04\0\x02\x0c\x04\x12\x03F\x02\n\n\x0c\n\x05\x04\0\x02\
    \x0c\x05\x12\x03F\x0b\x11\n\x0c\n\x05\x04\0\x02\x0c\x01\x12\x03F\x12\x1e\
    \n\x0c\n\x05\x04\0\x02\x0c\x03\x12\x03F!#\n\x0c\n\x05\x04\0\x02\x0c\x08\
    \x12\x03F$3\n\x0f\n\x08\x04\0\x02\x0c\x08\xe7\x07\0\x12\x03F%2\n\x10\n\t\
    \x04\0\x02\x0c\x08\xe7\x07\0\x02\x12\x03F%+\n\x11\n\n\x04\0\x02\x0c\x08\
    \xe7\x07\0\x02\0\x12\x03F%+\n\x12\n\x0b\x04\0\x02\x0c\x08\xe7\x07\0\x02\
    \0\x01\x12\x03F%+\n\x10\n\t\x04\0\x02\x0c\x08\xe7\x07\0\x03\x12\x03F.2\n\
    \x1a\n\x04\x04\0\x02\r\x12\x03I\x028\x1a\r\x20DT_RESOURCE\n\n\x0c\n\x05\
    \x04\0\x02\r\x04\x12\x03I\x02\n\n\x0c\n\x05\x04\0\x02\r\x06\x12\x03I\x0b\
    \x1e\n\x0c\n\x05\x04\0\x02\r\x01\x12\x03I\x1f2\n\x0c\n\x05\x04\0\x02\r\
    \x03\x12\x03I57\n\x19\n\x04\x04\0\x02\x0e\x12\x03L\x023\x1a\x0c\x20DT_VA\
    RIANT\n\n\x0c\n\x05\x04\0\x02\x0e\x04\x12\x03L\x02\n\n\x0c\n\x05\x04\0\
    \x02\x0e\x06\x12\x03L\x0b!\n\x0c\n\x05\x04\0\x02\x0e\x01\x12\x03L\"-\n\
    \x0c\n\x05\x04\0\x02\x0e\x03\x12\x03L02\nZ\n\x02\x04\x01\x12\x04P\0W\x01\
    \x1aN\x20Protocol\x20buffer\x20representing\x20the\x20serialization\x20f\
    ormat\x20of\x20DT_VARIANT\x20tensors.\n\n\n\n\x03\x04\x01\x01\x12\x03P\
    \x08\x1e\n<\n\x04\x04\x01\x02\0\x12\x03R\x02\x17\x1a/\x20Name\x20of\x20t\
    he\x20type\x20of\x20objects\x20being\x20serialized.\n\n\r\n\x05\x04\x01\
    \x02\0\x04\x12\x04R\x02P\x20\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03R\x02\
    \x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03R\t\x12\n\x0c\n\x05\x04\x01\
    \x02\0\x03\x12\x03R\x15\x16\n;\n\x04\x04\x01\x02\x01\x12\x03T\x02\x15\
    \x1a.\x20Portions\x20of\x20the\x20object\x20that\x20are\x20not\x20Tensor\
    s.\n\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04T\x02R\x17\n\x0c\n\x05\x04\x01\
    \x02\x01\x05\x12\x03T\x02\x07\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03T\
    \x08\x10\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03T\x13\x14\nA\n\x04\x04\
    \x01\x02\x02\x12\x03V\x02#\x1a4\x20Tensors\x20contained\x20within\x20obj\
    ects\x20being\x20serialized.\n\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x03V\
    \x02\n\n\x0c\n\x05\x04\x01\x02\x02\x06\x12\x03V\x0b\x16\n\x0c\n\x05\x04\
    \x01\x02\x02\x01\x12\x03V\x17\x1e\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\
    \x03V!\"b\x06proto3\
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
