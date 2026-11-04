import { defineStore } from "pinia";
import {
  getTool,
  listTools,
  registerTool,
  removeTool,
  toRegistryError,
  updateTool,
} from "../services/registry";
import type {
  RegisterToolRequest,
  ToolProfileView,
  ToolSummaryView,
  UpdateToolRequest,
} from "../types/tool";

interface ToolsState {
  items: ToolSummaryView[];
  warnings: string[];
  profiles: Record<string, ToolProfileView>;
  selectedId: string | null;
  query: string;
  loading: boolean;
  error: string | null;
}

export const useToolStore = defineStore("tools", {
  state: (): ToolsState => ({
    items: [],
    warnings: [],
    profiles: {},
    selectedId: null,
    query: "",
    loading: false,
    error: null,
  }),
  getters: {
    filtered(state): ToolSummaryView[] {
      const q = state.query.trim().toLowerCase();
      if (!q) {
        return state.items;
      }
      return state.items.filter((item) =>
        [item.id, item.name, item.category, item.executableName].some((v) =>
          v.toLowerCase().includes(q),
        ),
      );
    },
    selected(state): ToolProfileView | null {
      return state.selectedId ? (state.profiles[state.selectedId] ?? null) : null;
    },
  },
  actions: {
    async load() {
      this.loading = true;
      this.error = null;
      try {
        const result = await listTools();
        this.items = result.tools;
        this.warnings = result.warnings;
      } catch (e) {
        this.error = toRegistryError(e).message;
      } finally {
        this.loading = false;
      }
    },
    async select(id: string | null) {
      this.selectedId = id;
      if (!id) {
        return;
      }
      try {
        this.profiles[id] = await getTool(id);
        this.error = null;
      } catch (e) {
        this.error = toRegistryError(e).message;
      }
    },
    async register(request: RegisterToolRequest): Promise<ToolProfileView> {
      const view = await registerTool(request);
      this.profiles[view.id] = view;
      await this.load();
      await this.select(view.id);
      return view;
    },
    async update(
      id: string,
      request: UpdateToolRequest,
    ): Promise<ToolProfileView> {
      const view = await updateTool(id, request);
      this.profiles[id] = view;
      await this.load();
      return view;
    },
    async remove(id: string) {
      await removeTool(id);
      delete this.profiles[id];
      if (this.selectedId === id) {
        this.selectedId = null;
      }
      await this.load();
    },
  },
});
