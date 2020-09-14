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
pub struct VersionDef {
    // message fields
    pub producer: i32,
    pub min_consumer: i32,
    pub bad_consumers: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VersionDef {}

impl VersionDef {
    pub fn new() -> VersionDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VersionDef {
        static mut instance: ::protobuf::lazy::Lazy<VersionDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VersionDef,
        };
        unsafe {
            instance.get(VersionDef::new)
        }
    }

    // int32 producer = 1;

    pub fn clear_producer(&mut self) {
        self.producer = 0;
    }

    // Param is passed by value, moved
    pub fn set_producer(&mut self, v: i32) {
        self.producer = v;
    }

    pub fn get_producer(&self) -> i32 {
        self.producer
    }

    fn get_producer_for_reflect(&self) -> &i32 {
        &self.producer
    }

    fn mut_producer_for_reflect(&mut self) -> &mut i32 {
        &mut self.producer
    }

    // int32 min_consumer = 2;

    pub fn clear_min_consumer(&mut self) {
        self.min_consumer = 0;
    }

    // Param is passed by value, moved
    pub fn set_min_consumer(&mut self, v: i32) {
        self.min_consumer = v;
    }

    pub fn get_min_consumer(&self) -> i32 {
        self.min_consumer
    }

    fn get_min_consumer_for_reflect(&self) -> &i32 {
        &self.min_consumer
    }

    fn mut_min_consumer_for_reflect(&mut self) -> &mut i32 {
        &mut self.min_consumer
    }

    // repeated int32 bad_consumers = 3;

    pub fn clear_bad_consumers(&mut self) {
        self.bad_consumers.clear();
    }

    // Param is passed by value, moved
    pub fn set_bad_consumers(&mut self, v: ::std::vec::Vec<i32>) {
        self.bad_consumers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_bad_consumers(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.bad_consumers
    }

    // Take field
    pub fn take_bad_consumers(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.bad_consumers, ::std::vec::Vec::new())
    }

    pub fn get_bad_consumers(&self) -> &[i32] {
        &self.bad_consumers
    }

    fn get_bad_consumers_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.bad_consumers
    }

    fn mut_bad_consumers_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.bad_consumers
    }
}

