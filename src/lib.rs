pub mod fvad {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!("../output", "/bindings.rs"));

    pub unsafe fn is_silent(fvad: *mut Fvad, buffers: &[i16]) -> ::std::os::raw::c_int {
        let buffer = &buffers[0] as *const i16;

        fvad_process(fvad, buffer, buffers.len())
    }

    pub unsafe fn new() -> *mut Fvad {
        fvad_new()
    }

    pub unsafe fn set_sample_rate(fvad: *mut Fvad, sample_rate: i32) {
        fvad_set_sample_rate(fvad, sample_rate);
    }
}