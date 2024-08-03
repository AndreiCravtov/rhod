use rhai::Dynamic;

use crate::{ImmutableString, Map};
use crate::context::ParseContext;
use crate::error::RhodError;
use crate::path::ImmutablePath;
use crate::value::{AnyRhodValue, RhodValue};

pub type ObjectKey = ImmutableString;
pub struct RhodObject {
    shape: Map<ObjectKey, Box<AnyRhodValue>>,
}

impl RhodObject {
    #[inline(always)]
    pub fn new(shape: Map<ObjectKey, Box<AnyRhodValue>>) -> Self {
        Self { shape }
    }

    fn _parse_visit(
        &self,
        value: &Dynamic,
        path: ImmutablePath,
        parent: ParseContext,
    ) -> Result<Dynamic, RhodError> {
        // may allow for stronger type guarantees than just "dynamic" with optional <Output> type??
        todo!()
    }
}

impl RhodValue for RhodObject {
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
