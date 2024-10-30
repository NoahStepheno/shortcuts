import React from "react";
import type { Extension } from "../../types/extension";
import { Switch } from "../ui/switch";
import { InfoCircledIcon } from "@radix-ui/react-icons";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import Shortcut from "../shortcut";
import { useExtensionStore } from "../../store/extension";

interface ExtensionProps {
  data: Extension[];
}

const Extension: React.FC<ExtensionProps> = () => {
  const { extensions, setExtensions, invokeSetExtensions } = useExtensionStore(
    (state) => state
  );
  return (
    <div className="p-4">
      {extensions.map((extension) => {
        return (
          <div className="divide-y-2 divide-slate-600" key={extension.name}>
            <div
              key={extension.name}
              className="flex flex-row items-center py-4"
            >
              <h2>{extension.name}</h2>
              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger>
                    <InfoCircledIcon
                      className="ml-2"
                      style={{ width: 20, height: 20 }}
                    />
                  </TooltipTrigger>
                  <TooltipContent>
                    <p>{extension.description}</p>
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
              <Switch
                className="ml-auto"
                checked={extension.enabled}
                onCheckedChange={() => {
                  const newExtensions = extensions.map((o) =>
                    o.name === extension.name
                      ? { ...o, enabled: !o.enabled }
                      : o
                  );
                  setExtensions(newExtensions);
                  invokeSetExtensions();
                }}
              />
            </div>
            {extension.enabled && (
              <div className="p-4 pr-0 space-y-4">
                {extension.shortcuts.map((shortcut) => {
                  return (
                    <Shortcut
                      {...shortcut}
                      key={shortcut.name}
                      onChange={(value) => {
                        const newExtensions = extensions.map((o) =>
                          o.name === extension.name
                            ? {
                                ...o,
                                shortcuts: o.shortcuts.map((s) =>
                                  s.name === shortcut.name
                                    ? { ...s, shortcut: value }
                                    : s
                                ),
                              }
                            : o
                        );
                        setExtensions(newExtensions);
                        invokeSetExtensions();
                      }}
                    />
                  );
                })}
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
};

export default Extension;
