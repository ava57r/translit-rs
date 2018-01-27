mod gost779;
mod passport2013;
mod transliterator;

#[cfg(test)]
mod tests;

pub use gost779::*;
pub use passport2013::*;
pub use transliterator::*;

pub type CharsMapping = Vec<(&'static str, &'static str)>;
