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

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DataType {
    DT_INVALID = 0,
    DT_FLOAT = 1,
    DT_DOUBLE = 2,
    DT_INT32 = 3,
    DT_UINT8 = 4,
    DT_INT16 = 5,
    DT_INT8 = 6,
    DT_STRING = 7,
    DT_COMPLEX64 = 8,
    DT_INT64 = 9,
    DT_BOOL = 10,
    DT_QINT8 = 11,
    DT_QUINT8 = 12,
    DT_QINT32 = 13,
    DT_BFLOAT16 = 14,
    DT_QINT16 = 15,
    DT_QUINT16 = 16,
    DT_UINT16 = 17,
    DT_COMPLEX128 = 18,
    DT_HALF = 19,
    DT_RESOURCE = 20,
    DT_VARIANT = 21,
    DT_FLOAT_REF = 101,
    DT_DOUBLE_REF = 102,
    DT_INT32_REF = 103,
    DT_UINT8_REF = 104,
    DT_INT16_REF = 105,
    DT_INT8_REF = 106,
    DT_STRING_REF = 107,
    DT_COMPLEX64_REF = 108,
    DT_INT64_REF = 109,
    DT_BOOL_REF = 110,
    DT_QINT8_REF = 111,
    DT_QUINT8_REF = 112,
    DT_QINT32_REF = 113,
    DT_BFLOAT16_REF = 114,
    DT_QINT16_REF = 115,
    DT_QUINT16_REF = 116,
    DT_UINT16_REF = 117,
    DT_COMPLEX128_REF = 118,
    DT_HALF_REF = 119,
    DT_RESOURCE_REF = 120,
    DT_VARIANT_REF = 121,
}

