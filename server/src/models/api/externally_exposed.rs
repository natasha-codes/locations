use serde::Serialize;

use super::OutgoingModel;

/// A wrapper trait around a serializable type that allows us to explicitly
/// restrict what types are exposed in our JSON API.
///
/// Useful because we use `Serialize` for both our API and storage layers, and
/// don't want to leak models from one to the other.
pub trait ExternallyExposed: Serialize {}
