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
pub struct JobDef {
    // message fields
    pub name: ::std::string::String,
    pub tasks: ::std::collections::HashMap<i32, ::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobDef {}

impl JobDef {
    pub fn new() -> JobDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobDef {
        static mut instance: ::protobuf::lazy::Lazy<JobDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobDef,
        };
        unsafe {
            instance.get(JobDef::new)
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

    // repeated .tensorflow.JobDef.TasksEntry tasks = 2;

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }

    // Param is passed by value, moved
    pub fn set_tasks(&mut self, v: ::std::collections::HashMap<i32, ::std::string::String>) {
        self.tasks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tasks(&mut self) -> &mut ::std::collections::HashMap<i32, ::std::string::String> {
        &mut self.tasks
    }

    // Take field
    pub fn take_tasks(&mut self) -> ::std::collections::HashMap<i32, ::std::string::String> {
        ::std::mem::replace(&mut self.tasks, ::std::collections::HashMap::new())
    }

    pub fn get_tasks(&self) -> &::std::collections::HashMap<i32, ::std::string::String> {
        &self.tasks
    }

    fn get_tasks_for_reflect(&self) -> &::std::collections::HashMap<i32, ::std::string::String> {
        &self.tasks
    }

    fn mut_tasks_for_reflect(&mut self) -> &mut ::std::collections::HashMap<i32, ::std::string::String> {
        &mut self.tasks
    }
}

impl ::protobuf::Message for JobDef {
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
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.tasks)?;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeString>(2, &self.tasks);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeString>(2, &self.tasks, os)?;
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

impl ::protobuf::MessageStatic for JobDef {
    fn new() -> JobDef {
        JobDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    JobDef::get_name_for_reflect,
                    JobDef::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeInt32, ::protobuf::types::ProtobufTypeString>(
                    "tasks",
                    JobDef::get_tasks_for_reflect,
                    JobDef::mut_tasks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobDef>(
                    "JobDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobDef {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_tasks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClusterDef {
    // message fields
    pub job: ::protobuf::RepeatedField<JobDef>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClusterDef {}

impl ClusterDef {
    pub fn new() -> ClusterDef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClusterDef {
        static mut instance: ::protobuf::lazy::Lazy<ClusterDef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClusterDef,
        };
        unsafe {
            instance.get(ClusterDef::new)
        }
    }

    // repeated .tensorflow.JobDef job = 1;

    pub fn clear_job(&mut self) {
        self.job.clear();
    }

    // Param is passed by value, moved
    pub fn set_job(&mut self, v: ::protobuf::RepeatedField<JobDef>) {
        self.job = v;
    }

    // Mutable pointer to the field.
    pub fn mut_job(&mut self) -> &mut ::protobuf::RepeatedField<JobDef> {
        &mut self.job
    }

    // Take field
    pub fn take_job(&mut self) -> ::protobuf::RepeatedField<JobDef> {
        ::std::mem::replace(&mut self.job, ::protobuf::RepeatedField::new())
    }

    pub fn get_job(&self) -> &[JobDef] {
        &self.job
    }

    fn get_job_for_reflect(&self) -> &::protobuf::RepeatedField<JobDef> {
        &self.job
    }

    fn mut_job_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<JobDef> {
        &mut self.job
    }
}

impl ::protobuf::Message for ClusterDef {
    fn is_initialized(&self) -> bool {
        for v in &self.job {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.job)?;
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
        for value in &self.job {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.job {
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

impl ::protobuf::MessageStatic for ClusterDef {
    fn new() -> ClusterDef {
        ClusterDef::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClusterDef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JobDef>>(
                    "job",
                    ClusterDef::get_job_for_reflect,
                    ClusterDef::mut_job_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClusterDef>(
                    "ClusterDef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClusterDef {
    fn clear(&mut self) {
        self.clear_job();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClusterDef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClusterDef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&tensorflow/core/protobuf/cluster.proto\x12\ntensorflow\"\x8b\x01\n\
    \x06JobDef\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x123\n\x05tasks\
    \x18\x02\x20\x03(\x0b2\x1d.tensorflow.JobDef.TasksEntryR\x05tasks\x1a8\n\
    \nTasksEntry\x12\x10\n\x03key\x18\x01\x20\x01(\x05R\x03key\x12\x14\n\x05\
    value\x18\x02\x20\x01(\tR\x05value:\x028\x01\"2\n\nClusterDef\x12$\n\x03\
    job\x18\x01\x20\x03(\x0b2\x12.tensorflow.JobDefR\x03jobB0\n\x1aorg.tenso\
    rflow.distruntimeB\rClusterProtosP\x01\xf8\x01\x01J\x9b\x18\n\x06\x12\
    \x04\x0f\0Q\x01\n\x9f\x05\n\x01\x0c\x12\x03\x0f\0\x122\x94\x05\x20Copyri\
    ght\x202016\x20The\x20TensorFlow\x20Authors.\x20All\x20Rights\x20Reserve\
    d.\n\nLicensed\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\
    \x20(the\x20\"License\");\nyou\x20may\x20not\x20use\x20this\x20file\x20e\
    xcept\x20in\x20compliance\x20with\x20the\x20License.\nYou\x20may\x20obta\
    in\x20a\x20copy\x20of\x20the\x20License\x20at\n\nhttp://www.apache.org/l\
    icenses/LICENSE-2.0\n\nUnless\x20required\x20by\x20applicable\x20law\x20\
    or\x20agreed\x20to\x20in\x20writing,\x20software\ndistributed\x20under\
    \x20the\x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20B\
    ASIS,\nWITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\
    \x20either\x20express\x20or\x20implied.\nSee\x20the\x20License\x20for\
    \x20the\x20specific\x20language\x20governing\x20permissions\x20and\nlimi\
    tations\x20under\x20the\x20License.\n===================================\
    ===========================================\n\x08\n\x01\x02\x12\x03\x11\
    \x08\x12\n\x08\n\x01\x08\x12\x03\x12\0\x1f\n\x0b\n\x04\x08\xe7\x07\0\x12\
    \x03\x12\0\x1f\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x12\x07\x17\n\r\n\
    \x06\x08\xe7\x07\0\x02\0\x12\x03\x12\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\
    \x02\0\x01\x12\x03\x12\x07\x17\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x12\
    \x1a\x1e\n\x08\n\x01\x08\x12\x03\x13\0.\n\x0b\n\x04\x08\xe7\x07\x01\x12\
    \x03\x13\0.\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x13\x07\x1b\n\r\n\
    \x06\x08\xe7\x07\x01\x02\0\x12\x03\x13\x07\x1b\n\x0e\n\x07\x08\xe7\x07\
    \x01\x02\0\x01\x12\x03\x13\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x01\x07\x12\
    \x03\x13\x1e-\n\x08\n\x01\x08\x12\x03\x14\0\"\n\x0b\n\x04\x08\xe7\x07\
    \x02\x12\x03\x14\0\"\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x14\x07\x1a\
    \n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x14\x07\x1a\n\x0e\n\x07\x08\xe7\
    \x07\x02\x02\0\x01\x12\x03\x14\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x02\x03\
    \x12\x03\x14\x1d!\n\x08\n\x01\x08\x12\x03\x15\03\n\x0b\n\x04\x08\xe7\x07\
    \x03\x12\x03\x15\03\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x15\x07\x13\
    \n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x15\x07\x13\n\x0e\n\x07\x08\xe7\
    \x07\x03\x02\0\x01\x12\x03\x15\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\
    \x12\x03\x15\x162\n\xbc\x0b\n\x02\x04\0\x12\x04A\0K\x01\x1a/\x20Defines\
    \x20a\x20single\x20job\x20in\x20a\x20TensorFlow\x20cluster.\n2\xfe\n\x20\
    This\x20file\x20contains\x20protos\x20to\x20be\x20used\x20when\x20defini\
    ng\x20a\x20TensorFlow\n\x20cluster.\n\n\x20EXAMPLES\n\x20--------\n\n\
    \x201.\x20A\x20single-process\x20cluster,\x20containing\x20\"/job:local/\
    task:0\".\n\n\x20\x20\x20\x20Cluster:\n\x20\x20\x20\x20\x20\x20job\x20{\
    \x20name:\x20'local'\x20tasks\x20{\x20key:\x200\x20value:\x20'localhost:\
    2222'\x20}\x20}\n\n\x20\x20\x20\x20Server:\n\x20\x20\x20\x20\x20\x20clus\
    ter\x20{\x20$CLUSTER\x20}\x20job_name:\x20'local'\x20task_index:\x200\n\
    \n\x202.\x20A\x20two-process\x20cluster,\x20containing\x20\"/job:local/t\
    ask:{0,1}\".\n\n\x20\x20\x20\x20Cluster:\n\x20\x20\x20\x20\x20\x20job\
    \x20{\x20name:\x20'local'\x20tasks\x20{\x20key:\x200\x20value:\x20'local\
    host:2222'\x20}\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20tasks\x20{\x20key:\
    \x201\x20value:\x20'localhost:2223'\x20}\x20}\n\n\x20\x20\x20\x20Servers\
    :\n\x20\x20\x20\x20\x20\x20cluster\x20{\x20$CLUSTER\x20}\x20job_name:\
    \x20'local'\x20task_index:\x200\n\x20\x20\x20\x20\x20\x20cluster\x20{\
    \x20$CLUSTER\x20}\x20job_name:\x20'local'\x20task_index:\x201\n\n\x203.\
    \x20A\x20two-job\x20cluster,\x20containing\x20\"/job:worker/task:{0,1,2}\
    \"\x20and\n\x20\x20\x20\x20\"/job:ps/task:{0,1}\".\n\n\x20\x20\x20\x20Cl\
    uster:\n\x20\x20\x20\x20\x20\x20job\x20{\x20name:\x20'worker'\x20tasks\
    \x20{\x20key:\x200\x20value:\x20'worker1:2222'\x20}\n\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20tasks\x20{\x20key:\x201\x20value:\x20'worker2:2222'\
    \x20}\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20tasks\x20{\x20key:\x202\x20v\
    alue:\x20'worker3:2222'\x20}\x20}\n\x20\x20\x20\x20\x20\x20job\x20{\x20n\
    ame:\x20'ps'\x20\x20\x20\x20\x20tasks\x20{\x20key:\x200\x20value:\x20'ps\
    0:2222'\x20}\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20tasks\x20{\x20key:\
    \x201\x20value:\x20'ps1:2222'\x20}\x20}\n\n\x20\x20\x20\x20Servers:\n\
    \x20\x20\x20\x20\x20\x20cluster\x20{\x20$CLUSTER\x20}\x20job_name:\x20'w\
    orker'\x20task_index:\x200\n\x20\x20\x20\x20\x20\x20cluster\x20{\x20$CLU\
    STER\x20}\x20job_name:\x20'worker'\x20task_index:\x201\n\x20\x20\x20\x20\
    \x20\x20cluster\x20{\x20$CLUSTER\x20}\x20job_name:\x20'worker'\x20task_i\
    ndex:\x202\n\x20\x20\x20\x20\x20\x20cluster\x20{\x20$CLUSTER\x20}\x20job\
    _name:\x20'ps'\x20\x20\x20\x20\x20task_index:\x200\n\x20\x20\x20\x20\x20\
    \x20cluster\x20{\x20$CLUSTER\x20}\x20job_name:\x20'ps'\x20\x20\x20\x20\
    \x20task_index:\x201\n\n\n\n\x03\x04\0\x01\x12\x03A\x08\x0e\n$\n\x04\x04\
    \0\x02\0\x12\x03C\x02\x12\x1a\x17\x20The\x20name\x20of\x20this\x20job.\n\
    \n\r\n\x05\x04\0\x02\0\x04\x12\x04C\x02A\x10\n\x0c\n\x05\x04\0\x02\0\x05\
    \x12\x03C\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03C\t\r\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03C\x10\x11\n\x83\x02\n\x04\x04\0\x02\x01\x12\x03J\
    \x02\x1f\x1a\xf5\x01\x20Mapping\x20from\x20task\x20ID\x20to\x20\"hostnam\
    e:port\"\x20string.\n\n\x20If\x20the\x20`name`\x20field\x20contains\x20\
    \"worker\",\x20and\x20the\x20`tasks`\x20map\x20contains\x20a\n\x20mappin\
    g\x20from\x207\x20to\x20\"example.org:2222\",\x20then\x20the\x20device\
    \x20prefix\n\x20\"/job:worker/task:7\"\x20will\x20be\x20assigned\x20to\
    \x20\"example.org:2222\".\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04J\x02C\
    \x12\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03J\x02\x14\n\x0c\n\x05\x04\0\
    \x02\x01\x01\x12\x03J\x15\x1a\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03J\x1d\
    \x1e\n<\n\x02\x04\x01\x12\x04N\0Q\x01\x1a0\x20Defines\x20a\x20TensorFlow\
    \x20cluster\x20as\x20a\x20set\x20of\x20jobs.\n\n\n\n\x03\x04\x01\x01\x12\
    \x03N\x08\x12\n2\n\x04\x04\x01\x02\0\x12\x03P\x02\x1a\x1a%\x20The\x20job\
    s\x20that\x20comprise\x20the\x20cluster.\n\n\x0c\n\x05\x04\x01\x02\0\x04\
    \x12\x03P\x02\n\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03P\x0b\x11\n\x0c\n\
    \x05\x04\x01\x02\0\x01\x12\x03P\x12\x15\n\x0c\n\x05\x04\x01\x02\0\x03\
    \x12\x03P\x18\x19b\x06proto3\
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
