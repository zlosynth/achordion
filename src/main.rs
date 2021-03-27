#![no_main]
#![no_std]

use core::convert::Infallible;

use cortex_m::asm::delay;
use panic_halt as _;
use rtic::app;
use rtic::cyccnt::{Instant, U32Ext as _};
use stm32f3xx_hal::{
    gpio::{
        gpioe::{MODER, OTYPER, PE13},
        Output, PushPull,
    },
    prelude::*,
    usb::{Peripheral, UsbBus, UsbBusType},
};
use usb_device::{
    bus::{self, UsbBusAllocator},
    prelude::*,
};
use usbd_midi::{
    data::{
        midi::{message::Message, notes::Note},
        usb::constants::{USB_AUDIO_CLASS, USB_MIDISTREAMING_SUBCLASS},
        usb_midi::midi_packet_reader::MidiPacketBufferReader,
    },
    midi_device::MidiClass,
};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

const PERIOD: u32 = 8_000_000;

static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;

#[app(device = stm32f3xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: Led,
        usb_dev: UsbDevice<'static, UsbBusType>,
        midi: MidiClass<'static, UsbBusType>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let mut rcc = cx.device.RCC.constrain();
        let mut gpioe = cx.device.GPIOE.split(&mut rcc.ahb);

        let mut led = Led::new(gpioe.pe13, &mut gpioe.moder, &mut gpioe.otyper);
        led.set_low().unwrap();

        let mut flash = cx.device.FLASH.constrain();

        let clocks = rcc
            .cfgr
            .use_hse(8u32.mhz())
            .sysclk(48u32.mhz())
            .pclk1(24u32.mhz())
            .pclk2(24u32.mhz())
            .freeze(&mut flash.acr);

        assert!(clocks.usbclk_valid());

        // Configure the on-board LED (LD10, south red)
        let mut led2 = gpioe
            .pe12
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        led2.set_low().ok(); // Turn off

        let mut gpioa = cx.device.GPIOA.split(&mut rcc.ahb);

        // F3 Discovery board has a pull-up resistor on the D+ line.
        // Pull the D+ pin down to send a RESET condition to the USB bus.
        // This forced reset is needed only for development, without it host
        // will not reset your device when you upload new firmware.
        let mut usb_dp = gpioa
            .pa12
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        usb_dp.set_low().ok();
        delay(clocks.sysclk().0 / 100);

        let usb_dm = gpioa.pa11.into_af14(&mut gpioa.moder, &mut gpioa.afrh);
        let usb_dp = usb_dp.into_af14(&mut gpioa.moder, &mut gpioa.afrh);

        let usb = Peripheral {
            usb: cx.device.USB,
            pin_dm: usb_dm,
            pin_dp: usb_dp,
        };

        unsafe {
            USB_BUS = Some(UsbBus::new(usb));
        }

        let midi = MidiClass::new(unsafe { USB_BUS.as_ref().unwrap() });

        let usb_dev = UsbDeviceBuilder::new(
            unsafe { USB_BUS.as_ref().unwrap() },
            UsbVidPid(0x16c0, 0x27dd),
        )
        .manufacturer("Zlosynth")
        .product("Achordion")
        .serial_number("0")
        .device_class(USB_AUDIO_CLASS)
        .device_sub_class(USB_MIDISTREAMING_SUBCLASS)
        .build();

        init::LateResources { led, usb_dev, midi }
    }

    #[task(binds = USB_HP_CAN_TX, resources = [usb_dev, midi, led])]
    fn usb_tx(mut cx: usb_tx::Context) {
        usb_poll(
            &mut cx.resources.usb_dev,
            &mut cx.resources.midi,
            &mut cx.resources.led,
        );
    }

    #[task(binds = USB_LP_CAN_RX0, resources = [usb_dev, midi, led])]
    fn usb_rx0(mut cx: usb_rx0::Context) {
        usb_poll(
            &mut cx.resources.usb_dev,
            &mut cx.resources.midi,
            &mut cx.resources.led,
        );
    }

    extern "C" {
        fn EXTI0();
    }
};

pub struct Led {
    led: PE13<Output<PushPull>>,
}

impl Led {
    pub fn new<T>(pe13: PE13<T>, moder: &mut MODER, otyper: &mut OTYPER) -> Self {
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

fn usb_poll<B: bus::UsbBus>(
    usb_dev: &mut UsbDevice<'static, B>,
    midi: &mut MidiClass<'static, B>,
    led: &mut Led,
) {
    if !usb_dev.poll(&mut [midi]) {
        return;
    }

    let mut buffer = [0u8; 64];

    if let Ok(size) = midi.read(&mut buffer) {
        let buffer_reader = MidiPacketBufferReader::new(&buffer, size);
        for packet in buffer_reader.into_iter() {
            if let Ok(packet) = packet {
                match packet.message {
                    Message::NoteOn(Channel1, Note::C2, ..) => {
                        led.set_high().unwrap();
                    }
                    Message::NoteOff(Channel1, Note::C2, ..) => {
                        led.set_low().unwrap();
                    }
                    _ => {}
                }
            }
        }
    }
}
