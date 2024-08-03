use crate::{ImmutableArray, ImmutableString, MutableArray};

/// Reference-counted pointer to an immutable array of [PathComponent]s
pub type ImmutablePath = ImmutableArray<PathComponent>;

/// A contiguous growable array of [PathComponent]s
pub type MutablePath = MutableArray<PathComponent>;

pub enum PathComponent {
    Key(ImmutableString),
    Index(usize),
}
