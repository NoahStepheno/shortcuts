import React from "react";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "../ui/tooltip";
import { InfoCircledIcon } from "@radix-ui/react-icons";
import ShortcutInput from "../shortcut-input";
import type { Shortcut as ShortcutProps } from "../../types/extension";

const Shortcut: React.FC<
  ShortcutProps & { onChange: (value: string) => void }
> = ({ name, description, shortcut, onChange }) => {
  return (
    <div className="flex flex-row items-center">
      <h2>{name}</h2>
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger>
            <InfoCircledIcon
              className="ml-2"
              style={{ width: 20, height: 20 }}
            />
          </TooltipTrigger>
          <TooltipContent>
            <p>{description}</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
      <ShortcutInput className="ml-auto" value={shortcut} onChange={onChange} />
    </div>
  );
};

export default Shortcut;
