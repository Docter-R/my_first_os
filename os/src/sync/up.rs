use core::cell::{RefCell, RefMut};

/// Wrap a static data structure inside it so that we are
/// able to access it without any `unsafe`.
///
/// We should only use it in uniprocessor.
pub struct UPSafeCell<T> {
    /// inner data
    inner: RefCell<T>,
    /// 调试信息：记录当前持有锁的代码位置 (文件名, 行号)
    trace: RefCell<Option<(&'static str, u32)>>,
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    /// User is responsible to guarantee that inner struct is only used in
    /// uniprocessor.
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
            trace: RefCell::new(None),
        }
    }

    /// Panic if the data has been borrowed.
    /// 关键修改：添加 #[track_caller] 属性
    #[track_caller] 
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        // 1. 尝试获取锁，而不是直接 borrow_mut (后者会直接 panic)
        match self.inner.try_borrow_mut() {
            Ok(guard) => {
                // 2. 如果成功拿到锁，记录当前调用者的位置
                let location = core::panic::Location::caller();
                // 将 (文件名, 行号) 写入 trace
                *self.trace.borrow_mut() = Some((location.file(), location.line()));
                guard
            }
            Err(_) => {
                // 3. 如果获取失败，说明有人占着锁。读取 trace 里的信息并报错
                let trace = self.trace.borrow();
                if let Some((file, line)) = *trace {
                    panic!(
                        "UPSafeCell: already borrowed! \n  -> Current Owner: {}:{} \n  -> New Request: {}:{}", 
                        file, line, 
                        core::panic::Location::caller().file(), core::panic::Location::caller().line()
                    );
                } else {
                    panic!("UPSafeCell: already borrowed (No trace info)!");
                }
            }
        }
    }
}