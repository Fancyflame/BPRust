use std::{cell::UnsafeCell, ffi::c_char};

pub mod custom_thunk;

static CPP_FUNCTION_TABLE: InitCell = InitCell(UnsafeCell::new(None));

struct InitCell(UnsafeCell<Option<CppFunctionTable>>);
unsafe impl Send for InitCell {}
unsafe impl Sync for InitCell {}

#[repr(C)]
pub struct CppFunctionTable {
    pub handle_custom_thunk: extern "C" fn(
        handler: &mut custom_thunk::Handler,
        user_data: *mut (),
        resolve_param: extern "C" fn(user_data: *mut (), handler: &mut custom_thunk::Handler),
        call_function: extern "C" fn(user_data: *mut (), u_object: *mut ()),
    ),
    pub process_event: extern "C" fn(u_object: *mut (), fn_name: &c_char, params: *mut ()),
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
unsafe extern "C" fn BPRustSys_init(table: CppFunctionTable) {
    unsafe {
        match &mut *CPP_FUNCTION_TABLE.0.get() {
            Some(_) => {
                panic!(
                    "BPRust ERROR: BPRust is already initialized, don't call `BPRustSys_init` twice"
                )
            }
            place @ None => *place = Some(table),
        }
    }
}

pub fn cpp_get() -> &'static CppFunctionTable {
    unsafe {
        let table = &*CPP_FUNCTION_TABLE.0.get();
        match table {
            Some(t) => t,
            None => {
                panic!("BPRust ERROR: should NOT use any functions before calling `BPRustSys_init`")
            }
        }
    }
}
