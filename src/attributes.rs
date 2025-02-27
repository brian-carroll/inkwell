//! `Attribute`s are optional modifiers to functions, function parameters, and return types.

use llvm_sys::core::{
    LLVMGetEnumAttributeKind, LLVMGetEnumAttributeKindForName, LLVMGetEnumAttributeValue, LLVMGetLastEnumAttributeKind,
    LLVMGetStringAttributeKind, LLVMGetStringAttributeValue, LLVMIsEnumAttribute, LLVMIsStringAttribute,
};
#[llvm_versions(12.0..=latest)]
use llvm_sys::core::{LLVMGetTypeAttributeValue, LLVMIsTypeAttribute};
use llvm_sys::prelude::LLVMAttributeRef;

use std::ffi::CStr;

#[llvm_versions(12.0..=latest)]
use crate::types::AnyTypeEnum;
#[cfg(feature = "internal-getters")]
use crate::LLVMReference;

// SubTypes: Attribute<Enum>, Attribute<String>
/// Functions, function parameters, and return types can have `Attribute`s to indicate
/// how they should be treated by optimizations and code generation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Attribute {
    pub(crate) attribute: LLVMAttributeRef,
}

impl Attribute {
    pub(crate) unsafe fn new(attribute: LLVMAttributeRef) -> Self {
        debug_assert!(!attribute.is_null());

        Attribute { attribute }
    }

    /// Determines whether or not an `Attribute` is an enum. This method will
    /// likely be removed in the future in favor of `Attribute`s being generically
    /// defined.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use inkwell::context::Context;
    ///
    /// let context = Context::create();
    /// let enum_attribute = context.create_enum_attribute(0, 10);
    ///
    /// assert!(enum_attribute.is_enum());
    /// ```
    pub fn is_enum(self) -> bool {
        unsafe { LLVMIsEnumAttribute(self.attribute) == 1 }
    }

    /// Determines whether or not an `Attribute` is a string. This method will
    /// likely be removed in the future in favor of `Attribute`s being generically
    /// defined.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use inkwell::context::Context;
    ///
    /// let context = Context::create();
    /// let string_attribute = context.create_string_attribute("my_key_123", "my_val");
    ///
    /// assert!(string_attribute.is_string());
    /// ```
    pub fn is_string(self) -> bool {
        unsafe { LLVMIsStringAttribute(self.attribute) == 1 }
    }

    /// Determines whether or not an `Attribute` is a type attribute. This method will
    /// likely be removed in the future in favor of `Attribute`s being generically
    /// defined.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use inkwell::context::Context;
    /// use inkwell::attributes::Attribute;
    ///
    /// let context = Context::create();
    /// let kind_id = Attribute::get_named_enum_kind_id("sret");
    /// let type_attribute = context.create_type_attribute(
    ///     kind_id,
    ///     context.i32_type().into(),
    /// );
    ///
    /// assert!(type_attribute.is_type());
    /// ```
    #[llvm_versions(12.0..=latest)]
    pub fn is_type(self) -> bool {
        unsafe { LLVMIsTypeAttribute(self.attribute) == 1 }
    }

    /// Gets the enum kind id associated with a builtin name.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use inkwell::attributes::Attribute;
    ///
    /// // This kind id doesn't exist:
    /// assert_eq!(Attribute::get_named_enum_kind_id("foobar"), 0);
    ///
    /// // These are real kind ids:
    /// assert_eq!(Attribute::get_named_enum_kind_id("align"), 1);
    /// assert_eq!(Attribute::get_named_enum_kind_id("builtin"), 5);
    /// ```
    pub fn get_named_enum_kind_id(name: &str) -> u32 {
        unsafe {
            LLVMGetEnumAttributeKindForName(name.as_ptr() as *const ::libc::c_char, name.len())
        }
    }

    /// Gets the kind id associated with an enum `Attribute`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use inkwell::context::Context;
    ///
    /// let context = Context::create();
    /// let enum_attribute = context.create_enum_attribute(0, 10);
    ///
    /// assert_eq!(enum_attribute.get_enum_kind_id(), 0);
    /// ```
    ///
    /// This function also works for type `Attribute`s.
    ///
    /// ```no_run
    /// use inkwell::context::Context;
    /// use inkwell::attributes::Attribute;
    /// use inkwell::types::AnyType;
    ///
    /// let context = Context::create();
    /// let kind_id = Attribute::get_named_enum_kind_id("sret");
    /// let any_type = context.i32_type().as_any_type_enum();
    /// let type_attribute = context.create_type_attribute(
    ///     kind_id,
    ///     any_type,
    /// );
    ///
    /// assert_eq!(type_attribute.get_enum_kind_id(), kind_id);
    /// ```
    #[llvm_versions(12.0..=latest)]
    pub fn get_enum_kind_id(self) -> u32 {
        assert!(self.get_enum_kind_id_is_valid()); // FIXME: SubTypes

        unsafe {
            LLVMGetEnumAttributeKind(self.attribute)
        }
    }

