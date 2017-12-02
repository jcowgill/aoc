//! Useful functions used by various AOC days

/// Function type for all star functions
pub type StarFunction = fn (&str) -> String;

/// Type returned by year stars() functions.
///  The vector is always sorted by name and maps star names to implementations.
pub type StarVector = Vec<(&'static str, StarFunction)>;
