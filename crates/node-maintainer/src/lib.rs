pub mod error;
mod set_relation;
pub mod term;

#[derive(Debug, Clone)]
pub struct Incompat {
    cause: IncompatCause,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IncompatCause {
    Root,
    NoVersions,
    Dependency,
}
