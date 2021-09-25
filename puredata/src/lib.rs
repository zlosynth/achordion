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

use achordion_lib::bank;
use achordion_lib::instrument::Instrument;
use achordion_lib::waveform;
use achordion_lib::wavetable::Wavetable;

static mut CLASS: Option<*mut pd_sys::_class> = None;

macro_rules! bank {
    ( $module:ident, $bank:ident, $len:expr, $( $waveform:ident, $factors:ident, $ref:ident ),* ) => {
        lazy_static! {
            $(
            static ref $factors: bank::factor::Factors =
                bank::factor::Factors::from_raw(&waveform::$module::$waveform);
            static ref $ref: [&'static [u16]; 11] = {
                [
                    &$factors.factor1,
                    &$factors.factor2,
                    &$factors.factor4,
                    &$factors.factor8,
                    &$factors.factor16,
                    &$factors.factor32,
                    &$factors.factor64,
                    &$factors.factor128,
                    &$factors.factor256,
                    &$factors.factor512,
                    &$factors.factor1024,
                ]
            };
            )*
            static ref $bank: [Wavetable<'static>; $len] = {
                let sample_rate = unsafe { pd_sys::sys_getsr() as u32 };
                [
                    $(
                    Wavetable::new(&(*$ref), sample_rate),
                    )*
                ]
            };
        }
    };
}

bank!(
    perfect,
    BANK_PERFECT,
    4,
    PERFECT_0,
    PERFECT_0_FACTORS,
    PERFECT_0_FACTORS_REF,
    PERFECT_1,
    PERFECT_1_FACTORS,
    PERFECT_1_FACTORS_REF,
    PERFECT_2,
    PERFECT_2_FACTORS,
    PERFECT_2_FACTORS_REF,
    PERFECT_3,
    PERFECT_3_FACTORS,
    PERFECT_3_FACTORS_REF
);

bank!(
    harsh,
    BANK_HARSH,
    6,
    HARSH_0,
    HARSH_0_FACTORS,
    HARSH_0_FACTORS_REF,
    HARSH_1,
    HARSH_1_FACTORS,
    HARSH_1_FACTORS_REF,
    HARSH_2,
    HARSH_2_FACTORS,
    HARSH_2_FACTORS_REF,
    HARSH_3,
    HARSH_3_FACTORS,
    HARSH_3_FACTORS_REF,
    HARSH_4,
    HARSH_4_FACTORS,
    HARSH_4_FACTORS_REF,
    HARSH_5,
    HARSH_5_FACTORS,
    HARSH_5_FACTORS_REF
);

bank!(
    soft,
    BANK_SOFT,
    6,
    SOFT_0,
    SOFT_0_FACTORS,
    SOFT_0_FACTORS_REF,
    SOFT_1,
    SOFT_1_FACTORS,
    SOFT_1_FACTORS_REF,
    SOFT_2,
    SOFT_2_FACTORS,
    SOFT_2_FACTORS_REF,
    SOFT_3,
    SOFT_3_FACTORS,
    SOFT_3_FACTORS_REF,
    SOFT_4,
    SOFT_4_FACTORS,
    SOFT_4_FACTORS_REF,
    SOFT_5,
    SOFT_5_FACTORS,
    SOFT_5_FACTORS_REF
);

bank!(
    vocal,
    BANK_VOCAL,
    5,
    VOCAL_0,
    VOCAL_0_FACTORS,
    VOCAL_0_FACTORS_REF,
    VOCAL_1,
    VOCAL_1_FACTORS,
    VOCAL_1_FACTORS_REF,
    VOCAL_2,
    VOCAL_2_FACTORS,
    VOCAL_2_FACTORS_REF,
    VOCAL_3,
    VOCAL_3_FACTORS,
    VOCAL_3_FACTORS_REF,
    VOCAL_4,
    VOCAL_4_FACTORS,
    VOCAL_4_FACTORS_REF
);

lazy_static! {
    static ref WAVETABLE_BANKS: [&'static [Wavetable<'static>]; 4] = [
        &BANK_PERFECT[..],
        &BANK_HARSH[..],
        &BANK_SOFT[..],
        &BANK_VOCAL[..]
    ];
}

#[repr(C)]
struct Class<'a> {
    pd_obj: pd_sys::t_object,
    solo_outlet: *mut pd_sys::_outlet,
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

    register_float_method(class, "solo", set_solo);
    register_float_method(class, "float", set_chord_root);
    register_float_method(class, "chord_degrees", set_chord_degrees);
    register_float_method(class, "scale_mode", set_scale_mode);
    register_float_method(class, "scale_root", set_scale_root);
    register_float_method(class, "wavetable_bank", set_wavetable_bank);
    register_float_method(class, "wavetable", set_wavetable);
    register_float_method(class, "detune", set_detune);
    register_float_method(class, "style", set_style);
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
    (*class).solo_outlet = pd_sys::outlet_new(&mut (*class).pd_obj, &mut pd_sys::s_signal);
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

unsafe extern "C" fn set_solo(class: *mut Class, value: pd_sys::t_float) {
    if value < 0.1 {
        (*class).instrument.set_solo_voct(None);
    } else {
        (*class)
            .instrument
            .set_solo_voct(Some(value.clamp(0.0, 10.0)));
    }
}

unsafe extern "C" fn set_chord_root(class: *mut Class, value: pd_sys::t_float) {
    (*class)
        .instrument
        .set_chord_root_linear(value.clamp(0.0, 10.0));
}

unsafe extern "C" fn set_chord_degrees(class: *mut Class, value: pd_sys::t_float) {
    (*class).instrument.set_chord_degrees(value.clamp(0.0, 1.0));
}

unsafe extern "C" fn set_scale_mode(class: *mut Class, value: pd_sys::t_float) {
    (*class).instrument.set_scale_mode(value.clamp(0.0, 1.0));
}

unsafe extern "C" fn set_scale_root(class: *mut Class, value: pd_sys::t_float) {
    (*class)
        .instrument
        .set_scale_root_voct(value.clamp(0.0, 20.0));
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

unsafe extern "C" fn set_style(class: *mut Class, value: pd_sys::t_float) {
    (*class).instrument.set_style(value.clamp(0.0, 1.0));
}

fn perform(
    class: &mut Class,
    _number_of_frames: usize,
    _inlets: &[&mut [pd_sys::t_float]],
    outlets: &mut [&mut [pd_sys::t_float]],
) {
    const BUFFER_LEN: usize = 32;
    assert!(outlets[0].len() % BUFFER_LEN == 0);

    let mut buffer_solo = [0.0; BUFFER_LEN];
    let mut buffer_chord = [0.0; BUFFER_LEN];

    for chunk_index in 0..outlets[0].len() / BUFFER_LEN {
        class
            .instrument
            .populate(&mut buffer_solo[..], &mut buffer_chord[..]);

        let start = chunk_index * BUFFER_LEN;
        for i in 0..BUFFER_LEN {
            outlets[1][start + i] = buffer_solo[i];
            outlets[2][start + i] = buffer_chord[i];
            outlets[0][start + i] = (outlets[1][start + i] + outlets[2][start + i]) / 2.0;
        }
    }
}
