use std::collections::HashMap;

use vector_config::configurable_component;

use crate::{config::SecretBackend, signal};

/// Configuration for the `test` secrets backend.
#[configurable_component(secrets("test"))]
#[derive(Clone, Debug, Default)]
pub struct TestBackend {
    /// Fixed value to replace all secrets with.
    pub replacement: String,
}

impl_generate_config_from_default!(TestBackend);

impl SecretBackend for TestBackend {
    fn retrieve(
        &mut self,
        secret_keys: Vec<String>,
        _: &mut signal::SignalRx,
    ) -> crate::Result<HashMap<String, String>> {
        Ok(secret_keys
            .into_iter()
            .map(|k| (k, self.replacement.clone()))
            .collect())
    }
}
