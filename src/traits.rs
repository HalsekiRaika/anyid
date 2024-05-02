use dyn_hash::DynHash;
use std::any::Any;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::sync::Arc;

/// Marker Trait, which is a collection of very often used items in using as identifiers.
///
/// ### Require bounds
/// * [`Copy`] Trait is implemented.
///   * Because most of the time, keys are used in K/V arrays.
/// * [`PartialEq`]/[`Eq`] Trait is implemented.
/// * [`Display`]/[`Debug`] is implemented.
///   * Because keys are sometimes displayed in logging.
/// * [`Sync`]/[`Send`] is implemented.
/// * [`Hash`] is implemented.
///   * Because true comparison performed.
///
/// In addition, you will probably never use this Trait directly,
/// as it is automatically implemented in the default implementation if the conditions are met.
pub trait Identifier: Any + DynHash + Debug + Sync + Send + Display + 'static {
    fn as_any(&self) -> &dyn Any;

    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Sync + Send>;

    fn eq(&self, other: &dyn Identifier) -> bool;
}

dyn_hash::hash_trait_object!(Identifier);

impl<T> Identifier for T
where
    T: Any + PartialEq + Eq + Display + Hash + Sync + Send + 'static + Copy + Debug,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Sync + Send> {
        self
    }

    fn eq(&self, other: &dyn Identifier) -> bool {
        let Some(other) = other.as_any().downcast_ref::<T>() else {
            return false;
        };

        self.eq(other)
    }
}
