pub mod fvad {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!("../output", "/bindings.rs"));

    pub struct Vad {
        fvad: *mut Fvad
    }

    impl Vad {
        pub unsafe fn new(sample_rate: i32) -> Vad {
            let fvad: *mut Fvad = fvad_new();
            fvad_set_sample_rate(fvad, sample_rate);

            Vad {
                fvad
            }
        }

        pub unsafe fn is_silent(self, buffers: &[i16]) -> ::std::os::raw::c_int {
            let buffer = &buffers[0] as *const i16;

            fvad_process(self.fvad, buffer, buffers.len())
        }
    }
}