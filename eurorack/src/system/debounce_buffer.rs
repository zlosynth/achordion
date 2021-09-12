pub struct DebounceBuffer<const N: usize> {
    buffer: [bool; N],
    pointer: usize,
}

impl<const N: usize> DebounceBuffer<N> {
    pub fn new() -> Self {
        Self {
            buffer: [false; N],
            pointer: 0,
        }
    }

    pub fn write(&mut self, value: bool) {
        self.buffer[self.pointer] = value;
        self.pointer = (self.pointer + 1) % N;
    }

    pub fn read(&self) -> bool {
        let up: usize = self.buffer.iter().filter(|i| **i).count();
        up > N / 2
    }
}
