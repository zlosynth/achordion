use achordion_lib::store::{Parameters, Store};

use crate::system::flash::Flash;

const STORE_ADDRESSES: [u32; 2] = [0x0000, 0x1000];

pub struct Storage {
    flash: Flash,
}

impl Storage {
    pub fn new(flash: Flash) -> Self {
        Self { flash }
    }

    pub fn save_parameters(&mut self, parameters: Parameters, version: u16) {
        let data = Store::new(parameters, version).to_bytes();
        self.flash.write(
            STORE_ADDRESSES[version as usize % STORE_ADDRESSES.len()],
            &data,
        );
    }

    pub fn load_parameters(&mut self) -> Parameters {
        let mut store_buffer = [0; Store::SIZE];

        let mut stores = [None; STORE_ADDRESSES.len()];
        for (i, address) in STORE_ADDRESSES.iter().enumerate() {
            self.flash.read(*address, &mut store_buffer);
            stores[i] = Store::from_bytes(store_buffer).ok();
        }

        let mut latest_store: Option<Store> = None;

        for store in stores.iter().flatten() {
            if let Some(latest) = latest_store {
                if store.version() > latest.version() {
                    latest_store = Some(*store);
                }
            } else {
                latest_store = Some(*store);
            }
        }

        if let Some(latest) = latest_store {
            latest.parameters()
        } else {
            Parameters::default()
        }
    }
}
