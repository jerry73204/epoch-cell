use std::sync::atomic::Ordering::*;

use crossbeam::epoch::{self, Atomic, Guard, Owned};

#[derive(Debug)]
pub struct EpochCell<T> {
    atomic: Atomic<T>,
}

impl<T> EpochCell<T> {
    pub fn init<I>(init: I) -> Self
    where
        I: Into<Option<T>>,
    {
        let atomic = match init.into() {
            Some(init) => Atomic::new(init),
            None => Atomic::null(),
        };

        Self { atomic }
    }

    pub fn new(init: T) -> Self {
        Self {
            atomic: Atomic::new(init),
        }
    }

    pub fn null() -> Self {
        Self {
            atomic: Atomic::null(),
        }
    }

    pub fn pinned(&self) -> GuardedCell<'_, T> {
        GuardedCell {
            guard: epoch::pin(),
            atomic: &self.atomic,
        }
    }
}

pub struct GuardedCell<'g, T> {
    guard: Guard,
    atomic: &'g Atomic<T>,
}

impl<'g, T> GuardedCell<'g, T> {
    pub fn as_ref(&'g self) -> Option<&'g T> {
        unsafe { self.atomic.load_consume(&self.guard).as_ref() }
    }

    pub fn set(&'g self, value: T) -> Option<&'g T> {
        let value = Owned::new(value);
        let orig = self.atomic.swap(value, AcqRel, &self.guard);
        unsafe { orig.as_ref() }
    }

    pub fn is_null(&self) -> bool {
        let shared = self.atomic.load_consume(&self.guard);
        shared.is_null()
    }
}
