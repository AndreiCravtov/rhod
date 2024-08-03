/////////////////////////////////////////
/////////////////////////////////////////
//////////                     //////////
//////////     RhodObject      //////////
//////////                     //////////
/////////////////////////////////////////
/////////////////////////////////////////

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
    pub fn parse_visit(
        &self,
        value: Dynamic,
        path: ImmutablePath,
        parent: ParseContext,
    ) -> Result<(), ()> {
        todo!()
    }
}

impl RhodValue for RhodObject {
    fn parse_visit(&self, value: &Dynamic) -> Result<Dynamic, RhodError> {
        todo!()
    }
}
