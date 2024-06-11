use std::ptr::NonNull;

pub struct NodeRef<T> {
    ptr: NonNull<T>
}

impl<T> NodeRef<T> {
    pub unsafe fn new(v: *mut T) -> Self {
        NodeRef {
            ptr: NonNull::new_unchecked(v)
        }
    }

    pub unsafe fn as_mut<'a>(&self) -> &'a mut T {
        &mut *self.as_ptr()
    }

    pub unsafe fn as_ref<'a>(&self) -> &'a T {
        &*self.as_ptr()
    }

    pub unsafe fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}