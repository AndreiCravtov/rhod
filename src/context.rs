use rhai::Dynamic;

use crate::{ImmutableArray, ImmutableString, Shared, SharedWeak};
use crate::error::RhodErrorMap;
use crate::issue::RhodIssue;
use crate::path::ImmutablePath;

pub struct ParseContext {
    common_issues: ImmutableArray<RhodIssue>,
    common_contextual_error_map: Option<Shared<RhodErrorMap>>,
    path: ImmutablePath,
    schema_error_map: Option<Shared<RhodErrorMap>>,
    data: Shared<Dynamic>,
    parsed_type: ImmutableString, // TODO: in JS its an enum ["null", "array", "object"] etc.
    parent: SharedWeak<ParseContext>, // references to parent are usually "weak"
                                  // TODO: may change to `Option<Shared<ParseContext>>` to be closer to JS impl
}