impl ::protobuf::Message for VersionDef {
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
                    self.producer = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.min_consumer = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.bad_consumers)?;
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
        if self.producer != 0 {
            my_size += ::protobuf::rt::value_size(1, self.producer, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.min_consumer != 0 {
            my_size += ::protobuf::rt::value_size(2, self.min_consumer, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.bad_consumers {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.producer != 0 {
            os.write_int32(1, self.producer)?;
        }
        if self.min_consumer != 0 {
            os.write_int32(2, self.min_consumer)?;
        }
        for v in &self.bad_consumers {
            os.write_int32(3, *v)?;
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

impl ::protobuf::MessageStatic for VersionDef {
    fn new() -> VersionDef {
        VersionDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<VersionDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "producer",
                    VersionDef::get_producer_for_reflect,
                    VersionDef::mut_producer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "min_consumer",
                    VersionDef::get_min_consumer_for_reflect,
                    VersionDef::mut_min_consumer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "bad_consumers",
                    VersionDef::get_bad_consumers_for_reflect,
                    VersionDef::mut_bad_consumers_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VersionDef>(
                    "VersionDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VersionDef {
    fn clear(&mut self) {
        self.clear_producer();
        self.clear_min_consumer();
        self.clear_bad_consumers();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VersionDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VersionDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(tensorflow/core/framework/versions.proto\x12\ntensorflow\"p\n\nVersio\
    nDef\x12\x1a\n\x08producer\x18\x01\x20\x01(\x05R\x08producer\x12!\n\x0cm\
    in_consumer\x18\x02\x20\x01(\x05R\x0bminConsumer\x12#\n\rbad_consumers\
    \x18\x03\x20\x03(\x05R\x0cbadConsumersB/\n\x18org.tensorflow.frameworkB\
    \x0eVersionsProtosP\x01\xf8\x01\x01J\xaf\t\n\x06\x12\x04\0\0\x1e\x02\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x12\n\x08\
    \n\x01\x08\x12\x03\x03\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x03\0\x1f\
    \n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x03\x07\x17\n\r\n\x06\x08\xe7\x07\
    \0\x02\0\x12\x03\x03\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\
    \x03\x07\x17\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x03\x1a\x1e\n\x08\n\
    \x01\x08\x12\x03\x04\0/\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x04\0/\n\x0c\
    \n\x05\x08\xe7\x07\x01\x02\x12\x03\x04\x07\x1b\n\r\n\x06\x08\xe7\x07\x01\
    \x02\0\x12\x03\x04\x07\x1b\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\
    \x04\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\x03\x04\x1e.\n\x08\n\
    \x01\x08\x12\x03\x05\0\"\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x05\0\"\n\
    \x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\
    \x02\x02\0\x12\x03\x05\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\
    \x03\x05\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x05\x1d!\n\x08\
    \n\x01\x08\x12\x03\x06\01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x06\01\n\
    \x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x06\x07\x13\n\r\n\x06\x08\xe7\x07\
    \x03\x02\0\x12\x03\x06\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\
    \x03\x06\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x06\x160\n\xa8\
    \x03\n\x02\x04\0\x12\x04\x15\0\x1e\x01\x1a\x9b\x03\x20Version\x20informa\
    tion\x20for\x20a\x20piece\x20of\x20serialized\x20data\n\n\x20There\x20ar\
    e\x20different\x20types\x20of\x20versions\x20for\x20each\x20type\x20of\
    \x20data\n\x20(GraphDef,\x20etc.),\x20but\x20they\x20all\x20have\x20the\
    \x20same\x20common\x20shape\n\x20described\x20here.\n\n\x20Each\x20consu\
    mer\x20has\x20\"consumer\"\x20and\x20\"min_producer\"\x20versions\x20(sp\
    ecified\n\x20elsewhere).\x20\x20A\x20consumer\x20is\x20allowed\x20to\x20\
    consume\x20this\x20data\x20if\n\n\x20\x20\x20producer\x20>=\x20min_produ\
    cer\n\x20\x20\x20consumer\x20>=\x20min_consumer\n\x20\x20\x20consumer\
    \x20not\x20in\x20bad_consumers\n\n\n\n\n\x03\x04\0\x01\x12\x03\x15\x08\
    \x12\n?\n\x04\x04\0\x02\0\x12\x03\x17\x02\x15\x1a2\x20The\x20version\x20\
    of\x20the\x20code\x20that\x20produced\x20this\x20data.\n\n\r\n\x05\x04\0\
    \x02\0\x04\x12\x04\x17\x02\x15\x14\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\
    \x17\x02\x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x17\x08\x10\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x17\x13\x14\nS\n\x04\x04\0\x02\x01\x12\x03\x1a\
    \x02\x19\x1aF\x20Any\x20consumer\x20below\x20this\x20version\x20is\x20no\
    t\x20allowed\x20to\x20consume\x20this\x20data.\n\n\r\n\x05\x04\0\x02\x01\
    \x04\x12\x04\x1a\x02\x17\x15\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x1a\
    \x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x1a\x08\x14\n\x0c\n\x05\
    \x04\0\x02\x01\x03\x12\x03\x1a\x17\x18\nR\n\x04\x04\0\x02\x02\x12\x03\
    \x1d\x02#\x1aE\x20Specific\x20consumer\x20versions\x20which\x20are\x20di\
    sallowed\x20(e.g.\x20due\x20to\x20bugs).\n\n\x0c\n\x05\x04\0\x02\x02\x04\
    \x12\x03\x1d\x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x1d\x0b\x10\n\
    \x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x1d\x11\x1e\n\x0c\n\x05\x04\0\x02\
    \x02\x03\x12\x03\x1d!\"b\x06proto3\
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
