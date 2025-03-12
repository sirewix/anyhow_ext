#![doc = include_str!("../README.md")]

use core::convert::Infallible;
use std::fmt::Display;
use std::panic::Location;

/// re-exports
pub use anyhow::{anyhow, bail, ensure};
pub use anyhow::{Chain, Error, Ok, Result};

pub trait Context<T, E> {
    fn context<C>(self, context: C) -> Result<T>
    where
        C: Display + Send + Sync + 'static;

    fn with_context<C, F>(self, context: F) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;

    /// like google map red dot, only record the location info without any context message.
    fn dot(self) -> Result<T>;
}

impl<T, E> Context<T, E> for Result<T, E>
where
    E: Display,
    Result<T, E>: anyhow::Context<T, E>,
{
    #[inline]
    #[track_caller]
    fn context<C>(self, context: C) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
    {
        let caller = Location::caller();
        anyhow::Context::context(
            self,
            WithLocation {
                error: context,
                location: caller,
            },
        )
    }

    #[inline]
    #[track_caller]
    fn with_context<C, F>(self, context: F) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        let caller = Location::caller();
        anyhow::Context::with_context(self, || WithLocation {
            error: context(),
            location: caller,
        })
    }

    #[inline]
    #[track_caller]
    fn dot(self) -> Result<T> {
        let caller = Location::caller();
        anyhow::Context::context(self, format!("at `{}`", caller))
    }
}

impl<T> Context<T, Infallible> for Option<T>
where
    Option<T>: anyhow::Context<T, Infallible>,
{
    #[inline]
    #[track_caller]
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
    {
        let caller = Location::caller();
        anyhow::Context::context(
            self,
            WithLocation {
                error: context,
                location: caller,
            },
        )
    }

    #[inline]
    #[track_caller]
    fn with_context<C, F>(self, context: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        let caller = Location::caller();
        anyhow::Context::with_context(self, || WithLocation {
            error: context(),
            location: caller,
        })
    }

    #[inline]
    #[track_caller]
    fn dot(self) -> Result<T> {
        let caller = Location::caller();
        anyhow::Context::context(self, format!("at `{}`", caller))
    }
}

#[derive(Clone, Debug)]
pub struct WithLocation<E> {
    pub error: E,
    pub location: &'static Location<'static>,
}

impl<E: std::fmt::Display> std::fmt::Display for WithLocation<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} at `{}`", self.error, self.location)
    }
}

impl<E> AsRef<E> for WithLocation<E> {
    fn as_ref(&self) -> &E {
        &self.error
    }
}

impl<E> std::ops::Deref for WithLocation<E> {
    type Target = E;
    fn deref(&self) -> &Self::Target {
        &self.error
    }
}
