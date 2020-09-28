use super::super::value::wasm_valkind_t;
use std::convert::TryInto;
use wasmer::ValType;

/// cbindgen:ignore
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum wasm_valkind_enum {
    WASM_I32 = 0,
    WASM_I64 = 1,
    WASM_F32 = 2,
    WASM_F64 = 3,
    WASM_ANYREF = 128,
    WASM_FUNCREF = 129,
}

impl From<ValType> for wasm_valkind_enum {
    fn from(other: ValType) -> Self {
        match other {
            ValType::I32 => Self::WASM_I32,
            ValType::I64 => Self::WASM_I64,
            ValType::F32 => Self::WASM_F32,
            ValType::F64 => Self::WASM_F64,
            ValType::V128 => todo!("no v128 type in Wasm C API yet!"),
            ValType::ExternRef => Self::WASM_ANYREF,
            ValType::FuncRef => Self::WASM_FUNCREF,
        }
    }
}

impl From<wasm_valkind_enum> for ValType {
    fn from(other: wasm_valkind_enum) -> Self {
        use wasm_valkind_enum::*;
        match other {
            WASM_I32 => ValType::I32,
            WASM_I64 => ValType::I64,
            WASM_F32 => ValType::F32,
            WASM_F64 => ValType::F64,
            WASM_ANYREF => ValType::ExternRef,
            WASM_FUNCREF => ValType::FuncRef,
        }
    }
}

/// cbindgen:ignore
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct wasm_valtype_t {
    valkind: wasm_valkind_enum,
}

impl Default for wasm_valtype_t {
    fn default() -> Self {
        Self {
            valkind: wasm_valkind_enum::WASM_I32,
        }
    }
}

wasm_declare_boxed_vec!(valtype);

impl From<wasm_valtype_t> for ValType {
    fn from(other: wasm_valtype_t) -> Self {
        other.valkind.into()
    }
}

impl From<ValType> for wasm_valtype_t {
    fn from(other: ValType) -> Self {
        Self {
            valkind: other.into(),
        }
    }
}

/// cbindgen:ignore
#[no_mangle]
pub extern "C" fn wasm_valtype_new(kind: wasm_valkind_t) -> Option<Box<wasm_valtype_t>> {
    let kind_enum = kind.try_into().ok()?;
    let valtype = wasm_valtype_t { valkind: kind_enum };
    Some(Box::new(valtype))
}

/// cbindgen:ignore
#[no_mangle]
pub unsafe extern "C" fn wasm_valtype_delete(_valtype: Option<Box<wasm_valtype_t>>) {}

/// cbindgen:ignore
#[no_mangle]
pub unsafe extern "C" fn wasm_valtype_kind(valtype: *const wasm_valtype_t) -> wasm_valkind_t {
    if valtype.is_null() {
        // TODO: handle error
        panic!("wasm_valtype_kind: argument is null pointer");
    }
    return (*valtype).valkind as wasm_valkind_t;
}
