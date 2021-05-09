#[macro_export]
macro_rules! register_dsp_method {
    ( $class:ident, receiver = $receiver:ty, dummy_offset = $offset:expr, number_of_inlets = $inlets:expr, number_of_outlets = $outlets:expr, callback = $perform:expr ) => {
        register_dsp_method($class);

        unsafe fn register_dsp_method(class: *mut pd_sys::_class) {
            assert!($inlets >= 1, "number of inlets must be set to >= 1, pure data always register one inlet, even when it's not used");

            pd_sys::class_addmethod(
                class,
                Some(std::mem::transmute::<
                    unsafe extern "C" fn(*mut $receiver, *mut *mut pd_sys::t_signal),
                    _,
                >(__dsp_method)),
                pd_sys::gensym(cstr::cstr("dsp").as_ptr()),
                pd_sys::t_atomtype::A_CANT,
                0,
            );

            pd_sys::class_domainsignalin(class, $offset.get_byte_offset() as c_int);
        }

        unsafe extern "C" fn __dsp_method(
            receiver: *mut $receiver,
            signal: *mut *mut pd_sys::t_signal,
        ) {
            let iolets = $inlets + $outlets;

            let vector_length = {
                let receiver = 1;
                let number_of_frames = 1;
                receiver + number_of_frames + iolets
            };

            let signal = std::slice::from_raw_parts(signal, iolets);

            let number_of_frames = (*signal[0]).s_n as usize;

            let vector_size = vector_length * std::mem::size_of::<*mut pd_sys::t_int>();
            let vector_pointer = pd_sys::getbytes(vector_size);
            assert!(
                !vector_pointer.is_null(),
                "null pointer from pd_sys::getbytes",
            );

            let vector = vector_pointer as *mut *mut pd_sys::t_int;
            let vector: &mut [*mut pd_sys::t_int] =
                std::slice::from_raw_parts_mut(vector, vector_length);

            vector[1] = std::mem::transmute(number_of_frames);
            for i in 0..iolets {
                vector[2 + i] = (*signal[i]).s_vec as *mut pd_sys::t_int;
            }

            vector[0] = receiver as *mut pd_sys::t_int;

            pd_sys::dsp_addv(
                Some(__perform),
                vector_length as c_int,
                vector_pointer as *mut pd_sys::t_int,
            );

            pd_sys::freebytes(vector_pointer, vector_size);
        }

        unsafe extern "C" fn __perform(buffer_pointer: *mut pd_sys::t_int) -> *mut pd_sys::t_int {
            let buffer_length = {
                let reserved = 1;
                let receiver = 1;
                let number_of_frames = 1;
                let inlets = $inlets.min(1);
                let outlets = $outlets;
                reserved + receiver + number_of_frames + inlets + outlets
            };

            let arguments = std::slice::from_raw_parts(buffer_pointer, buffer_length);

            let receiver = arguments[1] as *mut $receiver;

            let number_of_frames = arguments[2] as usize;

            let mut inlets: [&mut [pd_sys::t_float]; $inlets] = Default::default();
            #[allow(clippy::reversed_empty_ranges)]
            for i in 0..$inlets {
                inlets[i] = crate::wrapper::read_signal(arguments[3 + i], number_of_frames);
            }

            let mut outlets: [&mut [pd_sys::t_float]; $outlets] = Default::default();
            #[allow(clippy::reversed_empty_ranges)]
            for i in 0..$outlets {
                outlets[i] =
                    crate::wrapper::read_signal(arguments[3 + $inlets + i], number_of_frames);
            }

            $perform(&mut *receiver, number_of_frames, &inlets, &mut outlets);

            buffer_pointer.add(buffer_length)
        }
    };
}

pub unsafe fn read_signal<'a>(
    pointer: pd_sys::t_int,
    number_of_frames: usize,
) -> &'a mut [pd_sys::t_float] {
    let samples = std::mem::transmute::<_, *mut pd_sys::t_sample>(pointer);
    std::slice::from_raw_parts_mut(samples, number_of_frames)
}
