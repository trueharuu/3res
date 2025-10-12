pub mod coordinate;
pub mod rotation;
pub mod color;
pub mod traits;
pub mod defaultdict;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct UnknownVariant(pub String);