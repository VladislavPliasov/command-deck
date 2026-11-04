use crate::registry::model::*;
use crate::registry::RegistryService;
use tauri::State;

#[tauri::command]
pub fn registry_list(
    service: State<'_, RegistryService>,
) -> Result<ToolListResult, RegistryError> {
    Ok(service.list())
}

#[tauri::command]
pub fn registry_get(
    service: State<'_, RegistryService>,
    id: String,
) -> Result<ToolProfileView, RegistryError> {
    service.get(&id)
}

#[tauri::command]
pub fn registry_register(
    service: State<'_, RegistryService>,
    request: RegisterToolRequest,
) -> Result<ToolProfileView, RegistryError> {
    service.register(request)
}

#[tauri::command]
pub fn registry_update(
    service: State<'_, RegistryService>,
    id: String,
    request: UpdateToolRequest,
) -> Result<ToolProfileView, RegistryError> {
    service.update(&id, request)
}

#[tauri::command]
pub fn registry_remove(
    service: State<'_, RegistryService>,
    id: String,
) -> Result<(), RegistryError> {
    service.remove(&id)
}
