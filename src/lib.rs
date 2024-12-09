mod bulgarian;
mod gost779;
mod macedonian;
mod order_n_995;
mod passport2013;
mod transliterator;

#[cfg(test)]
mod tests;

pub use bulgarian::*;
pub use gost779::*;
pub use macedonian::*;
pub use order_n_995::*;
pub use passport2013::*;
pub use transliterator::*;

pub type CharsMapping = Vec<(&'static str, &'static str)>;