impl ::protobuf::ProtobufEnum for DataType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DataType> {
        match value {
            0 => ::std::option::Option::Some(DataType::DT_INVALID),
            1 => ::std::option::Option::Some(DataType::DT_FLOAT),
            2 => ::std::option::Option::Some(DataType::DT_DOUBLE),
            3 => ::std::option::Option::Some(DataType::DT_INT32),
            4 => ::std::option::Option::Some(DataType::DT_UINT8),
            5 => ::std::option::Option::Some(DataType::DT_INT16),
            6 => ::std::option::Option::Some(DataType::DT_INT8),
            7 => ::std::option::Option::Some(DataType::DT_STRING),
            8 => ::std::option::Option::Some(DataType::DT_COMPLEX64),
            9 => ::std::option::Option::Some(DataType::DT_INT64),
            10 => ::std::option::Option::Some(DataType::DT_BOOL),
            11 => ::std::option::Option::Some(DataType::DT_QINT8),
            12 => ::std::option::Option::Some(DataType::DT_QUINT8),
            13 => ::std::option::Option::Some(DataType::DT_QINT32),
            14 => ::std::option::Option::Some(DataType::DT_BFLOAT16),
            15 => ::std::option::Option::Some(DataType::DT_QINT16),
            16 => ::std::option::Option::Some(DataType::DT_QUINT16),
            17 => ::std::option::Option::Some(DataType::DT_UINT16),
            18 => ::std::option::Option::Some(DataType::DT_COMPLEX128),
            19 => ::std::option::Option::Some(DataType::DT_HALF),
            20 => ::std::option::Option::Some(DataType::DT_RESOURCE),
            21 => ::std::option::Option::Some(DataType::DT_VARIANT),
            101 => ::std::option::Option::Some(DataType::DT_FLOAT_REF),
            102 => ::std::option::Option::Some(DataType::DT_DOUBLE_REF),
            103 => ::std::option::Option::Some(DataType::DT_INT32_REF),
            104 => ::std::option::Option::Some(DataType::DT_UINT8_REF),
            105 => ::std::option::Option::Some(DataType::DT_INT16_REF),
            106 => ::std::option::Option::Some(DataType::DT_INT8_REF),
            107 => ::std::option::Option::Some(DataType::DT_STRING_REF),
            108 => ::std::option::Option::Some(DataType::DT_COMPLEX64_REF),
            109 => ::std::option::Option::Some(DataType::DT_INT64_REF),
            110 => ::std::option::Option::Some(DataType::DT_BOOL_REF),
            111 => ::std::option::Option::Some(DataType::DT_QINT8_REF),
            112 => ::std::option::Option::Some(DataType::DT_QUINT8_REF),
            113 => ::std::option::Option::Some(DataType::DT_QINT32_REF),
            114 => ::std::option::Option::Some(DataType::DT_BFLOAT16_REF),
            115 => ::std::option::Option::Some(DataType::DT_QINT16_REF),
            116 => ::std::option::Option::Some(DataType::DT_QUINT16_REF),
            117 => ::std::option::Option::Some(DataType::DT_UINT16_REF),
            118 => ::std::option::Option::Some(DataType::DT_COMPLEX128_REF),
            119 => ::std::option::Option::Some(DataType::DT_HALF_REF),
            120 => ::std::option::Option::Some(DataType::DT_RESOURCE_REF),
            121 => ::std::option::Option::Some(DataType::DT_VARIANT_REF),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DataType] = &[
            DataType::DT_INVALID,
            DataType::DT_FLOAT,
            DataType::DT_DOUBLE,
            DataType::DT_INT32,
            DataType::DT_UINT8,
            DataType::DT_INT16,
            DataType::DT_INT8,
            DataType::DT_STRING,
            DataType::DT_COMPLEX64,
            DataType::DT_INT64,
            DataType::DT_BOOL,
            DataType::DT_QINT8,
            DataType::DT_QUINT8,
            DataType::DT_QINT32,
            DataType::DT_BFLOAT16,
            DataType::DT_QINT16,
            DataType::DT_QUINT16,
            DataType::DT_UINT16,
            DataType::DT_COMPLEX128,
            DataType::DT_HALF,
            DataType::DT_RESOURCE,
            DataType::DT_VARIANT,
            DataType::DT_FLOAT_REF,
            DataType::DT_DOUBLE_REF,
            DataType::DT_INT32_REF,
            DataType::DT_UINT8_REF,
            DataType::DT_INT16_REF,
            DataType::DT_INT8_REF,
            DataType::DT_STRING_REF,
            DataType::DT_COMPLEX64_REF,
            DataType::DT_INT64_REF,
            DataType::DT_BOOL_REF,
            DataType::DT_QINT8_REF,
            DataType::DT_QUINT8_REF,
            DataType::DT_QINT32_REF,
            DataType::DT_BFLOAT16_REF,
            DataType::DT_QINT16_REF,
            DataType::DT_QUINT16_REF,
            DataType::DT_UINT16_REF,
            DataType::DT_COMPLEX128_REF,
            DataType::DT_HALF_REF,
            DataType::DT_RESOURCE_REF,
            DataType::DT_VARIANT_REF,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DataType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DataType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DataType {
}

impl ::std::default::Default for DataType {
    fn default() -> Self {
        DataType::DT_INVALID
    }
}

impl ::protobuf::reflect::ProtobufValue for DataType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%tensorflow/core/framework/types.proto\x12\ntensorflow*\xe6\x05\n\x08D\
    ataType\x12\x0e\n\nDT_INVALID\x10\0\x12\x0c\n\x08DT_FLOAT\x10\x01\x12\r\
    \n\tDT_DOUBLE\x10\x02\x12\x0c\n\x08DT_INT32\x10\x03\x12\x0c\n\x08DT_UINT\
    8\x10\x04\x12\x0c\n\x08DT_INT16\x10\x05\x12\x0b\n\x07DT_INT8\x10\x06\x12\
    \r\n\tDT_STRING\x10\x07\x12\x10\n\x0cDT_COMPLEX64\x10\x08\x12\x0c\n\x08D\
    T_INT64\x10\t\x12\x0b\n\x07DT_BOOL\x10\n\x12\x0c\n\x08DT_QINT8\x10\x0b\
    \x12\r\n\tDT_QUINT8\x10\x0c\x12\r\n\tDT_QINT32\x10\r\x12\x0f\n\x0bDT_BFL\
    OAT16\x10\x0e\x12\r\n\tDT_QINT16\x10\x0f\x12\x0e\n\nDT_QUINT16\x10\x10\
    \x12\r\n\tDT_UINT16\x10\x11\x12\x11\n\rDT_COMPLEX128\x10\x12\x12\x0b\n\
    \x07DT_HALF\x10\x13\x12\x0f\n\x0bDT_RESOURCE\x10\x14\x12\x0e\n\nDT_VARIA\
    NT\x10\x15\x12\x10\n\x0cDT_FLOAT_REF\x10e\x12\x11\n\rDT_DOUBLE_REF\x10f\
    \x12\x10\n\x0cDT_INT32_REF\x10g\x12\x10\n\x0cDT_UINT8_REF\x10h\x12\x10\n\
    \x0cDT_INT16_REF\x10i\x12\x0f\n\x0bDT_INT8_REF\x10j\x12\x11\n\rDT_STRING\
    _REF\x10k\x12\x14\n\x10DT_COMPLEX64_REF\x10l\x12\x10\n\x0cDT_INT64_REF\
    \x10m\x12\x0f\n\x0bDT_BOOL_REF\x10n\x12\x10\n\x0cDT_QINT8_REF\x10o\x12\
    \x11\n\rDT_QUINT8_REF\x10p\x12\x11\n\rDT_QINT32_REF\x10q\x12\x13\n\x0fDT\
    _BFLOAT16_REF\x10r\x12\x11\n\rDT_QINT16_REF\x10s\x12\x12\n\x0eDT_QUINT16\
    _REF\x10t\x12\x11\n\rDT_UINT16_REF\x10u\x12\x15\n\x11DT_COMPLEX128_REF\
    \x10v\x12\x0f\n\x0bDT_HALF_REF\x10w\x12\x13\n\x0fDT_RESOURCE_REF\x10x\
    \x12\x12\n\x0eDT_VARIANT_REF\x10yB,\n\x18org.tensorflow.frameworkB\x0bTy\
    pesProtosP\x01\xf8\x01\x01J\xdf\x15\n\x06\x12\x04\0\0@\x01\n\x08\n\x01\
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
    \x06\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\x1b\n\
    \x02\x05\0\x12\x04\t\0@\x01\x1a\x0f\x20LINT.IfChange\n\n\n\n\x03\x05\0\
    \x01\x12\x03\t\x05\r\nd\n\x04\x05\0\x02\0\x12\x03\x0c\x02\x11\x1aW\x20No\
    t\x20a\x20legal\x20value\x20for\x20DataType.\x20\x20Used\x20to\x20indica\
    te\x20a\x20DataType\x20field\n\x20has\x20not\x20been\x20set.\n\n\x0c\n\
    \x05\x05\0\x02\0\x01\x12\x03\x0c\x02\x0c\n\x0c\n\x05\x05\0\x02\0\x02\x12\
    \x03\x0c\x0f\x10\n^\n\x04\x05\0\x02\x01\x12\x03\x10\x02\x0f\x1aQ\x20Data\
    \x20types\x20that\x20all\x20computation\x20devices\x20are\x20expected\
    \x20to\x20be\n\x20capable\x20to\x20support.\n\n\x0c\n\x05\x05\0\x02\x01\
    \x01\x12\x03\x10\x02\n\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x10\r\x0e\n\
    \x0b\n\x04\x05\0\x02\x02\x12\x03\x11\x02\x10\n\x0c\n\x05\x05\0\x02\x02\
    \x01\x12\x03\x11\x02\x0b\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x11\x0e\
    \x0f\n\x0b\n\x04\x05\0\x02\x03\x12\x03\x12\x02\x0f\n\x0c\n\x05\x05\0\x02\
    \x03\x01\x12\x03\x12\x02\n\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\x12\r\
    \x0e\n\x0b\n\x04\x05\0\x02\x04\x12\x03\x13\x02\x0f\n\x0c\n\x05\x05\0\x02\
    \x04\x01\x12\x03\x13\x02\n\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x13\r\
    \x0e\n\x0b\n\x04\x05\0\x02\x05\x12\x03\x14\x02\x0f\n\x0c\n\x05\x05\0\x02\
    \x05\x01\x12\x03\x14\x02\n\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03\x14\r\
    \x0e\n\x0b\n\x04\x05\0\x02\x06\x12\x03\x15\x02\x0e\n\x0c\n\x05\x05\0\x02\
    \x06\x01\x12\x03\x15\x02\t\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03\x15\x0c\
    \r\n\x0b\n\x04\x05\0\x02\x07\x12\x03\x16\x02\x10\n\x0c\n\x05\x05\0\x02\
    \x07\x01\x12\x03\x16\x02\x0b\n\x0c\n\x05\x05\0\x02\x07\x02\x12\x03\x16\
    \x0e\x0f\n'\n\x04\x05\0\x02\x08\x12\x03\x17\x02\x13\"\x1a\x20Single-prec\
    ision\x20complex\n\n\x0c\n\x05\x05\0\x02\x08\x01\x12\x03\x17\x02\x0e\n\
    \x0c\n\x05\x05\0\x02\x08\x02\x12\x03\x17\x11\x12\n\x0b\n\x04\x05\0\x02\t\
    \x12\x03\x18\x02\x0f\n\x0c\n\x05\x05\0\x02\t\x01\x12\x03\x18\x02\n\n\x0c\
    \n\x05\x05\0\x02\t\x02\x12\x03\x18\r\x0e\n\x0b\n\x04\x05\0\x02\n\x12\x03\
    \x19\x02\x0f\n\x0c\n\x05\x05\0\x02\n\x01\x12\x03\x19\x02\t\n\x0c\n\x05\
    \x05\0\x02\n\x02\x12\x03\x19\x0c\x0e\n\x1d\n\x04\x05\0\x02\x0b\x12\x03\
    \x1a\x02\x10\"\x10\x20Quantized\x20int8\n\n\x0c\n\x05\x05\0\x02\x0b\x01\
    \x12\x03\x1a\x02\n\n\x0c\n\x05\x05\0\x02\x0b\x02\x12\x03\x1a\r\x0f\n\x1e\
    \n\x04\x05\0\x02\x0c\x12\x03\x1b\x02\x11\"\x11\x20Quantized\x20uint8\n\n\
    \x0c\n\x05\x05\0\x02\x0c\x01\x12\x03\x1b\x02\x0b\n\x0c\n\x05\x05\0\x02\
    \x0c\x02\x12\x03\x1b\x0e\x10\n\x1e\n\x04\x05\0\x02\r\x12\x03\x1c\x02\x11\
    \"\x11\x20Quantized\x20int32\n\n\x0c\n\x05\x05\0\x02\r\x01\x12\x03\x1c\
    \x02\x0b\n\x0c\n\x05\x05\0\x02\r\x02\x12\x03\x1c\x0e\x10\n@\n\x04\x05\0\
    \x02\x0e\x12\x03\x1d\x02\x13\"3\x20Float32\x20truncated\x20to\x2016\x20b\
    its.\x20\x20Only\x20for\x20cast\x20ops.\n\n\x0c\n\x05\x05\0\x02\x0e\x01\
    \x12\x03\x1d\x02\r\n\x0c\n\x05\x05\0\x02\x0e\x02\x12\x03\x1d\x10\x12\n\
    \x1e\n\x04\x05\0\x02\x0f\x12\x03\x1e\x02\x11\"\x11\x20Quantized\x20int16\
    \n\n\x0c\n\x05\x05\0\x02\x0f\x01\x12\x03\x1e\x02\x0b\n\x0c\n\x05\x05\0\
    \x02\x0f\x02\x12\x03\x1e\x0e\x10\n\x1f\n\x04\x05\0\x02\x10\x12\x03\x1f\
    \x02\x12\"\x12\x20Quantized\x20uint16\n\n\x0c\n\x05\x05\0\x02\x10\x01\
    \x12\x03\x1f\x02\x0c\n\x0c\n\x05\x05\0\x02\x10\x02\x12\x03\x1f\x0f\x11\n\
    \x0b\n\x04\x05\0\x02\x11\x12\x03\x20\x02\x11\n\x0c\n\x05\x05\0\x02\x11\
    \x01\x12\x03\x20\x02\x0b\n\x0c\n\x05\x05\0\x02\x11\x02\x12\x03\x20\x0e\
    \x10\n'\n\x04\x05\0\x02\x12\x12\x03!\x02\x15\"\x1a\x20Double-precision\
    \x20complex\n\n\x0c\n\x05\x05\0\x02\x12\x01\x12\x03!\x02\x0f\n\x0c\n\x05\
    \x05\0\x02\x12\x02\x12\x03!\x12\x14\n\x0b\n\x04\x05\0\x02\x13\x12\x03\"\
    \x02\x0f\n\x0c\n\x05\x05\0\x02\x13\x01\x12\x03\"\x02\t\n\x0c\n\x05\x05\0\
    \x02\x13\x02\x12\x03\"\x0c\x0e\n\x0b\n\x04\x05\0\x02\x14\x12\x03#\x02\
    \x13\n\x0c\n\x05\x05\0\x02\x14\x01\x12\x03#\x02\r\n\x0c\n\x05\x05\0\x02\
    \x14\x02\x12\x03#\x10\x12\n'\n\x04\x05\0\x02\x15\x12\x03$\x02\x12\"\x1a\
    \x20Arbitrary\x20C++\x20data\x20types\n\n\x0c\n\x05\x05\0\x02\x15\x01\
    \x12\x03$\x02\x0c\n\x0c\n\x05\x05\0\x02\x15\x02\x12\x03$\x0f\x11\n\xe5\
    \x01\n\x04\x05\0\x02\x16\x12\x03+\x02\x15\x1a\x82\x01\x20Do\x20not\x20us\
    e!\x20\x20These\x20are\x20only\x20for\x20parameters.\x20\x20Every\x20enu\
    m\x20above\n\x20should\x20have\x20a\x20corresponding\x20value\x20below\
    \x20(verified\x20by\x20types_test).\n2S\x20TODO(josh11b):\x20DT_GENERIC_\
    PROTO\x20=\x20??;\n\x20TODO(jeff,josh11b):\x20DT_UINT64?\x20\x20DT_UINT3\
    2?\n\n\x0c\n\x05\x05\0\x02\x16\x01\x12\x03+\x02\x0e\n\x0c\n\x05\x05\0\
    \x02\x16\x02\x12\x03+\x11\x14\n\x0b\n\x04\x05\0\x02\x17\x12\x03,\x02\x16\
    \n\x0c\n\x05\x05\0\x02\x17\x01\x12\x03,\x02\x0f\n\x0c\n\x05\x05\0\x02\
    \x17\x02\x12\x03,\x12\x15\n\x0b\n\x04\x05\0\x02\x18\x12\x03-\x02\x15\n\
    \x0c\n\x05\x05\0\x02\x18\x01\x12\x03-\x02\x0e\n\x0c\n\x05\x05\0\x02\x18\
    \x02\x12\x03-\x11\x14\n\x0b\n\x04\x05\0\x02\x19\x12\x03.\x02\x15\n\x0c\n\
    \x05\x05\0\x02\x19\x01\x12\x03.\x02\x0e\n\x0c\n\x05\x05\0\x02\x19\x02\
    \x12\x03.\x11\x14\n\x0b\n\x04\x05\0\x02\x1a\x12\x03/\x02\x15\n\x0c\n\x05\
    \x05\0\x02\x1a\x01\x12\x03/\x02\x0e\n\x0c\n\x05\x05\0\x02\x1a\x02\x12\
    \x03/\x11\x14\n\x0b\n\x04\x05\0\x02\x1b\x12\x030\x02\x14\n\x0c\n\x05\x05\
    \0\x02\x1b\x01\x12\x030\x02\r\n\x0c\n\x05\x05\0\x02\x1b\x02\x12\x030\x10\
    \x13\n\x0b\n\x04\x05\0\x02\x1c\x12\x031\x02\x16\n\x0c\n\x05\x05\0\x02\
    \x1c\x01\x12\x031\x02\x0f\n\x0c\n\x05\x05\0\x02\x1c\x02\x12\x031\x12\x15\
    \n\x0b\n\x04\x05\0\x02\x1d\x12\x032\x02\x19\n\x0c\n\x05\x05\0\x02\x1d\
    \x01\x12\x032\x02\x12\n\x0c\n\x05\x05\0\x02\x1d\x02\x12\x032\x15\x18\n\
    \x0b\n\x04\x05\0\x02\x1e\x12\x033\x02\x15\n\x0c\n\x05\x05\0\x02\x1e\x01\
    \x12\x033\x02\x0e\n\x0c\n\x05\x05\0\x02\x1e\x02\x12\x033\x11\x14\n\x0b\n\
    \x04\x05\0\x02\x1f\x12\x034\x02\x14\n\x0c\n\x05\x05\0\x02\x1f\x01\x12\
    \x034\x02\r\n\x0c\n\x05\x05\0\x02\x1f\x02\x12\x034\x10\x13\n\x0b\n\x04\
    \x05\0\x02\x20\x12\x035\x02\x15\n\x0c\n\x05\x05\0\x02\x20\x01\x12\x035\
    \x02\x0e\n\x0c\n\x05\x05\0\x02\x20\x02\x12\x035\x11\x14\n\x0b\n\x04\x05\
    \0\x02!\x12\x036\x02\x16\n\x0c\n\x05\x05\0\x02!\x01\x12\x036\x02\x0f\n\
    \x0c\n\x05\x05\0\x02!\x02\x12\x036\x12\x15\n\x0b\n\x04\x05\0\x02\"\x12\
    \x037\x02\x16\n\x0c\n\x05\x05\0\x02\"\x01\x12\x037\x02\x0f\n\x0c\n\x05\
    \x05\0\x02\"\x02\x12\x037\x12\x15\n\x0b\n\x04\x05\0\x02#\x12\x038\x02\
    \x18\n\x0c\n\x05\x05\0\x02#\x01\x12\x038\x02\x11\n\x0c\n\x05\x05\0\x02#\
    \x02\x12\x038\x14\x17\n\x0b\n\x04\x05\0\x02$\x12\x039\x02\x16\n\x0c\n\
    \x05\x05\0\x02$\x01\x12\x039\x02\x0f\n\x0c\n\x05\x05\0\x02$\x02\x12\x039\
    \x12\x15\n\x0b\n\x04\x05\0\x02%\x12\x03:\x02\x17\n\x0c\n\x05\x05\0\x02%\
    \x01\x12\x03:\x02\x10\n\x0c\n\x05\x05\0\x02%\x02\x12\x03:\x13\x16\n\x0b\
    \n\x04\x05\0\x02&\x12\x03;\x02\x16\n\x0c\n\x05\x05\0\x02&\x01\x12\x03;\
    \x02\x0f\n\x0c\n\x05\x05\0\x02&\x02\x12\x03;\x12\x15\n\x0b\n\x04\x05\0\
    \x02'\x12\x03<\x02\x1a\n\x0c\n\x05\x05\0\x02'\x01\x12\x03<\x02\x13\n\x0c\
    \n\x05\x05\0\x02'\x02\x12\x03<\x16\x19\n\x0b\n\x04\x05\0\x02(\x12\x03=\
    \x02\x14\n\x0c\n\x05\x05\0\x02(\x01\x12\x03=\x02\r\n\x0c\n\x05\x05\0\x02\
    (\x02\x12\x03=\x10\x13\n\x0b\n\x04\x05\0\x02)\x12\x03>\x02\x18\n\x0c\n\
    \x05\x05\0\x02)\x01\x12\x03>\x02\x11\n\x0c\n\x05\x05\0\x02)\x02\x12\x03>\
    \x14\x17\n\x0b\n\x04\x05\0\x02*\x12\x03?\x02\x17\n\x0c\n\x05\x05\0\x02*\
    \x01\x12\x03?\x02\x10\n\x0c\n\x05\x05\0\x02*\x02\x12\x03?\x13\x16b\x06pr\
    oto3\
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
