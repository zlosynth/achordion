#![deny(clippy::transmute_ptr_to_ptr)]
#![allow(clippy::missing_safety_doc)]

#[macro_use]
extern crate field_offset;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod wrapper;

mod cstr;
mod log;

use std::os::raw::{c_int, c_void};

use achordion_lib::instrument::Instrument;
use achordion_lib::waveform;
use achordion_lib::wavetable::Wavetable;

static mut CLASS: Option<*mut pd_sys::_class> = None;

lazy_static! {
    static ref SINE: Wavetable<'static> = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        Wavetable::new(&waveform::sine::SINE_FACTORS, sample_rate)
    };
    static ref TRIANGLE: Wavetable<'static> = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        Wavetable::new(&waveform::triangle::TRIANGLE_FACTORS, sample_rate)
    };
    static ref SQUARE: Wavetable<'static> = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        Wavetable::new(&waveform::square::SQUARE_FACTORS, sample_rate)
    };
    static ref SAW: Wavetable<'static> = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        Wavetable::new(&waveform::saw::SAW_FACTORS, sample_rate)
    };
    static ref WAVETABLES: [&'static Wavetable<'static>; 4] = [&SINE, &TRIANGLE, &SQUARE, &SAW];
}

#[repr(C)]
struct Class<'a> {
    pd_obj: pd_sys::t_object,
    instrument: Instrument<'a>,
    signal_dummy: f32,
}

#[no_mangle]
pub unsafe extern "C" fn achordion_tilde_setup() {
    let class = create_class();

    CLASS = Some(class);

    register_dsp_method!(
        class,
        receiver = Class,
        dummy_offset = offset_of!(Class => signal_dummy),
        number_of_inlets = 1,
        number_of_outlets = 1,
        callback = perform
    );

    register_set_frequency_method(class);
}

unsafe fn create_class() -> *mut pd_sys::_class {
    log::info("[achordion~] initializing");

    pd_sys::class_new(
        pd_sys::gensym(cstr::cstr("achordion~").as_ptr()),
        Some(std::mem::transmute::<
            unsafe extern "C" fn() -> *mut c_void,
            _,
        >(new)),
        None,
        std::mem::size_of::<Class>(),
        pd_sys::CLASS_DEFAULT as i32,
        0,
    )
}

unsafe extern "C" fn new() -> *mut c_void {
    let class = pd_sys::pd_new(CLASS.unwrap()) as *mut Class;

    let sample_rate = pd_sys::sys_getsr() as u32;
    let instrument = Instrument::new(&WAVETABLES[..], sample_rate);

    (*class).instrument = instrument;

    pd_sys::outlet_new(&mut (*class).pd_obj, &mut pd_sys::s_signal);

    class as *mut c_void
}

unsafe fn register_set_frequency_method(class: *mut pd_sys::_class) {
    pd_sys::class_addmethod(
        class,
        Some(std::mem::transmute::<
            unsafe extern "C" fn(*mut Class, pd_sys::t_float),
            _,
        >(set_frequency)),
        &mut pd_sys::s_float,
        pd_sys::t_atomtype::A_FLOAT,
        0,
    );
}

unsafe extern "C" fn set_frequency(class: *mut Class, value: pd_sys::t_float) {
    (*class)
        .instrument
        .set_frequency(value.clamp(0.0, 20_000.0));
}

fn perform(
    class: &mut Class,
    _number_of_frames: usize,
    _inlets: &[&mut [pd_sys::t_float]],
    outlets: &mut [&mut [pd_sys::t_float]],
) {
    let mut buffer = [0];
    for x in outlets[0].iter_mut() {
        class.instrument.populate(&mut buffer[..]);
        *x = buffer[0] as f32 / f32::powi(2.0, 16);
    }
}
