/////////////////////////////////////////
/////////////////////////////////////////
//////////                     //////////
//////////     RhodImmutable   //////////
//////////                     //////////
/////////////////////////////////////////
/////////////////////////////////////////

use rhai::Dynamic;

use crate::error::RhodError;
use crate::reify;
use crate::value::{AnyRhodValue, RhodValue};

/// Some [RhodValue] that is immutable
pub struct RhodImmutable {
    inner: Box<AnyRhodValue>,
}

impl RhodImmutable {
    pub fn new<T: RhodValue>(value: T) -> Self {
        reify! { value => // short circuit if already immutable
            |immutable: RhodImmutable| immutable,
            || RhodImmutable { inner: Box::new(value) }
        }
    }
}

impl RhodValue for RhodImmutable {
    fn parse_visit(&self, value: &Dynamic) -> Result<Dynamic, RhodError> {
        if value.is_read_only() {
            // probably propagate context here
            self.inner.parse_visit(value)
        } else {
            // probably propagate error-context here
            Err(RhodError::TodoError)
        }
    }
}

// Convenience traits with blanket implementations
pub trait IntoImmutable {
    /// Make this [RhodValue] immutable, consuming it
    fn immutable(self) -> RhodImmutable;
}

impl<T: RhodValue> IntoImmutable for T {
    fn immutable(self) -> RhodImmutable {
        RhodImmutable::new(self)
    }
}

pub trait IntoImmutableCloned: Clone {
    /// Make this [RhodValue] immutable, cloning it
    ///
    fn immutable_cloned(&self) -> RhodImmutable;
}

impl<T: RhodValue + Clone> IntoImmutableCloned for T {
    fn immutable_cloned(&self) -> RhodImmutable {
        RhodImmutable::new(self.clone())
    }
}
