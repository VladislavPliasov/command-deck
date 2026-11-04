import { invoke } from "@tauri-apps/api/core";
import type {
  DiscoveryResult,
  RegisterToolRequest,
  RegistryErrorShape,
  ToolListResult,
  ToolProfileView,
  UpdateToolRequest,
} from "../types/tool";

export function isRegistryError(value: unknown): value is RegistryErrorShape {
  return (
    typeof value === "object" &&
    value !== null &&
    typeof (value as RegistryErrorShape).code === "string" &&
    typeof (value as RegistryErrorShape).message === "string"
  );
}

export function toRegistryError(value: unknown): RegistryErrorShape {
  if (isRegistryError(value)) {
    return value;
  }
  return { code: "unknown", message: String(value) };
}

export async function listTools(): Promise<ToolListResult> {
  return invoke<ToolListResult>("registry_list");
}

export async function getTool(id: string): Promise<ToolProfileView> {
  return invoke<ToolProfileView>("registry_get", { id });
}

export async function registerTool(
  request: RegisterToolRequest,
): Promise<ToolProfileView> {
  return invoke<ToolProfileView>("registry_register", { request });
}

export async function updateTool(
  id: string,
  request: UpdateToolRequest,
): Promise<ToolProfileView> {
  return invoke<ToolProfileView>("registry_update", { id, request });
}

export async function removeTool(id: string): Promise<void> {
  await invoke<void>("registry_remove", { id });
}

export async function resolveDiscovery(name: string): Promise<DiscoveryResult> {
  return invoke<DiscoveryResult>("discovery_resolve", { name });
}
