use serde::{Deserialize, Serialize};

/// A wrapper trait around a serializable type that allows us to explicitly
/// restrict what types are returned by our JSON API.
///
/// Useful because we use `Serialize` for both our API and storage layers, and
/// don't want to leak models from one to the other.
pub trait ExternallyExposedOutgoing: Serialize {}

/// A wrapper trait around a serializable type that allows us to explicitly
/// restrict what types are accepted by our JSON API.
///
/// Useful because we use `Deserialize` for both our API and storage layers, and
/// don't want to leak models from one to the other.
pub trait ExternallyExposedIncoming<'de>: Deserialize<'de> {}
