use achordion_lib::store::{Parameters, Store};

use crate::system::flash::Flash;

const NUM_SECTORS: usize = 2048;

fn sector_address(sector_index: usize) -> u32 {
    (sector_index << 12) as u32
}

pub struct Storage {
    flash: Flash,
    version: u16,
}

impl Storage {
    pub fn new(flash: Flash) -> Self {
        Self { flash, version: 0 }
    }

    pub fn save_parameters(&mut self, parameters: Parameters) {
        let data = Store::new(parameters, self.version).to_bytes();
        self.flash
            .write(sector_address(self.version as usize % NUM_SECTORS), &data);
        self.version += 1;
    }

    pub fn load_parameters(&mut self) -> Parameters {
        let mut latest_store: Option<Store> = None;

        for i in 0..NUM_SECTORS {
            let mut store_buffer = [0; Store::SIZE];

            self.flash.read(sector_address(i), &mut store_buffer);

            if let Ok(store) = Store::from_bytes(store_buffer) {
                if let Some(latest) = latest_store {
                    if store.version() > latest.version() {
                        latest_store = Some(store);
                    }
                } else {
                    latest_store = Some(store);
                }
            }
        }

        if let Some(latest) = latest_store {
            self.version = latest.version() + 1;
            latest.parameters()
        } else {
            Parameters::default()
        }
    }
}
