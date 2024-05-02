mod error;
mod traits;

pub use self::{error::*, traits::*};

use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// An identifier that can be handled with some disregard for type information
/// by storing the type that meets the requirements as an identifier.
///
/// See [Identifier]
#[derive(Debug)]
pub struct AnyId(Arc<dyn Identifier>);

impl AnyId {
    pub fn new(id: impl Identifier) -> Self {
        Self(Arc::new(id))
    }

    pub fn downcast_ref<T: Identifier>(&self) -> Result<&T, Error> {
        self.0.as_any().downcast_ref::<T>().ok_or(Error::Downcast)
    }

    pub fn downcast<T: Identifier + Copy>(self) -> Result<T, Error> {
        let id = self
            .0
            .as_any_arc()
            .downcast::<T>()
            .map_err(|_| Error::Downcast)?;
        Ok(*id)
    }
}

impl Clone for AnyId {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl Display for AnyId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<Self> for AnyId {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(other.0.as_ref())
    }
}

impl<T: Identifier> PartialEq<T> for AnyId {
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

impl Eq for AnyId {}

impl Hash for AnyId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T: Identifier> From<T> for AnyId {
    fn from(value: T) -> Self {
        Self(Arc::new(value))
    }
}
