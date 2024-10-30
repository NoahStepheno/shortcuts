export interface Shortcut {
  name: string;
  description: string;
  shortcut: string;
}

export interface Extension {
  name: string;
  description: string;
  shortcuts: Shortcut[];

  enabled: boolean;
}
