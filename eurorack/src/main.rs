#![no_main]
#![no_std]

mod led;
mod midi;

use panic_halt as _;
use rtic::app;
use stm32f3xx_hal::{prelude::*, usb::UsbBusType};
use usb_device::bus::UsbBusAllocator;
use usbd_midi::{
    data::{
        midi::{message::Message, notes::Note},
        usb_midi::{
            midi_packet_reader::MidiPacketBufferReader, usb_midi_event_packet::UsbMidiEventPacket,
        },
    },
    midi_device::MAX_PACKET_SIZE,
};

use crate::led::Led;
use crate::midi::Midi;

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
            unsafe { &mut USB_BUS },
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

fn midi_poll(midi: &mut Midi, led: &mut Led) {
    if !midi.usb_device.poll(&mut [&mut midi.midi_class]) {
        return;
    }

    let mut buffer = [0u8; MAX_PACKET_SIZE];

    if let Ok(size) = midi.midi_class.read(&mut buffer) {
        let buffer_reader = MidiPacketBufferReader::new(&buffer, size);
        for packet in buffer_reader.into_iter() {
            if let Ok(packet) = packet {
                process_midi_message(packet, led);
            }
        }
    }
}

fn process_midi_message(packet: UsbMidiEventPacket, led: &mut Led) {
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
