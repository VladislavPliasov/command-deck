use crate::discovery;
use crate::registry::model::{DiscoveryResult, RegistryError};

#[tauri::command]
pub fn discovery_resolve(name: String) -> Result<DiscoveryResult, RegistryError> {
    Ok(discovery::resolve_name(&name))
}
