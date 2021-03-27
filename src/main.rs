#![no_main]
#![no_std]

use core::convert::Infallible;

use cortex_m::asm::delay;
use panic_halt as _;
use rtic::app;
use stm32f3xx_hal::{
    gpio::{
        gpioa::{self, PA11, PA12},
        gpioe::{self, PE13},
        Output, PushPull,
    },
    prelude::*,
    rcc::Clocks,
    usb::{Peripheral, UsbBus, UsbBusType},
};
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_midi::{
    data::{
        midi::{message::Message, notes::Note},
        usb::constants::{USB_AUDIO_CLASS, USB_MIDISTREAMING_SUBCLASS},
        usb_midi::midi_packet_reader::MidiPacketBufferReader,
    },
    midi_device::MidiClass,
};

static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;

#[app(device = stm32f3xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: Led,
        midi: Midi,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let mut rcc = cx.device.RCC.constrain();
        let mut gpioa = cx.device.GPIOA.split(&mut rcc.ahb);
        let mut gpioe = cx.device.GPIOE.split(&mut rcc.ahb);
        let mut flash = cx.device.FLASH.constrain();

        let clocks = rcc
            .cfgr
            .use_hse(8u32.mhz())
            .sysclk(48u32.mhz())
            .pclk1(24u32.mhz())
            .pclk2(24u32.mhz())
            .freeze(&mut flash.acr);

        let mut led = Led::new(gpioe.pe13, &mut gpioe.moder, &mut gpioe.otyper);
        led.set_low().unwrap();

        let midi = Midi::new(
            gpioa.pa11,
            gpioa.pa12,
            cx.device.USB,
            &clocks,
            &mut gpioa.moder,
            &mut gpioa.afrh,
            &mut gpioa.otyper,
        );

        init::LateResources { led, midi }
    }

    #[task(binds = USB_HP_CAN_TX, resources = [midi, led])]
    fn usb_tx(mut cx: usb_tx::Context) {
        midi_poll(&mut cx.resources.midi, &mut cx.resources.led);
    }

    #[task(binds = USB_LP_CAN_RX0, resources = [midi, led])]
    fn usb_rx0(mut cx: usb_rx0::Context) {
        midi_poll(&mut cx.resources.midi, &mut cx.resources.led);
    }

    extern "C" {
        fn EXTI0();
    }
};

pub struct Led {
    led: PE13<Output<PushPull>>,
}

impl Led {
    pub fn new<T>(pe13: PE13<T>, moder: &mut gpioe::MODER, otyper: &mut gpioe::OTYPER) -> Self {
        let led = pe13.into_push_pull_output(moder, otyper);
        Self { led }
    }

    pub fn set_low(&mut self) -> Result<(), Infallible> {
        self.led.set_low()
    }

    pub fn set_high(&mut self) -> Result<(), Infallible> {
        self.led.set_high()
    }
}

pub struct Midi {
    pub usb_device: UsbDevice<'static, UsbBusType>,
    pub midi_class: MidiClass<'static, UsbBusType>,
}

impl Midi {
    pub fn new<T1, T2>(
        pa11: PA11<T1>,
        pa12: PA12<T2>,
        usb: stm32f3xx_hal::pac::USB,
        clocks: &Clocks,
        moder: &mut gpioa::MODER,
        afrg: &mut gpioa::AFRH,
        otyper: &mut gpioa::OTYPER,
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

        unsafe {
            USB_BUS = Some(UsbBus::new(usb_peripheral));
        }

        let midi_class = MidiClass::new(unsafe { USB_BUS.as_ref().unwrap() });

        let usb_device = UsbDeviceBuilder::new(
            unsafe { USB_BUS.as_ref().unwrap() },
            UsbVidPid(0x16c0, 0x27dd),
        )
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

fn midi_poll(midi: &mut Midi, led: &mut Led) {
    if !midi.usb_device.poll(&mut [&mut midi.midi_class]) {
        return;
    }

    let mut buffer = [0u8; 64];

    if let Ok(size) = midi.midi_class.read(&mut buffer) {
        let buffer_reader = MidiPacketBufferReader::new(&buffer, size);
        for packet in buffer_reader.into_iter() {
            if let Ok(packet) = packet {
                match packet.message {
                    Message::NoteOn(_, Note::C2, ..) => {
                        led.set_high().unwrap();
                    }
                    Message::NoteOff(_, Note::C2, ..) => {
                        led.set_low().unwrap();
                    }
                    _ => {}
                }
            }
        }
    }
}
