use std::any::Any;

use rhai::Dynamic;

use crate::error::RhodError;

mod immutable;
mod object;
/////////////////////////////////////////
/////////////////////////////////////////
//////////                     //////////
//////////      RhodValue      //////////
//////////                     //////////
/////////////////////////////////////////
/////////////////////////////////////////

pub trait RhodValue: Any {
    fn parse_visit(&self, value: &Dynamic) -> Result<Dynamic, RhodError>;
}

/// The [RhodValue] trait object, which could be any [RhodValue]
pub type AnyRhodValue = dyn RhodValue;
