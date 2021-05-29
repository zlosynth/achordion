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
    static ref PULSE: Wavetable<'static> = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        Wavetable::new(&waveform::pulse::PULSE_FACTORS, sample_rate)
    };
    static ref SAW: Wavetable<'static> = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        Wavetable::new(&waveform::saw::SAW_FACTORS, sample_rate)
    };
    static ref WAVETABLES: [&'static Wavetable<'static>; 4] = [&SINE, &TRIANGLE, &PULSE, &SAW];
}

#[repr(C)]
struct Class<'a> {
    pd_obj: pd_sys::t_object,
    root_outlet: *mut pd_sys::_outlet,
    chord_outlet: *mut pd_sys::_outlet,
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
        number_of_outlets = 3,
        callback = perform
    );

    register_float_method(class, "float", set_chord_root);
    register_float_method(class, "chord_degrees", set_chord_degrees);
    register_float_method(class, "scale_mode", set_scale_mode);
    register_float_method(class, "scale_root", set_scale_root);
    register_float_method(class, "wavetable", set_wavetable);
    register_float_method(class, "detune", set_detune);
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
    (*class).root_outlet = pd_sys::outlet_new(&mut (*class).pd_obj, &mut pd_sys::s_signal);
    (*class).chord_outlet = pd_sys::outlet_new(&mut (*class).pd_obj, &mut pd_sys::s_signal);

    class as *mut c_void
}

unsafe fn register_float_method(
    class: *mut pd_sys::_class,
    symbol: &str,
    method: unsafe extern "C" fn(*mut Class, pd_sys::t_float),
) {
    pd_sys::class_addmethod(
        class,
        Some(std::mem::transmute::<
            unsafe extern "C" fn(*mut Class, pd_sys::t_float),
            _,
        >(method)),
        pd_sys::gensym(cstr::cstr(symbol).as_ptr()),
        pd_sys::t_atomtype::A_FLOAT,
        0,
    );
}

unsafe extern "C" fn set_chord_root(class: *mut Class, value: pd_sys::t_float) {
    (*class).instrument.set_chord_root(value.clamp(0.0, 20.0));
}

unsafe extern "C" fn set_chord_degrees(class: *mut Class, value: pd_sys::t_float) {
    (*class).instrument.set_chord_degrees(value.clamp(0.0, 1.0));
}

unsafe extern "C" fn set_scale_mode(class: *mut Class, value: pd_sys::t_float) {
    (*class).instrument.set_scale_mode(value.clamp(0.0, 1.0));
}

unsafe extern "C" fn set_scale_root(class: *mut Class, value: pd_sys::t_float) {
    (*class).instrument.set_scale_root(value.clamp(0.0, 20.0));
}

unsafe extern "C" fn set_wavetable(class: *mut Class, value: pd_sys::t_float) {
    (*class).instrument.set_wavetable(value.clamp(0.0, 1.0));
}

unsafe extern "C" fn set_detune(class: *mut Class, value: pd_sys::t_float) {
    (*class).instrument.set_detune(value.clamp(0.0, 1.0));
}

fn perform(
    class: &mut Class,
    _number_of_frames: usize,
    _inlets: &[&mut [pd_sys::t_float]],
    outlets: &mut [&mut [pd_sys::t_float]],
) {
    const BUFFER_LEN: usize = 32;
    assert!(outlets[0].len() % BUFFER_LEN == 0);

    let mut buffer_root = [0; BUFFER_LEN];
    let mut buffer_chord = [0; BUFFER_LEN];

    for chunk_index in 0..outlets[0].len() / BUFFER_LEN {
        class
            .instrument
            .populate(&mut buffer_root[..], &mut buffer_chord[..]);

        let start = chunk_index * BUFFER_LEN;
        for i in 0..BUFFER_LEN {
            outlets[1][start + i] = buffer_root[i] as f32 / f32::powi(2.0, 15) - 1.0;
            outlets[2][start + i] = buffer_chord[i] as f32 / f32::powi(2.0, 15) - 1.0;
            outlets[0][start + i] = (outlets[1][start + i] + outlets[2][start + i]) / 2.0;
        }
    }
}
