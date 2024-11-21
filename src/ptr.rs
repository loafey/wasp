use std::{
    fmt::{Debug, Display},
    ops::Deref,
    sync::{Arc, RwLock as STDLock},
};

pub struct RwLock<T> {
    lock: STDLock<T>,
}
impl<T> RwLock<T> {
    pub fn new(t: T) -> Self {
        Self {
            lock: STDLock::new(t),
        }
    }
    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, T> {
        self.lock.write().expect("poison!")
    }

    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, T> {
        self.lock.read().expect("poison!")
    }
}
impl<T: Debug> Debug for RwLock<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.lock.fmt(f)
    }
}

pub struct Ptr<T> {
    inner: Arc<T>,
}
impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
impl<T> From<T> for Ptr<T> {
    fn from(value: T) -> Self {
        Self {
            inner: Arc::new(value),
        }
    }
}
impl<T> Deref for Ptr<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<T: Debug> Debug for Ptr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T: Display> Display for Ptr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

pub struct PtrRW<T> {
    inner: Arc<RwLock<T>>,
}
impl<T> Clone for PtrRW<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
impl<T> Deref for PtrRW<T> {
    type Target = Arc<RwLock<T>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<T: Debug> Debug for PtrRW<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
impl<T> From<T> for PtrRW<T> {
    fn from(value: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(value)),
        }
    }
}
