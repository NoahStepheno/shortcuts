import { register } from "@tauri-apps/plugin-global-shortcut";

export const initRegister = async () => {
  console.log("Registering shortcuts");
  // register a single hotkey
  let result = await register("CommandOrControl+Shift+N", (event) => {
    if (event.state === "Pressed") {
      console.log("Shortcut triggered");
    }
  });

  console.log(result);

  // // or register multiple hotkeys at once
  // result = await register(["CommandOrControl+Shift+C", "Alt+A"], (event) => {
  //   console.log(`Shortcut ${event.shortcut} triggered`);
  // });

  // console.log(result);
};
