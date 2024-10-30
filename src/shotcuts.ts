import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export interface ShortcutModifier {
  ALT: 0x01;
  CONTROL: 0x8;
  FN: 0x10;
  SHIFT: 0x200;
}

export interface ShortcutPayload {
  mods: ShortcutModifier;
  code: string;
  id: number;
}

export class Shortcut {
  data: ShortcutPayload[];
  unListenEventHandler: UnlistenFn | null;
  isConfigReady: boolean;

  constructor() {
    this.data = [];
    this.unListenEventHandler = null;
    this.isConfigReady = false;
  }

  async listenEvent() {
    const unListen = await listen("shortcut-event", console.log);
    this.unListenEventHandler = unListen;
    return unListen;
  }

  async stopListenEvent() {
    if (this.unListenEventHandler) {
      this.unListenEventHandler();
      this.unListenEventHandler = null;
    }
  }

  async listenConfig() {
    const unListen = await listen("shortcut-config", (...args) => {
      console.log(args);
      this.isConfigReady = true;
      // todo: parse config
    });
    this.unListenEventHandler = unListen;
    return unListen;
  }

  async stopListenConfig() {
    if (this.unListenEventHandler) {
      this.unListenEventHandler();
      this.unListenEventHandler = null;
    }
  }

  async init() {
    this.listenConfig();
    this.listenEvent();
  }
}
