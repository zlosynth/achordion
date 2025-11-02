use rtic::cyccnt::U32Ext as _;
use rtic::cyccnt::{Duration, Instant};

use super::debounce_buffer::DebounceBuffer;
use crate::system::hal::hal::digital::v2::InputPin;

const LONG_CLICK: u32 = 5 * crate::SECOND;

pub struct Button<P> {
    pin: P,
    debounce_filter: DebounceBuffer<5>,
    active_for: Option<Duration>,
    active_timestamp: Option<Instant>,
    clicked: bool,
    released: bool,
    long_clicked: bool,
}

impl<P: InputPin> Button<P> {
    pub fn new(pin: P) -> Self {
        Self {
            pin,
            debounce_filter: DebounceBuffer::new(),
            active_for: None,
            active_timestamp: None,
            clicked: false,
            released: false,
            long_clicked: false,
        }
    }

    pub fn sample(&mut self) {
        self.debounce_filter.write(self.pin.is_low().ok().unwrap());
        let is_active = self.debounce_filter.read();
        let was_active = self.active_for.is_some();

        self.clicked = !was_active && is_active;
        self.released = was_active && !is_active;

        self.long_clicked = false;
        if is_active {
            let new_timestamp = Instant::now();

            self.active_for = if was_active {
                let old_timestamp = self.active_timestamp.unwrap();
                Some(Duration::from_cycles(
                    self.active_for
                        .unwrap()
                        .as_cycles()
                        .saturating_add(new_timestamp.duration_since(old_timestamp).as_cycles()),
                ))
            } else {
                Some(Duration::from_cycles(0))
            };

            self.active_timestamp = Some(new_timestamp);
        } else {
            if was_active {
                self.long_clicked = self.active_for.unwrap() > LONG_CLICK.cycles();
            }
            self.long_click_reset();
        }
    }

    pub fn active(&self) -> bool {
        self.debounce_filter.read()
    }

    pub fn active_no_filter(&self) -> bool {
        self.pin.is_low().ok().unwrap()
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }

    pub fn released(&self) -> bool {
        self.released
    }

    pub fn long_clicked(&self) -> bool {
        self.long_clicked
    }

    pub fn long_click_reset(&mut self) {
        self.active_for = None;
        self.active_timestamp = None;
    }
}
