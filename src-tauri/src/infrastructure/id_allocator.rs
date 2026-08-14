//! # Id 分配器
//! 
//! 提供线程安全的泛型 Id 生成器.


use std::sync::atomic::{AtomicU64, Ordering};
use std::marker::PhantomData;


/// 泛型 Id 自增分配器.
/// 可生成任意实现了 `From<u64>` 的 Id 类型.
/// 
/// 内部使用 `AtomicU64` 保证线程安全, 分配默认从 1 开始.
/// 通过 `reset` 可以让分配从指定值重新开始
pub struct IdAllocator<Id> {
    next: AtomicU64,
    _marker: PhantomData<Id>
}

impl<Id> IdAllocator<Id> {
    /// 创建一个新的 Id 分配器.
    pub const fn new() -> Self {
        Self {
            next: AtomicU64::new(1),
            _marker: PhantomData,
        }
    }

    /// 从指定值开始分配.
    pub fn reset(&self, max_id: u64) {
        self.next.store(max_id + 1, Ordering::SeqCst);
    }

    /// 分配一个新 Id.
    /// 
    /// # 要求
    /// `Id` 必须实现 `From<u64>`.
    pub fn allocate(&self) -> Id
    where 
        Id: From<u64>,
    {
        let id: u64 = self.next.fetch_add(1, Ordering::Relaxed);
        id.into()
    }
}

