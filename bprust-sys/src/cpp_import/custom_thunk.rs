#[repr(C)]
pub struct Handler {
    context: *mut (),
    fframe: *mut (),
    z_param_result: *mut (),
}
