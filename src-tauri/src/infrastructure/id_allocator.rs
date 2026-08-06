use std::sync::atomic::{AtomicU64, Ordering};
use std::marker::PhantomData;


// 泛型 Id 分配器
pub struct IdAllocator<Id> {
    next: AtomicU64,
    _marker: PhantomData<Id>
}

impl<Id> IdAllocator<Id> {
    pub const fn new() -> Self {
        Self { next: AtomicU64::new(1), _marker: PhantomData }
    }
    pub fn allocate(&self) -> Id
    where 
        Id: From<u64>,
    {
        let id: u64 = self.next.fetch_add(1, Ordering::Relaxed);
        id.into()
    }
}

