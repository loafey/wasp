use std::{
    fmt::{Debug, Display},
    ops::Deref,
    sync::{Arc, RwLock as STDLock, Weak as WeakArc},
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

pub enum Ptr<T> {
    Strong { inner: Arc<T> },
    Weak { inner: WeakArc<T> },
}
impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        match self {
            Ptr::Strong { inner } => Ptr::Weak {
                inner: Arc::downgrade(inner),
            },
            Ptr::Weak { inner } => Ptr::Weak {
                inner: inner.clone(),
            },
        }
    }
}
impl<T> From<T> for Ptr<T> {
    fn from(value: T) -> Self {
        Self::Strong {
            inner: Arc::new(value),
        }
    }
}
impl<T> AsRef<T> for Ptr<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}
impl<T> Deref for Ptr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Ptr::Strong { inner } => inner,
            Ptr::Weak { inner } => match inner.upgrade() {
                Some(_) => unsafe { &*inner.as_ptr() },
                None => panic!("null pointer exception"),
            },
        }
    }
}
impl<T: Debug> Debug for Ptr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T: Display> Display for Ptr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

pub struct PtrRW<T> {
    inner: Ptr<RwLock<T>>,
}
impl<T> Clone for PtrRW<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
impl<T> Deref for PtrRW<T> {
    type Target = RwLock<T>;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
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
            inner: RwLock::new(value).into(),
        }
    }
}
