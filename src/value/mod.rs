use std::any::Any;

use rhai::Dynamic;

use crate::context::ParseContext;
use crate::error::RhodError;
use crate::path::ImmutablePath;

pub mod immutable;
pub mod object;

/////////////////////////////////////////
/////////////////////////////////////////
//////////                     //////////
//////////      RhodValue      //////////
//////////                     //////////
/////////////////////////////////////////
/////////////////////////////////////////

pub trait RhodValue: Any {
    fn parse_visit(
        &self,
        value: &Dynamic,
        path: ImmutablePath,
        parent: ParseContext,
    ) -> Result<Dynamic, RhodError>;
}

/// The [RhodValue] trait object, which could be any [RhodValue]
pub type AnyRhodValue = dyn RhodValue;
