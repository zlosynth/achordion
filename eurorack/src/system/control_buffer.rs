#[allow(unused_imports)]
use micromath::F32Ext;

pub struct ControlBuffer<const N: usize> {
    buffer: [f32; N],
    pointer: usize,
}

impl<const N: usize> ControlBuffer<N> {
    pub fn new() -> Self {
        Self {
            buffer: [0.0; N],
            pointer: 0,
        }
    }

    pub fn write(&mut self, value: f32) {
        self.buffer[self.pointer] = value;
        self.pointer = (self.pointer + 1) % N;
    }

    pub fn read(&self) -> f32 {
        let sum: f32 = self.buffer.iter().sum();
        sum / N as f32
    }

    pub fn traveled(&self) -> f32 {
        // NOTE: This does not panic only thanks to the release mode
        // wrapping around instead of exploding on underflow.
        // XXX: This works only if N is power of 2.
        let newest = (self.pointer - 1).rem_euclid(N);
        let oldest = self.pointer;
        (self.buffer[newest] - self.buffer[oldest]).abs()
    }
}
