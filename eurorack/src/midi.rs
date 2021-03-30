use cortex_m::asm::delay;
use panic_halt as _;
use stm32f3xx_hal::{
    gpio::gpioa::{self, PA11, PA12},
    prelude::*,
    rcc::Clocks,
    usb::{Peripheral, UsbBus, UsbBusType},
};
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_midi::{
    data::usb::constants::{USB_AUDIO_CLASS, USB_MIDISTREAMING_SUBCLASS},
    midi_device::MidiClass,
};

pub struct Midi {
    pub usb_device: UsbDevice<'static, UsbBusType>,
    pub midi_class: MidiClass<'static, UsbBusType>,
}

impl Midi {
    #[allow(clippy::too_many_arguments)]
    pub fn new<T1, T2>(
        pa11: PA11<T1>,
        pa12: PA12<T2>,
        usb: stm32f3xx_hal::pac::USB,
        clocks: &Clocks,
        moder: &mut gpioa::MODER,
        afrg: &mut gpioa::AFRH,
        otyper: &mut gpioa::OTYPER,
        usb_bus: &'static mut Option<UsbBusAllocator<UsbBusType>>,
    ) -> Self {
        assert!(clocks.usbclk_valid());

        // F3 Discovery board has a pull-up resistor on the D+ line.
        // Pull the D+ pin down to send a RESET condition to the USB bus.
        // This forced reset is needed only for development, without it host
        // will not reset your device when you upload new firmware.
        let mut usb_dp = pa12.into_push_pull_output(moder, otyper);
        usb_dp.set_low().ok();
        delay(clocks.sysclk().0 / 100);

        let usb_dm = pa11.into_af14(moder, afrg);
        let usb_dp = usb_dp.into_af14(moder, afrg);

        let usb_peripheral = Peripheral {
            usb,
            pin_dm: usb_dm,
            pin_dp: usb_dp,
        };

        *usb_bus = Some(UsbBus::new(usb_peripheral));

        let midi_class = MidiClass::new(usb_bus.as_ref().unwrap());

        let usb_device =
            UsbDeviceBuilder::new(usb_bus.as_ref().unwrap(), UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("Zlosynth")
                .product("Achordion")
                .serial_number("0")
                .device_class(USB_AUDIO_CLASS)
                .device_sub_class(USB_MIDISTREAMING_SUBCLASS)
                .build();

        Self {
            usb_device,
            midi_class,
        }
    }
}
