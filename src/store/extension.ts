import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import type { Extension } from "../types/extension";

interface ExtensionStore {
  extensions: Extension[];
  setExtensions: (extensions: Extension[]) => void;
  invokeSetExtensions: () => void;
}

export const useExtensionStore = create<ExtensionStore>((set, get) => ({
  extensions: [],
  setExtensions: (extensions: Extension[]) => set({ extensions }),
  invokeSetExtensions: () => {
    invoke("set_extensions", {
      invokeMessage: JSON.stringify(get().extensions),
    });
  },
}));
