//! # Id 分配器
//! 
//! 提供线程安全的泛型 Id 生成器.
//! 
//! # 注意
//! 本模块尚未被正式使用.


use std::sync::atomic::{AtomicU64, Ordering};
use std::marker::PhantomData;


/// 泛型 Id 分配器.
/// 可生成任意实现了 `From<u64>` 的 Id 类型.
/// 
/// 内部使用 `AtomicU64` 保证线程安全, 分配从 1 开始.
///
/// # 注意
/// 当前尚未正式使用.
pub struct IdAllocator<Id> {
    next: AtomicU64,
    _marker: PhantomData<Id>
}

impl<Id> IdAllocator<Id> {
    pub const fn new() -> Self {
        Self { next: AtomicU64::new(1), _marker: PhantomData }
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

