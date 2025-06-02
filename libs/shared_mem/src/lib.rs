use memmap2::MmapMut;
use std::fs::OpenOptions;

pub struct SharedTensor {
    mmap: MmapMut,
}

impl SharedTensor {
    pub fn create(name: &str, size: usize) -> Self {
        let file = OpenOptions::new()
            .read(true).write(true).create(true)
            .open(name).unwrap();
        file.set_len(size as u64).unwrap();
        Self { mmap: unsafe { MmapMut::map_mut(&file).unwrap() } }
    }

    pub fn write(&mut self, data: &[f32]) {
        self.mmap[..data.len() * std::mem::size_of::<f32>()].copy_from_slice(unsafe {
            std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * std::mem::size_of::<f32>()
            )
        });
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
