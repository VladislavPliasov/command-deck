fn main() {
    tauri_build::try_build(
        tauri_build::Attributes::new().app_manifest(tauri_build::AppManifest::new().commands(&[
            "app_info",
            "registry_list",
            "registry_get",
            "registry_register",
            "registry_update",
            "registry_remove",
            "discovery_resolve",
        ])),
    )
    .unwrap();
}
