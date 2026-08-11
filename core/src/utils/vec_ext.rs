use std::ptr;

pub trait VecExt<T> {
    unsafe fn swap_remove_unchecked(&mut self, index: usize) -> T;
}

impl<T> VecExt<T> for Vec<T> {
    unsafe fn swap_remove_unchecked(&mut self, index: usize) -> T {
        unsafe {
            let len = self.len();
            let value = ptr::read(self.as_ptr().add(index));
            let base_ptr = self.as_mut_ptr();
            ptr::copy(base_ptr.add(len - 1), base_ptr.add(index), 1);
            self.set_len(len - 1);
            value
        }
    }
}
