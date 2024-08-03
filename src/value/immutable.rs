use rhai::Dynamic;

use crate::{reify, Shared};
use crate::context::ParseContext;
use crate::error::RhodError;
use crate::path::ImmutablePath;
use crate::value::{AnyRhodValue, RhodValue};

/// Some [RhodValue] that is immutable
#[derive(Clone)]
pub struct RhodImmutable {
    inner: Shared<AnyRhodValue>,
}

impl RhodImmutable {
    #[inline(always)]
    pub fn new<T: RhodValue>(value: T) -> Self {
        reify! { value => // short circuit if already immutable
            |immutable: RhodImmutable| immutable,
            || RhodImmutable { inner: Shared::new(value) }
        }
    }

    fn _parse_visit(
        &self,
        value: &Dynamic,
        path: ImmutablePath,
        parent: ParseContext,
    ) -> Result<Dynamic, RhodError> {
        if value.is_read_only() {
            // probably propagate context here
            self.inner.parse_visit(value, path, parent)
        } else {
            // probably propagate error-context here
            Err(RhodError::TodoError)
        }
    }
}

impl RhodValue for RhodImmutable {
    fn parse_visit(
        &self,
        value: &Dynamic,
        path: ImmutablePath,
        parent: ParseContext,
    ) -> Result<Dynamic, RhodError> {
        // check `Self::_parse_visit` for rough sketch of the code
        todo!()
    }
}

// Convenience traits with blanket implementations
pub trait IntoImmutable {
    /// Make this [RhodValue] immutable, consuming it
    fn immutable(self) -> RhodImmutable;
}

impl<T: RhodValue> IntoImmutable for T {
    #[inline(always)]
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
    #[inline(always)]
    fn immutable_cloned(&self) -> RhodImmutable {
        RhodImmutable::new(self.clone())
    }
}
