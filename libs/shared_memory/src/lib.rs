pub struct SharedTensor;

impl SharedTensor {
    pub fn create(_name: &str, _size: usize) -> Self {
        SharedTensor
    }
    pub fn write(&mut self, _data: &[f32]) {}
}
