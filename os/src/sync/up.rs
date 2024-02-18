use core::cell::{RefCell, RefMut};


pub struct UPSafeCell<T> {
    data: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    pub unsafe fn new(value: T) -> Self {
        Self { data: RefCell::new(value) }
    }


    pub fn borrow(&self) -> RefMut<'_, T> {
        self.data.borrow_mut()
    }
}