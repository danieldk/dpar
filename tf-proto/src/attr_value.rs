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
pub struct AttrValue {
    // message oneof groups
    value: ::std::option::Option<AttrValue_oneof_value>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AttrValue {}

#[derive(Clone,PartialEq)]
pub enum AttrValue_oneof_value {
    s(::std::vec::Vec<u8>),
    i(i64),
    f(f32),
    b(bool),
    field_type(super::types::DataType),
    shape(super::tensor_shape::TensorShapeProto),
    tensor(super::tensor::TensorProto),
    list(AttrValue_ListValue),
    func(NameAttrList),
    placeholder(::std::string::String),
}

impl AttrValue {
    pub fn new() -> AttrValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AttrValue {
        static mut instance: ::protobuf::lazy::Lazy<AttrValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AttrValue,
        };
        unsafe {
            instance.get(AttrValue::new)
        }
    }

    // bytes s = 2;

    pub fn clear_s(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_s(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::s(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_s(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::std::option::Option::Some(AttrValue_oneof_value::s(v))
    }

    // Mutable pointer to the field.
    pub fn mut_s(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(AttrValue_oneof_value::s(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(AttrValue_oneof_value::s(::std::vec::Vec::new()));
        }
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::s(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_s(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_s() {
            match self.value.take() {
                ::std::option::Option::Some(AttrValue_oneof_value::s(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_s(&self) -> &[u8] {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::s(ref v)) => v,
            _ => &[],
        }
    }

    // int64 i = 3;

    pub fn clear_i(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_i(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::i(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_i(&mut self, v: i64) {
        self.value = ::std::option::Option::Some(AttrValue_oneof_value::i(v))
    }

    pub fn get_i(&self) -> i64 {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::i(v)) => v,
            _ => 0,
        }
    }

    // float f = 4;

    pub fn clear_f(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_f(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::f(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_f(&mut self, v: f32) {
        self.value = ::std::option::Option::Some(AttrValue_oneof_value::f(v))
    }

    pub fn get_f(&self) -> f32 {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::f(v)) => v,
            _ => 0.,
        }
    }

    // bool b = 5;

    pub fn clear_b(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_b(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::b(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_b(&mut self, v: bool) {
        self.value = ::std::option::Option::Some(AttrValue_oneof_value::b(v))
    }

    pub fn get_b(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::b(v)) => v,
            _ => false,
        }
    }

    // .tensorflow.DataType type = 6;

    pub fn clear_field_type(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::field_type(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: super::types::DataType) {
        self.value = ::std::option::Option::Some(AttrValue_oneof_value::field_type(v))
    }

    pub fn get_field_type(&self) -> super::types::DataType {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::field_type(v)) => v,
            _ => super::types::DataType::DT_INVALID,
        }
    }

    // .tensorflow.TensorShapeProto shape = 7;

    pub fn clear_shape(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_shape(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::shape(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_shape(&mut self, v: super::tensor_shape::TensorShapeProto) {
        self.value = ::std::option::Option::Some(AttrValue_oneof_value::shape(v))
    }

    // Mutable pointer to the field.
    pub fn mut_shape(&mut self) -> &mut super::tensor_shape::TensorShapeProto {
        if let ::std::option::Option::Some(AttrValue_oneof_value::shape(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(AttrValue_oneof_value::shape(super::tensor_shape::TensorShapeProto::new()));
        }
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::shape(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_shape(&mut self) -> super::tensor_shape::TensorShapeProto {
        if self.has_shape() {
            match self.value.take() {
                ::std::option::Option::Some(AttrValue_oneof_value::shape(v)) => v,
                _ => panic!(),
            }
        } else {
            super::tensor_shape::TensorShapeProto::new()
        }
    }

    pub fn get_shape(&self) -> &super::tensor_shape::TensorShapeProto {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::shape(ref v)) => v,
            _ => super::tensor_shape::TensorShapeProto::default_instance(),
        }
    }

    // .tensorflow.TensorProto tensor = 8;

    pub fn clear_tensor(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_tensor(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::tensor(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_tensor(&mut self, v: super::tensor::TensorProto) {
        self.value = ::std::option::Option::Some(AttrValue_oneof_value::tensor(v))
    }

    // Mutable pointer to the field.
    pub fn mut_tensor(&mut self) -> &mut super::tensor::TensorProto {
        if let ::std::option::Option::Some(AttrValue_oneof_value::tensor(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(AttrValue_oneof_value::tensor(super::tensor::TensorProto::new()));
        }
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::tensor(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_tensor(&mut self) -> super::tensor::TensorProto {
        if self.has_tensor() {
            match self.value.take() {
                ::std::option::Option::Some(AttrValue_oneof_value::tensor(v)) => v,
                _ => panic!(),
            }
        } else {
            super::tensor::TensorProto::new()
        }
    }

    pub fn get_tensor(&self) -> &super::tensor::TensorProto {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::tensor(ref v)) => v,
            _ => super::tensor::TensorProto::default_instance(),
        }
    }

    // .tensorflow.AttrValue.ListValue list = 1;

    pub fn clear_list(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_list(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::list(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_list(&mut self, v: AttrValue_ListValue) {
        self.value = ::std::option::Option::Some(AttrValue_oneof_value::list(v))
    }

    // Mutable pointer to the field.
    pub fn mut_list(&mut self) -> &mut AttrValue_ListValue {
        if let ::std::option::Option::Some(AttrValue_oneof_value::list(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(AttrValue_oneof_value::list(AttrValue_ListValue::new()));
        }
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::list(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_list(&mut self) -> AttrValue_ListValue {
        if self.has_list() {
            match self.value.take() {
                ::std::option::Option::Some(AttrValue_oneof_value::list(v)) => v,
                _ => panic!(),
            }
        } else {
            AttrValue_ListValue::new()
        }
    }

    pub fn get_list(&self) -> &AttrValue_ListValue {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::list(ref v)) => v,
            _ => AttrValue_ListValue::default_instance(),
        }
    }

    // .tensorflow.NameAttrList func = 10;

    pub fn clear_func(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_func(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::func(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_func(&mut self, v: NameAttrList) {
        self.value = ::std::option::Option::Some(AttrValue_oneof_value::func(v))
    }

    // Mutable pointer to the field.
    pub fn mut_func(&mut self) -> &mut NameAttrList {
        if let ::std::option::Option::Some(AttrValue_oneof_value::func(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(AttrValue_oneof_value::func(NameAttrList::new()));
        }
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::func(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_func(&mut self) -> NameAttrList {
        if self.has_func() {
            match self.value.take() {
                ::std::option::Option::Some(AttrValue_oneof_value::func(v)) => v,
                _ => panic!(),
            }
        } else {
            NameAttrList::new()
        }
    }

    pub fn get_func(&self) -> &NameAttrList {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::func(ref v)) => v,
            _ => NameAttrList::default_instance(),
        }
    }

    // string placeholder = 9;

    pub fn clear_placeholder(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_placeholder(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::placeholder(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_placeholder(&mut self, v: ::std::string::String) {
        self.value = ::std::option::Option::Some(AttrValue_oneof_value::placeholder(v))
    }

    // Mutable pointer to the field.
    pub fn mut_placeholder(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(AttrValue_oneof_value::placeholder(_)) = self.value {
        } else {
            self.value = ::std::option::Option::Some(AttrValue_oneof_value::placeholder(::std::string::String::new()));
        }
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::placeholder(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_placeholder(&mut self) -> ::std::string::String {
        if self.has_placeholder() {
            match self.value.take() {
                ::std::option::Option::Some(AttrValue_oneof_value::placeholder(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_placeholder(&self) -> &str {
        match self.value {
            ::std::option::Option::Some(AttrValue_oneof_value::placeholder(ref v)) => v,
            _ => "",
        }
    }
}

impl ::protobuf::Message for AttrValue {
    fn is_initialized(&self) -> bool {
        if let Some(AttrValue_oneof_value::shape(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(AttrValue_oneof_value::tensor(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(AttrValue_oneof_value::list(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(AttrValue_oneof_value::func(ref v)) = self.value {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(AttrValue_oneof_value::s(is.read_bytes()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(AttrValue_oneof_value::i(is.read_int64()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(AttrValue_oneof_value::f(is.read_float()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(AttrValue_oneof_value::b(is.read_bool()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(AttrValue_oneof_value::field_type(is.read_enum()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(AttrValue_oneof_value::shape(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(AttrValue_oneof_value::tensor(is.read_message()?));
                },
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(AttrValue_oneof_value::list(is.read_message()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(AttrValue_oneof_value::func(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.value = ::std::option::Option::Some(AttrValue_oneof_value::placeholder(is.read_string()?));
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
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &AttrValue_oneof_value::s(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(2, &v);
                },
                &AttrValue_oneof_value::i(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
                },
                &AttrValue_oneof_value::f(v) => {
                    my_size += 5;
                },
                &AttrValue_oneof_value::b(v) => {
                    my_size += 2;
                },
                &AttrValue_oneof_value::field_type(v) => {
                    my_size += ::protobuf::rt::enum_size(6, v);
                },
                &AttrValue_oneof_value::shape(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &AttrValue_oneof_value::tensor(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &AttrValue_oneof_value::list(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &AttrValue_oneof_value::func(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &AttrValue_oneof_value::placeholder(ref v) => {
                    my_size += ::protobuf::rt::string_size(9, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &AttrValue_oneof_value::s(ref v) => {
                    os.write_bytes(2, v)?;
                },
                &AttrValue_oneof_value::i(v) => {
                    os.write_int64(3, v)?;
                },
                &AttrValue_oneof_value::f(v) => {
                    os.write_float(4, v)?;
                },
                &AttrValue_oneof_value::b(v) => {
                    os.write_bool(5, v)?;
                },
                &AttrValue_oneof_value::field_type(v) => {
                    os.write_enum(6, v.value())?;
                },
                &AttrValue_oneof_value::shape(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &AttrValue_oneof_value::tensor(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &AttrValue_oneof_value::list(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &AttrValue_oneof_value::func(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &AttrValue_oneof_value::placeholder(ref v) => {
                    os.write_string(9, v)?;
                },
            };
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

impl ::protobuf::MessageStatic for AttrValue {
    fn new() -> AttrValue {
        AttrValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<AttrValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "s",
                    AttrValue::has_s,
                    AttrValue::get_s,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor::<_>(
                    "i",
                    AttrValue::has_i,
                    AttrValue::get_i,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f32_accessor::<_>(
                    "f",
                    AttrValue::has_f,
                    AttrValue::get_f,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor::<_>(
                    "b",
                    AttrValue::has_b,
                    AttrValue::get_b,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor::<_, super::types::DataType>(
                    "type",
                    AttrValue::has_field_type,
                    AttrValue::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::tensor_shape::TensorShapeProto>(
                    "shape",
                    AttrValue::has_shape,
                    AttrValue::get_shape,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::tensor::TensorProto>(
                    "tensor",
                    AttrValue::has_tensor,
                    AttrValue::get_tensor,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, AttrValue_ListValue>(
                    "list",
                    AttrValue::has_list,
                    AttrValue::get_list,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, NameAttrList>(
                    "func",
                    AttrValue::has_func,
                    AttrValue::get_func,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "placeholder",
                    AttrValue::has_placeholder,
                    AttrValue::get_placeholder,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AttrValue>(
                    "AttrValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AttrValue {
    fn clear(&mut self) {
        self.clear_s();
        self.clear_i();
        self.clear_f();
        self.clear_b();
        self.clear_field_type();
        self.clear_shape();
        self.clear_tensor();
        self.clear_list();
        self.clear_func();
        self.clear_placeholder();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AttrValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AttrValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AttrValue_ListValue {
    // message fields
    pub s: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub i: ::std::vec::Vec<i64>,
    pub f: ::std::vec::Vec<f32>,
    pub b: ::std::vec::Vec<bool>,
    pub field_type: ::std::vec::Vec<super::types::DataType>,
    pub shape: ::protobuf::RepeatedField<super::tensor_shape::TensorShapeProto>,
    pub tensor: ::protobuf::RepeatedField<super::tensor::TensorProto>,
    pub func: ::protobuf::RepeatedField<NameAttrList>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AttrValue_ListValue {}

impl AttrValue_ListValue {
    pub fn new() -> AttrValue_ListValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AttrValue_ListValue {
        static mut instance: ::protobuf::lazy::Lazy<AttrValue_ListValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AttrValue_ListValue,
        };
        unsafe {
            instance.get(AttrValue_ListValue::new)
        }
    }

    // repeated bytes s = 2;

    pub fn clear_s(&mut self) {
        self.s.clear();
    }

    // Param is passed by value, moved
    pub fn set_s(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.s = v;
    }

    // Mutable pointer to the field.
    pub fn mut_s(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.s
    }

    // Take field
    pub fn take_s(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.s, ::protobuf::RepeatedField::new())
    }

    pub fn get_s(&self) -> &[::std::vec::Vec<u8>] {
        &self.s
    }

    fn get_s_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.s
    }

    fn mut_s_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.s
    }

    // repeated int64 i = 3;

    pub fn clear_i(&mut self) {
        self.i.clear();
    }

    // Param is passed by value, moved
    pub fn set_i(&mut self, v: ::std::vec::Vec<i64>) {
        self.i = v;
    }

    // Mutable pointer to the field.
    pub fn mut_i(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.i
    }

    // Take field
    pub fn take_i(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.i, ::std::vec::Vec::new())
    }

    pub fn get_i(&self) -> &[i64] {
        &self.i
    }

    fn get_i_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.i
    }

    fn mut_i_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.i
    }

    // repeated float f = 4;

    pub fn clear_f(&mut self) {
        self.f.clear();
    }

    // Param is passed by value, moved
    pub fn set_f(&mut self, v: ::std::vec::Vec<f32>) {
        self.f = v;
    }

    // Mutable pointer to the field.
    pub fn mut_f(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.f
    }

    // Take field
    pub fn take_f(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.f, ::std::vec::Vec::new())
    }

    pub fn get_f(&self) -> &[f32] {
        &self.f
    }

    fn get_f_for_reflect(&self) -> &::std::vec::Vec<f32> {
        &self.f
    }

    fn mut_f_for_reflect(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.f
    }

    // repeated bool b = 5;

    pub fn clear_b(&mut self) {
        self.b.clear();
    }

    // Param is passed by value, moved
    pub fn set_b(&mut self, v: ::std::vec::Vec<bool>) {
        self.b = v;
    }

    // Mutable pointer to the field.
    pub fn mut_b(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.b
    }

    // Take field
    pub fn take_b(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.b, ::std::vec::Vec::new())
    }

    pub fn get_b(&self) -> &[bool] {
        &self.b
    }

    fn get_b_for_reflect(&self) -> &::std::vec::Vec<bool> {
        &self.b
    }

    fn mut_b_for_reflect(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.b
    }

    // repeated .tensorflow.DataType type = 6;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::vec::Vec<super::types::DataType>) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    pub fn mut_field_type(&mut self) -> &mut ::std::vec::Vec<super::types::DataType> {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::vec::Vec<super::types::DataType> {
        ::std::mem::replace(&mut self.field_type, ::std::vec::Vec::new())
    }

    pub fn get_field_type(&self) -> &[super::types::DataType] {
        &self.field_type
    }

    fn get_field_type_for_reflect(&self) -> &::std::vec::Vec<super::types::DataType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::vec::Vec<super::types::DataType> {
        &mut self.field_type
    }

    // repeated .tensorflow.TensorShapeProto shape = 7;

    pub fn clear_shape(&mut self) {
        self.shape.clear();
    }

    // Param is passed by value, moved
    pub fn set_shape(&mut self, v: ::protobuf::RepeatedField<super::tensor_shape::TensorShapeProto>) {
        self.shape = v;
    }

    // Mutable pointer to the field.
    pub fn mut_shape(&mut self) -> &mut ::protobuf::RepeatedField<super::tensor_shape::TensorShapeProto> {
        &mut self.shape
    }

    // Take field
    pub fn take_shape(&mut self) -> ::protobuf::RepeatedField<super::tensor_shape::TensorShapeProto> {
        ::std::mem::replace(&mut self.shape, ::protobuf::RepeatedField::new())
    }

    pub fn get_shape(&self) -> &[super::tensor_shape::TensorShapeProto] {
        &self.shape
    }

    fn get_shape_for_reflect(&self) -> &::protobuf::RepeatedField<super::tensor_shape::TensorShapeProto> {
        &self.shape
    }

    fn mut_shape_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::tensor_shape::TensorShapeProto> {
        &mut self.shape
    }

    // repeated .tensorflow.TensorProto tensor = 8;

    pub fn clear_tensor(&mut self) {
        self.tensor.clear();
    }

    // Param is passed by value, moved
    pub fn set_tensor(&mut self, v: ::protobuf::RepeatedField<super::tensor::TensorProto>) {
        self.tensor = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tensor(&mut self) -> &mut ::protobuf::RepeatedField<super::tensor::TensorProto> {
        &mut self.tensor
    }

    // Take field
    pub fn take_tensor(&mut self) -> ::protobuf::RepeatedField<super::tensor::TensorProto> {
        ::std::mem::replace(&mut self.tensor, ::protobuf::RepeatedField::new())
    }

    pub fn get_tensor(&self) -> &[super::tensor::TensorProto] {
        &self.tensor
    }

    fn get_tensor_for_reflect(&self) -> &::protobuf::RepeatedField<super::tensor::TensorProto> {
        &self.tensor
    }

    fn mut_tensor_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::tensor::TensorProto> {
        &mut self.tensor
    }

    // repeated .tensorflow.NameAttrList func = 9;

    pub fn clear_func(&mut self) {
        self.func.clear();
    }

    // Param is passed by value, moved
    pub fn set_func(&mut self, v: ::protobuf::RepeatedField<NameAttrList>) {
        self.func = v;
    }

    // Mutable pointer to the field.
    pub fn mut_func(&mut self) -> &mut ::protobuf::RepeatedField<NameAttrList> {
        &mut self.func
    }

    // Take field
    pub fn take_func(&mut self) -> ::protobuf::RepeatedField<NameAttrList> {
        ::std::mem::replace(&mut self.func, ::protobuf::RepeatedField::new())
    }

    pub fn get_func(&self) -> &[NameAttrList] {
        &self.func
    }

    fn get_func_for_reflect(&self) -> &::protobuf::RepeatedField<NameAttrList> {
        &self.func
    }

    fn mut_func_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<NameAttrList> {
        &mut self.func
    }
}

impl ::protobuf::Message for AttrValue_ListValue {
    fn is_initialized(&self) -> bool {
        for v in &self.shape {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.tensor {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.func {
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
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.s)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.i)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.f)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.b)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.field_type)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.shape)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tensor)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.func)?;
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
        for value in &self.s {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        if !self.i.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(3, &self.i);
        }
        if !self.f.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.f.len() as u32) + (self.f.len() * 4) as u32;
        }
        if !self.b.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.b.len() as u32) + (self.b.len() * 1) as u32;
        }
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::vec_packed_enum_size(6, &self.field_type);
        }
        for value in &self.shape {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.tensor {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.func {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.s {
            os.write_bytes(2, &v)?;
        };
        if !self.i.is_empty() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.i))?;
            for v in &self.i {
                os.write_int64_no_tag(*v)?;
            };
        }
        if !self.f.is_empty() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.f.len() * 4) as u32)?;
            for v in &self.f {
                os.write_float_no_tag(*v)?;
            };
        }
        if !self.b.is_empty() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.b.len() * 1) as u32)?;
            for v in &self.b {
                os.write_bool_no_tag(*v)?;
            };
        }
        if !self.field_type.is_empty() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_enum_data_size(&self.field_type))?;
            for v in &self.field_type {
                os.write_enum_no_tag(v.value())?;
            };
        }
        for v in &self.shape {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.tensor {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.func {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for AttrValue_ListValue {
    fn new() -> AttrValue_ListValue {
        AttrValue_ListValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<AttrValue_ListValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "s",
                    AttrValue_ListValue::get_s_for_reflect,
                    AttrValue_ListValue::mut_s_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "i",
                    AttrValue_ListValue::get_i_for_reflect,
                    AttrValue_ListValue::mut_i_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "f",
                    AttrValue_ListValue::get_f_for_reflect,
                    AttrValue_ListValue::mut_f_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "b",
                    AttrValue_ListValue::get_b_for_reflect,
                    AttrValue_ListValue::mut_b_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::types::DataType>>(
                    "type",
                    AttrValue_ListValue::get_field_type_for_reflect,
                    AttrValue_ListValue::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tensor_shape::TensorShapeProto>>(
                    "shape",
                    AttrValue_ListValue::get_shape_for_reflect,
                    AttrValue_ListValue::mut_shape_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::tensor::TensorProto>>(
                    "tensor",
                    AttrValue_ListValue::get_tensor_for_reflect,
                    AttrValue_ListValue::mut_tensor_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NameAttrList>>(
                    "func",
                    AttrValue_ListValue::get_func_for_reflect,
                    AttrValue_ListValue::mut_func_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AttrValue_ListValue>(
                    "AttrValue_ListValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AttrValue_ListValue {
    fn clear(&mut self) {
        self.clear_s();
        self.clear_i();
        self.clear_f();
        self.clear_b();
        self.clear_field_type();
        self.clear_shape();
        self.clear_tensor();
        self.clear_func();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AttrValue_ListValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AttrValue_ListValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NameAttrList {
    // message fields
    pub name: ::std::string::String,
    pub attr: ::std::collections::HashMap<::std::string::String, AttrValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NameAttrList {}

impl NameAttrList {
    pub fn new() -> NameAttrList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NameAttrList {
        static mut instance: ::protobuf::lazy::Lazy<NameAttrList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NameAttrList,
        };
        unsafe {
            instance.get(NameAttrList::new)
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

    // repeated .tensorflow.NameAttrList.AttrEntry attr = 2;

    pub fn clear_attr(&mut self) {
        self.attr.clear();
    }

    // Param is passed by value, moved
    pub fn set_attr(&mut self, v: ::std::collections::HashMap<::std::string::String, AttrValue>) {
        self.attr = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attr(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, AttrValue> {
        &mut self.attr
    }

    // Take field
    pub fn take_attr(&mut self) -> ::std::collections::HashMap<::std::string::String, AttrValue> {
        ::std::mem::replace(&mut self.attr, ::std::collections::HashMap::new())
    }

    pub fn get_attr(&self) -> &::std::collections::HashMap<::std::string::String, AttrValue> {
        &self.attr
    }

    fn get_attr_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, AttrValue> {
        &self.attr
    }

    fn mut_attr_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, AttrValue> {
        &mut self.attr
    }
}

impl ::protobuf::Message for NameAttrList {
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
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<AttrValue>>(wire_type, is, &mut self.attr)?;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<AttrValue>>(2, &self.attr);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<AttrValue>>(2, &self.attr, os)?;
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

impl ::protobuf::MessageStatic for NameAttrList {
    fn new() -> NameAttrList {
        NameAttrList::new()
    }

    fn descriptor_static(_: ::std::option::Option<NameAttrList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    NameAttrList::get_name_for_reflect,
                    NameAttrList::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<AttrValue>>(
                    "attr",
                    NameAttrList::get_attr_for_reflect,
                    NameAttrList::mut_attr_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NameAttrList>(
                    "NameAttrList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NameAttrList {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_attr();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NameAttrList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NameAttrList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n*tensorflow/core/framework/attr_value.proto\x12\ntensorflow\x1a&tensor\
    flow/core/framework/tensor.proto\x1a,tensorflow/core/framework/tensor_sh\
    ape.proto\x1a%tensorflow/core/framework/types.proto\"\x87\x05\n\tAttrVal\
    ue\x12\x0e\n\x01s\x18\x02\x20\x01(\x0cH\0R\x01s\x12\x0e\n\x01i\x18\x03\
    \x20\x01(\x03H\0R\x01i\x12\x0e\n\x01f\x18\x04\x20\x01(\x02H\0R\x01f\x12\
    \x0e\n\x01b\x18\x05\x20\x01(\x08H\0R\x01b\x12*\n\x04type\x18\x06\x20\x01\
    (\x0e2\x14.tensorflow.DataTypeH\0R\x04type\x124\n\x05shape\x18\x07\x20\
    \x01(\x0b2\x1c.tensorflow.TensorShapeProtoH\0R\x05shape\x121\n\x06tensor\
    \x18\x08\x20\x01(\x0b2\x17.tensorflow.TensorProtoH\0R\x06tensor\x125\n\
    \x04list\x18\x01\x20\x01(\x0b2\x1f.tensorflow.AttrValue.ListValueH\0R\
    \x04list\x12.\n\x04func\x18\n\x20\x01(\x0b2\x18.tensorflow.NameAttrListH\
    \0R\x04func\x12\"\n\x0bplaceholder\x18\t\x20\x01(\tH\0R\x0bplaceholder\
    \x1a\x90\x02\n\tListValue\x12\x0c\n\x01s\x18\x02\x20\x03(\x0cR\x01s\x12\
    \x10\n\x01i\x18\x03\x20\x03(\x03R\x01iB\x02\x10\x01\x12\x10\n\x01f\x18\
    \x04\x20\x03(\x02R\x01fB\x02\x10\x01\x12\x10\n\x01b\x18\x05\x20\x03(\x08\
    R\x01bB\x02\x10\x01\x12,\n\x04type\x18\x06\x20\x03(\x0e2\x14.tensorflow.\
    DataTypeR\x04typeB\x02\x10\x01\x122\n\x05shape\x18\x07\x20\x03(\x0b2\x1c\
    .tensorflow.TensorShapeProtoR\x05shape\x12/\n\x06tensor\x18\x08\x20\x03(\
    \x0b2\x17.tensorflow.TensorProtoR\x06tensor\x12,\n\x04func\x18\t\x20\x03\
    (\x0b2\x18.tensorflow.NameAttrListR\x04funcB\x07\n\x05value\"\xaa\x01\n\
    \x0cNameAttrList\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x126\n\
    \x04attr\x18\x02\x20\x03(\x0b2\".tensorflow.NameAttrList.AttrEntryR\x04a\
    ttr\x1aN\n\tAttrEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12+\n\
    \x05value\x18\x02\x20\x01(\x0b2\x15.tensorflow.AttrValueR\x05value:\x028\
    \x01B0\n\x18org.tensorflow.frameworkB\x0fAttrValueProtosP\x01\xf8\x01\
    \x01J\xfb\x1a\n\x06\x12\x04\0\0=\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\
    \x08\n\x01\x02\x12\x03\x02\x08\x12\n\x08\n\x01\x08\x12\x03\x03\0\x1f\n\
    \x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\n\x0c\n\x05\x08\xe7\x07\0\x02\
    \x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x03\x07\x17\n\
    \x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x03\x07\x17\n\x0c\n\x05\x08\
    \xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\x01\x08\x12\x03\x04\00\n\x0b\
    \n\x04\x08\xe7\x07\x01\x12\x03\x04\00\n\x0c\n\x05\x08\xe7\x07\x01\x02\
    \x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x04\x07\x1b\
    \n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x04\x07\x1b\n\x0c\n\x05\
    \x08\xe7\x07\x01\x07\x12\x03\x04\x1e/\n\x08\n\x01\x08\x12\x03\x05\0\"\n\
    \x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\x0c\n\x05\x08\xe7\x07\x02\
    \x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x05\x07\
    \x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x05\x07\x1a\n\x0c\n\
    \x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\n\x01\x08\x12\x03\x06\01\
    \n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\x0c\n\x05\x08\xe7\x07\x03\
    \x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x06\x07\
    \x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x06\x07\x13\n\x0c\n\
    \x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\t\n\x02\x03\0\x12\x03\x08\
    \x07/\n\t\n\x02\x03\x01\x12\x03\t\x075\n\t\n\x02\x03\x02\x12\x03\n\x07.\
    \n\xc1\x01\n\x02\x04\0\x12\x04\x0f\06\x01\x1a\xb4\x01\x20Protocol\x20buf\
    fer\x20representing\x20the\x20value\x20for\x20an\x20attr\x20used\x20to\
    \x20configure\x20an\x20Op.\n\x20Comment\x20indicates\x20the\x20correspon\
    ding\x20attr\x20type.\x20\x20Only\x20the\x20field\x20matching\x20the\n\
    \x20attr\x20type\x20may\x20be\x20filled.\n\n\n\n\x03\x04\0\x01\x12\x03\
    \x0f\x08\x11\n\x1d\n\x04\x04\0\x03\0\x12\x04\x11\x02\x1a\x03\x1a\x0f\x20\
    LINT.IfChange\n\n\x0c\n\x05\x04\0\x03\0\x01\x12\x03\x11\n\x13\n\x1f\n\
    \x06\x04\0\x03\0\x02\0\x12\x03\x12\x04\x19\"\x10\x20\"list(string)\"\n\n\
    \x0e\n\x07\x04\0\x03\0\x02\0\x04\x12\x03\x12\x04\x0c\n\x0e\n\x07\x04\0\
    \x03\0\x02\0\x05\x12\x03\x12\r\x12\n\x0e\n\x07\x04\0\x03\0\x02\0\x01\x12\
    \x03\x12\x13\x14\n\x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\x03\x12\x17\x18\n\
    \x1c\n\x06\x04\0\x03\0\x02\x01\x12\x03\x13\x04)\"\r\x20\"list(int)\"\n\n\
    \x0e\n\x07\x04\0\x03\0\x02\x01\x04\x12\x03\x13\x04\x0c\n\x0e\n\x07\x04\0\
    \x03\0\x02\x01\x05\x12\x03\x13\r\x12\n\x0e\n\x07\x04\0\x03\0\x02\x01\x01\
    \x12\x03\x13\x13\x14\n\x0e\n\x07\x04\0\x03\0\x02\x01\x03\x12\x03\x13\x17\
    \x18\n\x0e\n\x07\x04\0\x03\0\x02\x01\x08\x12\x03\x13\x19(\n\x11\n\n\x04\
    \0\x03\0\x02\x01\x08\xe7\x07\0\x12\x03\x13\x1a'\n\x12\n\x0b\x04\0\x03\0\
    \x02\x01\x08\xe7\x07\0\x02\x12\x03\x13\x1a\x20\n\x13\n\x0c\x04\0\x03\0\
    \x02\x01\x08\xe7\x07\0\x02\0\x12\x03\x13\x1a\x20\n\x14\n\r\x04\0\x03\0\
    \x02\x01\x08\xe7\x07\0\x02\0\x01\x12\x03\x13\x1a\x20\n\x12\n\x0b\x04\0\
    \x03\0\x02\x01\x08\xe7\x07\0\x03\x12\x03\x13#'\n\x1e\n\x06\x04\0\x03\0\
    \x02\x02\x12\x03\x14\x04)\"\x0f\x20\"list(float)\"\n\n\x0e\n\x07\x04\0\
    \x03\0\x02\x02\x04\x12\x03\x14\x04\x0c\n\x0e\n\x07\x04\0\x03\0\x02\x02\
    \x05\x12\x03\x14\r\x12\n\x0e\n\x07\x04\0\x03\0\x02\x02\x01\x12\x03\x14\
    \x13\x14\n\x0e\n\x07\x04\0\x03\0\x02\x02\x03\x12\x03\x14\x17\x18\n\x0e\n\
    \x07\x04\0\x03\0\x02\x02\x08\x12\x03\x14\x19(\n\x11\n\n\x04\0\x03\0\x02\
    \x02\x08\xe7\x07\0\x12\x03\x14\x1a'\n\x12\n\x0b\x04\0\x03\0\x02\x02\x08\
    \xe7\x07\0\x02\x12\x03\x14\x1a\x20\n\x13\n\x0c\x04\0\x03\0\x02\x02\x08\
    \xe7\x07\0\x02\0\x12\x03\x14\x1a\x20\n\x14\n\r\x04\0\x03\0\x02\x02\x08\
    \xe7\x07\0\x02\0\x01\x12\x03\x14\x1a\x20\n\x12\n\x0b\x04\0\x03\0\x02\x02\
    \x08\xe7\x07\0\x03\x12\x03\x14#'\n\x1d\n\x06\x04\0\x03\0\x02\x03\x12\x03\
    \x15\x04(\"\x0e\x20\"list(bool)\"\n\n\x0e\n\x07\x04\0\x03\0\x02\x03\x04\
    \x12\x03\x15\x04\x0c\n\x0e\n\x07\x04\0\x03\0\x02\x03\x05\x12\x03\x15\r\
    \x11\n\x0e\n\x07\x04\0\x03\0\x02\x03\x01\x12\x03\x15\x12\x13\n\x0e\n\x07\
    \x04\0\x03\0\x02\x03\x03\x12\x03\x15\x16\x17\n\x0e\n\x07\x04\0\x03\0\x02\
    \x03\x08\x12\x03\x15\x18'\n\x11\n\n\x04\0\x03\0\x02\x03\x08\xe7\x07\0\
    \x12\x03\x15\x19&\n\x12\n\x0b\x04\0\x03\0\x02\x03\x08\xe7\x07\0\x02\x12\
    \x03\x15\x19\x1f\n\x13\n\x0c\x04\0\x03\0\x02\x03\x08\xe7\x07\0\x02\0\x12\
    \x03\x15\x19\x1f\n\x14\n\r\x04\0\x03\0\x02\x03\x08\xe7\x07\0\x02\0\x01\
    \x12\x03\x15\x19\x1f\n\x12\n\x0b\x04\0\x03\0\x02\x03\x08\xe7\x07\0\x03\
    \x12\x03\x15\"&\n\x1d\n\x06\x04\0\x03\0\x02\x04\x12\x03\x16\x04/\"\x0e\
    \x20\"list(type)\"\n\n\x0e\n\x07\x04\0\x03\0\x02\x04\x04\x12\x03\x16\x04\
    \x0c\n\x0e\n\x07\x04\0\x03\0\x02\x04\x06\x12\x03\x16\r\x15\n\x0e\n\x07\
    \x04\0\x03\0\x02\x04\x01\x12\x03\x16\x16\x1a\n\x0e\n\x07\x04\0\x03\0\x02\
    \x04\x03\x12\x03\x16\x1d\x1e\n\x0e\n\x07\x04\0\x03\0\x02\x04\x08\x12\x03\
    \x16\x1f.\n\x11\n\n\x04\0\x03\0\x02\x04\x08\xe7\x07\0\x12\x03\x16\x20-\n\
    \x12\n\x0b\x04\0\x03\0\x02\x04\x08\xe7\x07\0\x02\x12\x03\x16\x20&\n\x13\
    \n\x0c\x04\0\x03\0\x02\x04\x08\xe7\x07\0\x02\0\x12\x03\x16\x20&\n\x14\n\
    \r\x04\0\x03\0\x02\x04\x08\xe7\x07\0\x02\0\x01\x12\x03\x16\x20&\n\x12\n\
    \x0b\x04\0\x03\0\x02\x04\x08\xe7\x07\0\x03\x12\x03\x16)-\n\x1e\n\x06\x04\
    \0\x03\0\x02\x05\x12\x03\x17\x04(\"\x0f\x20\"list(shape)\"\n\n\x0e\n\x07\
    \x04\0\x03\0\x02\x05\x04\x12\x03\x17\x04\x0c\n\x0e\n\x07\x04\0\x03\0\x02\
    \x05\x06\x12\x03\x17\r\x1d\n\x0e\n\x07\x04\0\x03\0\x02\x05\x01\x12\x03\
    \x17\x1e#\n\x0e\n\x07\x04\0\x03\0\x02\x05\x03\x12\x03\x17&'\n\x1f\n\x06\
    \x04\0\x03\0\x02\x06\x12\x03\x18\x04$\"\x10\x20\"list(tensor)\"\n\n\x0e\
    \n\x07\x04\0\x03\0\x02\x06\x04\x12\x03\x18\x04\x0c\n\x0e\n\x07\x04\0\x03\
    \0\x02\x06\x06\x12\x03\x18\r\x18\n\x0e\n\x07\x04\0\x03\0\x02\x06\x01\x12\
    \x03\x18\x19\x1f\n\x0e\n\x07\x04\0\x03\0\x02\x06\x03\x12\x03\x18\"#\n\
    \x1d\n\x06\x04\0\x03\0\x02\x07\x12\x03\x19\x04#\"\x0e\x20\"list(attr)\"\
    \n\n\x0e\n\x07\x04\0\x03\0\x02\x07\x04\x12\x03\x19\x04\x0c\n\x0e\n\x07\
    \x04\0\x03\0\x02\x07\x06\x12\x03\x19\r\x19\n\x0e\n\x07\x04\0\x03\0\x02\
    \x07\x01\x12\x03\x19\x1a\x1e\n\x0e\n\x07\x04\0\x03\0\x02\x07\x03\x12\x03\
    \x19!\"\n\x0c\n\x04\x04\0\x08\0\x12\x04\x1d\x025\x03\n\x0c\n\x05\x04\0\
    \x08\0\x01\x12\x03\x1d\x08\r\n\x17\n\x04\x04\0\x02\0\x12\x03\x1e\x04\x10\
    \"\n\x20\"string\"\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x1e\x04\t\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03\x1e\n\x0b\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x1e\x0e\x0f\n\x14\n\x04\x04\0\x02\x01\x12\x03\x1f\x04\x10\"\x07\x20\
    \"int\"\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x1f\x04\t\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\x1f\n\x0b\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\x1f\x0e\x0f\n\x16\n\x04\x04\0\x02\x02\x12\x03\x20\x04\x10\"\t\x20\"\
    float\"\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x20\x04\t\n\x0c\n\x05\
    \x04\0\x02\x02\x01\x12\x03\x20\n\x0b\n\x0c\n\x05\x04\0\x02\x02\x03\x12\
    \x03\x20\x0e\x0f\n\x15\n\x04\x04\0\x02\x03\x12\x03!\x04\x0f\"\x08\x20\"b\
    ool\"\n\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03!\x04\x08\n\x0c\n\x05\x04\0\
    \x02\x03\x01\x12\x03!\t\n\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03!\r\x0e\n\
    \x15\n\x04\x04\0\x02\x04\x12\x03\"\x04\x16\"\x08\x20\"type\"\n\n\x0c\n\
    \x05\x04\0\x02\x04\x06\x12\x03\"\x04\x0c\n\x0c\n\x05\x04\0\x02\x04\x01\
    \x12\x03\"\r\x11\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\"\x14\x15\n\x16\n\
    \x04\x04\0\x02\x05\x12\x03#\x04\x1f\"\t\x20\"shape\"\n\n\x0c\n\x05\x04\0\
    \x02\x05\x06\x12\x03#\x04\x14\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03#\x15\
    \x1a\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03#\x1d\x1e\n\x17\n\x04\x04\0\
    \x02\x06\x12\x03$\x04\x1b\"\n\x20\"tensor\"\n\n\x0c\n\x05\x04\0\x02\x06\
    \x06\x12\x03$\x04\x0f\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03$\x10\x16\n\
    \x0c\n\x05\x04\0\x02\x06\x03\x12\x03$\x19\x1a\n\x1e\n\x04\x04\0\x02\x07\
    \x12\x03%\x04\x17\"\x11\x20any\x20\"list(...)\"\n\n\x0c\n\x05\x04\0\x02\
    \x07\x06\x12\x03%\x04\r\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03%\x0e\x12\n\
    \x0c\n\x05\x04\0\x02\x07\x03\x12\x03%\x15\x16\n\xed\x01\n\x04\x04\0\x02\
    \x08\x12\x03+\x04\x1b\x1a\xdf\x01\x20\"func\"\x20represents\x20a\x20func\
    tion.\x20func.name\x20is\x20a\x20function's\x20name\x20or\n\x20a\x20prim\
    itive\x20op's\x20name.\x20func.attr.first\x20is\x20the\x20name\x20of\x20\
    an\x20attr\n\x20defined\x20for\x20that\x20function.\x20func.attr.second\
    \x20is\x20the\x20value\x20for\n\x20that\x20attr\x20in\x20the\x20instanti\
    ation.\n\n\x0c\n\x05\x04\0\x02\x08\x06\x12\x03+\x04\x10\n\x0c\n\x05\x04\
    \0\x02\x08\x01\x12\x03+\x11\x15\n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03+\
    \x18\x1a\n\x95\x03\n\x04\x04\0\x02\t\x12\x034\x04\x1b\x1a\x87\x03\x20Thi\
    s\x20is\x20a\x20placeholder\x20only\x20used\x20in\x20nodes\x20defined\
    \x20inside\x20a\n\x20function.\x20\x20It\x20indicates\x20the\x20attr\x20\
    value\x20will\x20be\x20supplied\x20when\n\x20the\x20function\x20is\x20in\
    stantiated.\x20\x20For\x20example,\x20let\x20us\x20suppose\x20a\n\x20nod\
    e\x20\"N\"\x20in\x20function\x20\"FN\".\x20\"N\"\x20has\x20an\x20attr\
    \x20\"A\"\x20with\x20value\n\x20placeholder\x20=\x20\"foo\".\x20When\x20\
    FN\x20is\x20instantiated\x20with\x20attr\x20\"foo\"\n\x20set\x20to\x20\"\
    bar\",\x20the\x20instantiated\x20node\x20N's\x20attr\x20A\x20will\x20hav\
    e\x20been\n\x20given\x20the\x20value\x20\"bar\".\n\n\x0c\n\x05\x04\0\x02\
    \t\x05\x12\x034\x04\n\n\x0c\n\x05\x04\0\x02\t\x01\x12\x034\x0b\x16\n\x0c\
    \n\x05\x04\0\x02\t\x03\x12\x034\x19\x1a\n|\n\x02\x04\x01\x12\x04:\0=\x01\
    \x1ap\x20A\x20list\x20of\x20attr\x20names\x20and\x20their\x20values.\x20\
    The\x20whole\x20list\x20is\x20attached\n\x20with\x20a\x20string\x20name.\
    \x20\x20E.g.,\x20MatMul[T=float].\n\n\n\n\x03\x04\x01\x01\x12\x03:\x08\
    \x14\n\x0b\n\x04\x04\x01\x02\0\x12\x03;\x02\x12\n\r\n\x05\x04\x01\x02\0\
    \x04\x12\x04;\x02:\x16\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03;\x02\x08\n\
    \x0c\n\x05\x04\x01\x02\0\x01\x12\x03;\t\r\n\x0c\n\x05\x04\x01\x02\0\x03\
    \x12\x03;\x10\x11\n\x0b\n\x04\x04\x01\x02\x01\x12\x03<\x02\"\n\r\n\x05\
    \x04\x01\x02\x01\x04\x12\x04<\x02;\x12\n\x0c\n\x05\x04\x01\x02\x01\x06\
    \x12\x03<\x02\x18\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03<\x19\x1d\n\x0c\
    \n\x05\x04\x01\x02\x01\x03\x12\x03<\x20!b\x06proto3\
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
