import React, { useEffect } from "react";
import type { OverrideStyle } from "../../types/override-style";
import { cn } from "../../lib/utils";
import { Input } from "../ui/input";
import { mapCodeToPreset, mapKeyToPresent, mapPresetToKey } from "./helper";

interface ShortcutInputProps extends OverrideStyle {
  value: string;
  onChange: (value: string) => void;
}

const ShortcutInput: React.FC<ShortcutInputProps> = ({
  className,
  value,
  onChange,
}) => {
  const [key, setKey] = React.useState<string>();

  console.log(value);
  useEffect(() => {
    setKey(mapKeyToPresent(value));
  }, [value]);

  return (
    <div className={cn("flex flex-row space-x-1 w-28", className)}>
      <Input
        className="text-right"
        onKeyDown={(event) => {
          const keys = new Set();
          if (event.ctrlKey) {
            keys.add("Ctrl");
          }
          if (event.shiftKey) {
            keys.add("Shift");
          }
          if (event.altKey) {
            keys.add("Opt");
          }
          if (event.metaKey) {
            keys.add("Cmd");
          }

          if (
            event.key !== "Control" &&
            event.key !== "Shift" &&
            event.key !== "Alt" &&
            event.key !== "Meta"
          ) {
            keys.add(mapCodeToPreset(event.code));

            onChange(mapPresetToKey(Array.from(keys).join("+")));
          }

          setKey(Array.from(keys).join("+"));
        }}
        placeholder="Shortcut"
        value={key}
        onChange={() => {}}
      />
    </div>
  );
};

export default ShortcutInput;
