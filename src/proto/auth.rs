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
pub struct User {
    // message fields
    pub name: ::std::vec::Vec<u8>,
    pub password: ::std::vec::Vec<u8>,
    roles: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for User {}

impl User {
    pub fn new() -> User {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static User {
        static mut instance: ::protobuf::lazy::Lazy<User> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const User,
        };
        unsafe {
            instance.get(User::new)
        }
    }

    // bytes name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.name, ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.name
    }

    // bytes password = 2;

    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::vec::Vec<u8>) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.password, ::std::vec::Vec::new())
    }

    pub fn get_password(&self) -> &[u8] {
        &self.password
    }

    fn get_password_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.password
    }

    fn mut_password_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.password
    }

    // repeated string roles = 3;

    pub fn clear_roles(&mut self) {
        self.roles.clear();
    }

    // Param is passed by value, moved
    pub fn set_roles(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.roles = v;
    }

    // Mutable pointer to the field.
    pub fn mut_roles(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.roles
    }

    // Take field
    pub fn take_roles(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.roles, ::protobuf::RepeatedField::new())
    }

    pub fn get_roles(&self) -> &[::std::string::String] {
        &self.roles
    }

    fn get_roles_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.roles
    }

    fn mut_roles_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.roles
    }
}

impl ::protobuf::Message for User {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.password)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.roles)?;
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
            my_size += ::protobuf::rt::bytes_size(1, &self.name);
        };
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.password);
        };
        for value in &self.roles {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_bytes(1, &self.name)?;
        };
        if !self.password.is_empty() {
            os.write_bytes(2, &self.password)?;
        };
        for v in &self.roles {
            os.write_string(3, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for User {
    fn new() -> User {
        User::new()
    }

    fn descriptor_static(_: ::std::option::Option<User>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    User::get_name_for_reflect,
                    User::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "password",
                    User::get_password_for_reflect,
                    User::mut_password_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "roles",
                    User::get_roles_for_reflect,
                    User::mut_roles_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<User>(
                    "User",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for User {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_password();
        self.clear_roles();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for User {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for User {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Permission {
    // message fields
    pub permType: Permission_Type,
    pub key: ::std::vec::Vec<u8>,
    pub range_end: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Permission {}

impl Permission {
    pub fn new() -> Permission {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Permission {
        static mut instance: ::protobuf::lazy::Lazy<Permission> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Permission,
        };
        unsafe {
            instance.get(Permission::new)
        }
    }

    // .authpb.Permission.Type permType = 1;

    pub fn clear_permType(&mut self) {
        self.permType = Permission_Type::READ;
    }

    // Param is passed by value, moved
    pub fn set_permType(&mut self, v: Permission_Type) {
        self.permType = v;
    }

    pub fn get_permType(&self) -> Permission_Type {
        self.permType
    }

    fn get_permType_for_reflect(&self) -> &Permission_Type {
        &self.permType
    }

    fn mut_permType_for_reflect(&mut self) -> &mut Permission_Type {
        &mut self.permType
    }

    // bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // bytes range_end = 3;

    pub fn clear_range_end(&mut self) {
        self.range_end.clear();
    }

    // Param is passed by value, moved
    pub fn set_range_end(&mut self, v: ::std::vec::Vec<u8>) {
        self.range_end = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range_end(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.range_end
    }

    // Take field
    pub fn take_range_end(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.range_end, ::std::vec::Vec::new())
    }

    pub fn get_range_end(&self) -> &[u8] {
        &self.range_end
    }

    fn get_range_end_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.range_end
    }

    fn mut_range_end_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.range_end
    }
}

impl ::protobuf::Message for Permission {
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
                    };
                    let tmp = is.read_enum()?;
                    self.permType = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.range_end)?;
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
        if self.permType != Permission_Type::READ {
            my_size += ::protobuf::rt::enum_size(1, self.permType);
        };
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.key);
        };
        if !self.range_end.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.range_end);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.permType != Permission_Type::READ {
            os.write_enum(1, self.permType.value())?;
        };
        if !self.key.is_empty() {
            os.write_bytes(2, &self.key)?;
        };
        if !self.range_end.is_empty() {
            os.write_bytes(3, &self.range_end)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Permission {
    fn new() -> Permission {
        Permission::new()
    }

    fn descriptor_static(_: ::std::option::Option<Permission>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Permission_Type>>(
                    "permType",
                    Permission::get_permType_for_reflect,
                    Permission::mut_permType_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    Permission::get_key_for_reflect,
                    Permission::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "range_end",
                    Permission::get_range_end_for_reflect,
                    Permission::mut_range_end_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Permission>(
                    "Permission",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Permission {
    fn clear(&mut self) {
        self.clear_permType();
        self.clear_key();
        self.clear_range_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Permission {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Permission {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Permission_Type {
    READ = 0,
    WRITE = 1,
    READWRITE = 2,
}

impl ::protobuf::ProtobufEnum for Permission_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Permission_Type> {
        match value {
            0 => ::std::option::Option::Some(Permission_Type::READ),
            1 => ::std::option::Option::Some(Permission_Type::WRITE),
            2 => ::std::option::Option::Some(Permission_Type::READWRITE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Permission_Type] = &[
            Permission_Type::READ,
            Permission_Type::WRITE,
            Permission_Type::READWRITE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Permission_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Permission_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Permission_Type {
}

impl ::std::default::Default for Permission_Type {
    fn default() -> Self {
        Permission_Type::READ
    }
}

impl ::protobuf::reflect::ProtobufValue for Permission_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Role {
    // message fields
    pub name: ::std::vec::Vec<u8>,
    keyPermission: ::protobuf::RepeatedField<Permission>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Role {}

impl Role {
    pub fn new() -> Role {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Role {
        static mut instance: ::protobuf::lazy::Lazy<Role> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Role,
        };
        unsafe {
            instance.get(Role::new)
        }
    }

    // bytes name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.name, ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.name
    }

    // repeated .authpb.Permission keyPermission = 2;

    pub fn clear_keyPermission(&mut self) {
        self.keyPermission.clear();
    }

    // Param is passed by value, moved
    pub fn set_keyPermission(&mut self, v: ::protobuf::RepeatedField<Permission>) {
        self.keyPermission = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keyPermission(&mut self) -> &mut ::protobuf::RepeatedField<Permission> {
        &mut self.keyPermission
    }

    // Take field
    pub fn take_keyPermission(&mut self) -> ::protobuf::RepeatedField<Permission> {
        ::std::mem::replace(&mut self.keyPermission, ::protobuf::RepeatedField::new())
    }

    pub fn get_keyPermission(&self) -> &[Permission] {
        &self.keyPermission
    }

    fn get_keyPermission_for_reflect(&self) -> &::protobuf::RepeatedField<Permission> {
        &self.keyPermission
    }

    fn mut_keyPermission_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Permission> {
        &mut self.keyPermission
    }
}

impl ::protobuf::Message for Role {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keyPermission)?;
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
            my_size += ::protobuf::rt::bytes_size(1, &self.name);
        };
        for value in &self.keyPermission {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_bytes(1, &self.name)?;
        };
        for v in &self.keyPermission {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Role {
    fn new() -> Role {
        Role::new()
    }

    fn descriptor_static(_: ::std::option::Option<Role>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    Role::get_name_for_reflect,
                    Role::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Permission>>(
                    "keyPermission",
                    Role::get_keyPermission_for_reflect,
                    Role::mut_keyPermission_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Role>(
                    "Role",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Role {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_keyPermission();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Role {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Role {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1b, 0x65, 0x74, 0x63, 0x64, 0x2f, 0x61, 0x75, 0x74, 0x68, 0x2f, 0x61, 0x75, 0x74, 0x68,
    0x70, 0x62, 0x2f, 0x61, 0x75, 0x74, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x06, 0x61,
    0x75, 0x74, 0x68, 0x70, 0x62, 0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x4c, 0x0a, 0x04, 0x55,
    0x73, 0x65, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x70, 0x61, 0x73, 0x73, 0x77,
    0x6f, 0x72, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x08, 0x70, 0x61, 0x73, 0x73, 0x77,
    0x6f, 0x72, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x72, 0x6f, 0x6c, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03,
    0x28, 0x09, 0x52, 0x05, 0x72, 0x6f, 0x6c, 0x65, 0x73, 0x22, 0x9c, 0x01, 0x0a, 0x0a, 0x50, 0x65,
    0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x33, 0x0a, 0x08, 0x70, 0x65, 0x72, 0x6d,
    0x54, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x17, 0x2e, 0x61, 0x75, 0x74,
    0x68, 0x70, 0x62, 0x2e, 0x50, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x54,
    0x79, 0x70, 0x65, 0x52, 0x08, 0x70, 0x65, 0x72, 0x6d, 0x54, 0x79, 0x70, 0x65, 0x12, 0x10, 0x0a,
    0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12,
    0x1b, 0x0a, 0x09, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x65, 0x6e, 0x64, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x08, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x45, 0x6e, 0x64, 0x22, 0x2a, 0x0a, 0x04,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x08, 0x0a, 0x04, 0x52, 0x45, 0x41, 0x44, 0x10, 0x00, 0x12, 0x09,
    0x0a, 0x05, 0x57, 0x52, 0x49, 0x54, 0x45, 0x10, 0x01, 0x12, 0x0d, 0x0a, 0x09, 0x52, 0x45, 0x41,
    0x44, 0x57, 0x52, 0x49, 0x54, 0x45, 0x10, 0x02, 0x22, 0x54, 0x0a, 0x04, 0x52, 0x6f, 0x6c, 0x65,
    0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04,
    0x6e, 0x61, 0x6d, 0x65, 0x12, 0x38, 0x0a, 0x0d, 0x6b, 0x65, 0x79, 0x50, 0x65, 0x72, 0x6d, 0x69,
    0x73, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x61, 0x75,
    0x74, 0x68, 0x70, 0x62, 0x2e, 0x50, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x52,
    0x0d, 0x6b, 0x65, 0x79, 0x50, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x42, 0x14,
    0xc8, 0xe1, 0x1e, 0x00, 0xd0, 0xe2, 0x1e, 0x01, 0xd0, 0xe1, 0x1e, 0x00, 0xe0, 0xe2, 0x1e, 0x01,
    0xc8, 0xe2, 0x1e, 0x01, 0x4a, 0xe5, 0x0a, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x24, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x01, 0x08, 0x0e, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x07, 0x1d, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x05, 0x00, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x05, 0x00, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x05, 0x07, 0x20, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05,
    0x07, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05,
    0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x05, 0x23, 0x27,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7,
    0x07, 0x01, 0x12, 0x03, 0x06, 0x00, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02,
    0x12, 0x03, 0x06, 0x07, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x06, 0x07, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x06, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03, 0x06,
    0x1f, 0x23, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x2a, 0x0a, 0x0b, 0x0a, 0x04,
    0x08, 0xe7, 0x07, 0x02, 0x12, 0x03, 0x07, 0x00, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x02, 0x02, 0x12, 0x03, 0x07, 0x07, 0x22, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x02, 0x02,
    0x00, 0x12, 0x03, 0x07, 0x07, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x07, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02, 0x03, 0x12,
    0x03, 0x07, 0x25, 0x29, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x2f, 0x0a, 0x0b,
    0x0a, 0x04, 0x08, 0xe7, 0x07, 0x03, 0x12, 0x03, 0x08, 0x00, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x08,
    0xe7, 0x07, 0x03, 0x02, 0x12, 0x03, 0x08, 0x07, 0x26, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x08, 0x07, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x03,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x03,
    0x03, 0x12, 0x03, 0x08, 0x29, 0x2e, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x33,
    0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x04, 0x12, 0x03, 0x09, 0x00, 0x33, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x04, 0x02, 0x12, 0x03, 0x09, 0x07, 0x2a, 0x0a, 0x0d, 0x0a, 0x06, 0x08,
    0xe7, 0x07, 0x04, 0x02, 0x00, 0x12, 0x03, 0x09, 0x07, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7,
    0x07, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7,
    0x07, 0x04, 0x03, 0x12, 0x03, 0x09, 0x2d, 0x32, 0x0a, 0x3c, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x0c, 0x00, 0x10, 0x01, 0x1a, 0x30, 0x20, 0x55, 0x73, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x61,
    0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x20, 0x69, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x61, 0x75, 0x74, 0x68,
    0x55, 0x73, 0x65, 0x72, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0c,
    0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x02, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0d, 0x02, 0x0c, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x0e, 0x02, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x0e, 0x02, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0e,
    0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x13, 0x14, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x0f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x0f, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x0f, 0x1a, 0x1b, 0x0a, 0x2b, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x13, 0x00, 0x1d, 0x01, 0x1a,
    0x1f, 0x20, 0x50, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20,
    0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x13, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x01, 0x04, 0x00, 0x12, 0x04, 0x14, 0x02, 0x18, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x14, 0x07, 0x0b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x15, 0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x15, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x15, 0x0b, 0x0c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x16, 0x04, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x16, 0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x16, 0x0c, 0x0d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x17, 0x04, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x17, 0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x17, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x19,
    0x02, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x19, 0x02, 0x18,
    0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x19, 0x02, 0x06, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x07, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x02, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x1b, 0x02, 0x19, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x1b, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x1b, 0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1b, 0x0e,
    0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x1c, 0x02, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x04, 0x1c, 0x02, 0x1b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1c, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x1c, 0x14, 0x15, 0x0a, 0x3c, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x20,
    0x00, 0x24, 0x01, 0x1a, 0x30, 0x20, 0x52, 0x6f, 0x6c, 0x65, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20,
    0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x20, 0x69, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x62, 0x75, 0x63, 0x6b, 0x65, 0x74, 0x20, 0x61, 0x75, 0x74, 0x68, 0x52,
    0x6f, 0x6c, 0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x20, 0x08,
    0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x21, 0x02, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x21, 0x02, 0x20, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x21, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x21, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x23, 0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x23,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x23, 0x0b, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x23, 0x16, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x23, 0x26, 0x27, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33,
];

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
