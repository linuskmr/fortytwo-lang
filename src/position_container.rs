

use std::fmt::{Debug};



#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PositionContainer<T> {
    /// The data of this container.
    pub data: T,
    pub position: miette::SourceSpan,
}