    #[llvm_versions(4.0..12.0)]
    fn get_enum_kind_id_is_valid(self) -> bool {
        self.is_enum()
    }

    #[llvm_versions(12.0..=latest)]
    fn get_enum_kind_id_is_valid(self) -> bool {
        self.is_enum() || self.is_type()
    }

    /// Gets the last enum kind id associated with builtin names.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use inkwell::attributes::Attribute;
    ///
    /// assert_eq!(Attribute::get_last_enum_kind_id(), 56);
    /// ```
    pub fn get_last_enum_kind_id() -> u32 {
        unsafe { LLVMGetLastEnumAttributeKind() }
    }

    /// Gets the value associated with an enum `Attribute`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use inkwell::context::Context;
    ///
    /// let context = Context::create();
    /// let enum_attribute = context.create_enum_attribute(0, 10);
    ///
    /// assert_eq!(enum_attribute.get_enum_value(), 10);
    /// ```
    pub fn get_enum_value(self) -> u64 {
        assert!(self.is_enum()); // FIXME: SubTypes

        unsafe { LLVMGetEnumAttributeValue(self.attribute) }
    }

    /// Gets the string kind id associated with a string attribute.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use inkwell::context::Context;
    ///
    /// let context = Context::create();
    /// let string_attribute = context.create_string_attribute("my_key", "my_val");
    ///
    /// assert_eq!(string_attribute.get_string_kind_id().to_str(), Ok("my_key"));
    /// ```
    // TODO: Check if null, return option
    pub fn get_string_kind_id(&self) -> &CStr {
        assert!(self.is_string()); // FIXME: SubTypes

        let mut length = 0;
        let cstr_ptr = unsafe { LLVMGetStringAttributeKind(self.attribute, &mut length) };

        unsafe { CStr::from_ptr(cstr_ptr) }
    }

    /// Gets the string value associated with a string attribute.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use inkwell::context::Context;
    ///
    /// let context = Context::create();
    /// let string_attribute = context.create_string_attribute("my_key", "my_val");
    ///
    /// assert_eq!(string_attribute.get_string_value().to_str(), Ok("my_val"));
    /// ```
    pub fn get_string_value(&self) -> &CStr {
        assert!(self.is_string()); // FIXME: SubTypes

        let mut length = 0;
        let cstr_ptr = unsafe { LLVMGetStringAttributeValue(self.attribute, &mut length) };

        unsafe { CStr::from_ptr(cstr_ptr) }
    }

    /// Gets the type associated with a type attribute.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use inkwell::context::Context;
    /// use inkwell::attributes::Attribute;
    /// use inkwell::types::AnyType;
    ///
    /// let context = Context::create();
    /// let kind_id = Attribute::get_named_enum_kind_id("sret");
    /// let any_type = context.i32_type().as_any_type_enum();
    /// let type_attribute = context.create_type_attribute(
    ///     kind_id,
    ///     any_type,
    /// );
    ///
    /// assert!(type_attribute.is_type());
    /// assert_eq!(type_attribute.get_type_value(), any_type);
    /// assert_ne!(type_attribute.get_type_value(), context.i64_type().as_any_type_enum());
    /// ```
    #[llvm_versions(12.0..=latest)]
    pub fn get_type_value(&self) -> AnyTypeEnum {
        assert!(self.is_type()); // FIXME: SubTypes

        unsafe { AnyTypeEnum::new(LLVMGetTypeAttributeValue(self.attribute)) }
    }
}

/// An `AttributeLoc` determines where on a function an attribute is assigned to.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AttributeLoc {
    /// Assign to the `FunctionValue`'s return type.
    Return,
    /// Assign to one of the `FunctionValue`'s params (0-indexed).
    Param(u32),
    /// Assign to the `FunctionValue` itself.
    Function,
}

impl AttributeLoc {
    pub(crate) fn get_index(self) -> u32 {
        match self {
            AttributeLoc::Return => 0,
            AttributeLoc::Param(index) => {
                assert!(
                    index <= u32::max_value() - 2,
                    "Param index must be <= u32::max_value() - 2"
                );

                index + 1
            },
            AttributeLoc::Function => u32::max_value(),
        }
    }
}

#[cfg(feature = "internal-getters")]
impl LLVMReference<LLVMAttributeRef> for Attribute {
    unsafe fn get_ref(&self) -> LLVMAttributeRef {
        self.attribute
    }
}
