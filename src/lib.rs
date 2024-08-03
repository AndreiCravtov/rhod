// TODO: invest in configuring linting and formatting
#![warn(clippy::missing_inline_in_public_items)]

mod context;
mod error;
mod issue;
mod path;
mod value;

pub(crate) mod reify;

/// Internal convenience trait for [Send] and [Sync] together
#[cfg(feature = "sync")]
pub(crate) trait SendSync: Send + Sync {}

#[cfg(feature = "sync")]
impl<T: Send + Sync> SendSync for T {}

/// Reference-counted smart pointer
#[cfg(not(feature = "sync"))]
pub type Shared<T> = std::rc::Rc<T>;

/// Weak reference counted smart pointer
#[cfg(not(feature = "sync"))]
pub type SharedWeak<T> = Shared<std::rc::Weak<T>>;

/// Reference-counted smart pointer
#[cfg(feature = "sync")]
pub type Shared<T> = std::sync::Arc<T>;

/// Weak reference counted smart pointer
#[cfg(feature = "sync")]
pub type SharedWeak<T> = Shared<std::sync::Weak<T>>;

/// Reference-counted pointer to an immutable string
pub type ImmutableString = Shared<str>;

/// Growable string type
pub type MutableString = String;

/// Reference-counted pointer to an immutable array
pub type ImmutableArray<T> = Shared<[T]>;

/// A contiguous growable array type
pub type MutableArray<T> = Vec<T>;

/// Map type, for retrieving values by key
pub type Map<K, V> = std::collections::BTreeMap<K, V>;
