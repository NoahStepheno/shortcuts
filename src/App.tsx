import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

import "./App.css";
import { useExtensionStore } from "./store/extension";
import { default as ExtensionComponent } from "./components/extension";
import { ThemeProvider } from "@/components/theme-provider";
import type { Extension } from "./types/extension";

function App() {
  const { extensions, setExtensions } = useExtensionStore((state) => state);

  useEffect(() => {
    invoke("init");
    invoke("get_extensions").then((messages) => {
      const json = JSON.parse(messages as string) as Extension[];
      setExtensions(json);
    });
  }, []);

  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <ExtensionComponent data={extensions} />
    </ThemeProvider>
  );
}

export default App;
