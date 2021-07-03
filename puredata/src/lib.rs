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
    static ref BANK_A: [Wavetable<'static>; 4] = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        [
            Wavetable::new(&waveform::perfect::PERFECT_0_FACTORS, sample_rate),
            Wavetable::new(&waveform::perfect::PERFECT_1_FACTORS, sample_rate),
            Wavetable::new(&waveform::perfect::PERFECT_2_FACTORS, sample_rate),
            Wavetable::new(&waveform::perfect::PERFECT_3_FACTORS, sample_rate),
        ]
    };
    static ref BANK_B: [Wavetable<'static>; 6] = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        [
            Wavetable::new(&waveform::akwf::FM_0_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::FM_1_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::FM_2_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::FM_3_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::FM_4_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::FM_5_FACTORS, sample_rate),
        ]
    };
    static ref BANK_C: [Wavetable<'static>; 2] = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        [
            Wavetable::new(&waveform::akwf::EGUITAR_0_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::EGUITAR_1_FACTORS, sample_rate),
        ]
    };
    static ref BANK_D: [Wavetable<'static>; 5] = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        [
            Wavetable::new(&waveform::akwf::DISTORTED_0_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::DISTORTED_1_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::DISTORTED_2_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::DISTORTED_3_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::DISTORTED_4_FACTORS, sample_rate),
        ]
    };
    static ref BANK_E: [Wavetable<'static>; 6] = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        [
            Wavetable::new(&waveform::akwf::GRANULAR_0_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::GRANULAR_1_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::GRANULAR_2_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::GRANULAR_3_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::GRANULAR_4_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::GRANULAR_5_FACTORS, sample_rate),
        ]
    };
    static ref BANK_F: [Wavetable<'static>; 4] = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        [
            Wavetable::new(&waveform::akwf::OSCCHIP_0_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::OSCCHIP_1_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::OSCCHIP_2_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::OSCCHIP_3_FACTORS, sample_rate),
        ]
    };
    static ref BANK_G: [Wavetable<'static>; 4] = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        [
            Wavetable::new(&waveform::akwf::STRINGBOX_0_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::STRINGBOX_1_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::STRINGBOX_2_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::STRINGBOX_3_FACTORS, sample_rate),
        ]
    };
    static ref BANK_H: [Wavetable<'static>; 6] = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        [
            Wavetable::new(&waveform::akwf::FLUTE_0_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::FLUTE_1_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::FLUTE_2_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::FLUTE_3_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::FLUTE_4_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::FLUTE_5_FACTORS, sample_rate),
        ]
    };
    static ref BANK_I: [Wavetable<'static>; 5] = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        [
            Wavetable::new(&waveform::akwf::OBOE_0_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::OBOE_1_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::OBOE_2_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::OBOE_3_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::OBOE_4_FACTORS, sample_rate),
        ]
    };
    static ref BANK_J: [Wavetable<'static>; 9] = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        [
            Wavetable::new(&waveform::akwf::VIOLIN_0_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::VIOLIN_1_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::VIOLIN_2_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::VIOLIN_3_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::VIOLIN_4_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::VIOLIN_5_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::VIOLIN_6_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::VIOLIN_7_FACTORS, sample_rate),
            Wavetable::new(&waveform::akwf::VIOLIN_8_FACTORS, sample_rate),
        ]
    };
    static ref BANK_K: [Wavetable<'static>; 9] = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        [
            Wavetable::new(&waveform::harsh::HARSH_0_FACTORS, sample_rate),
            Wavetable::new(&waveform::harsh::HARSH_1_FACTORS, sample_rate),
            Wavetable::new(&waveform::harsh::HARSH_2_FACTORS, sample_rate),
            Wavetable::new(&waveform::harsh::HARSH_3_FACTORS, sample_rate),
            Wavetable::new(&waveform::harsh::HARSH_4_FACTORS, sample_rate),
            Wavetable::new(&waveform::harsh::HARSH_5_FACTORS, sample_rate),
            Wavetable::new(&waveform::harsh::HARSH_6_FACTORS, sample_rate),
            Wavetable::new(&waveform::harsh::HARSH_7_FACTORS, sample_rate),
            Wavetable::new(&waveform::harsh::HARSH_8_FACTORS, sample_rate),
        ]
    };
    static ref BANK_L: [Wavetable<'static>; 5] = {
        let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
        [
            Wavetable::new(&waveform::soft::SOFT_0_FACTORS, sample_rate),
            Wavetable::new(&waveform::soft::SOFT_1_FACTORS, sample_rate),
            Wavetable::new(&waveform::soft::SOFT_2_FACTORS, sample_rate),
            Wavetable::new(&waveform::soft::SOFT_3_FACTORS, sample_rate),
            Wavetable::new(&waveform::soft::SOFT_4_FACTORS, sample_rate),
        ]
    };
    static ref WAVETABLE_BANKS: [&'static [Wavetable<'static>]; 12] = [
        &BANK_A[..],
        &BANK_B[..],
        &BANK_C[..],
        &BANK_D[..],
        &BANK_E[..],
        &BANK_F[..],
        &BANK_G[..],
        &BANK_H[..],
        &BANK_I[..],
        &BANK_J[..],
        &BANK_K[..],
        &BANK_L[..],
    ];
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
    register_float_method(class, "wavetable_bank", set_wavetable_bank);
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
    let instrument = Instrument::new(&WAVETABLE_BANKS[..], sample_rate);

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

unsafe extern "C" fn set_wavetable_bank(class: *mut Class, value: pd_sys::t_float) {
    (*class)
        .instrument
        .set_wavetable_bank(value.clamp(0.0, 1.0));
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

    let mut buffer_root = [0.0; BUFFER_LEN];
    let mut buffer_chord = [0.0; BUFFER_LEN];

    for chunk_index in 0..outlets[0].len() / BUFFER_LEN {
        class
            .instrument
            .populate(&mut buffer_root[..], &mut buffer_chord[..]);

        let start = chunk_index * BUFFER_LEN;
        for i in 0..BUFFER_LEN {
            outlets[1][start + i] = buffer_root[i];
            outlets[2][start + i] = buffer_chord[i];
            outlets[0][start + i] = (outlets[1][start + i] + outlets[2][start + i]) / 2.0;
        }
    }
}
