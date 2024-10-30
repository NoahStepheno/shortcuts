export const mapKeyToPresent = (str: string): string => {
  const list = str.split("+");
  return list
    .map((key) => {
      switch (key) {
        case "control":
          return "Ctrl";
        case "alt":
          return "Opt";
        case "super":
          return "Cmd";
        case "shift":
          return "Shift";
        default:
          return (mapCodeToPreset(key) as string) || key;
      }
    })
    .join("+");
};

export const mapPresetToKey = (str: string): string => {
  const list = str.split("+");
  return list
    .map((key) => {
      switch (key) {
        case "Ctrl":
          return "control";
        case "Opt":
          return "alt";
        case "Cmd":
          return "super";
        case "Shift":
          return "shift";
        default:
          return mapPresetToCode(key) || key;
      }
    })
    .join("+");
};

export const mapCodeToPreset = (keyCode: string) => {
  const keyMap: { [key: string]: string } = {
    KeyA: "A",
    KeyB: "B",
    KeyC: "C",
    KeyD: "D",
    KeyE: "E",
    KeyF: "F",
    KeyG: "G",
    KeyH: "H",
    KeyI: "I",
    KeyJ: "J",
    KeyK: "K",
    KeyL: "L",
    KeyM: "M",
    KeyN: "N",
    KeyO: "O",
    KeyP: "P",
    KeyQ: "Q",
    KeyR: "R",
    KeyS: "S",
    KeyT: "T",
    KeyU: "U",
    KeyV: "V",
    KeyW: "W",
    KeyX: "X",
    KeyY: "Y",
    KeyZ: "Z",
    Digit0: "0",
    Digit1: "1",
    Digit2: "2",
    Digit3: "3",
    Digit4: "4",
    Digit5: "5",
    Digit6: "6",
    Digit7: "7",
    Digit8: "8",
    Digit9: "9",
  };

  return keyCode in keyMap ? keyMap[keyCode] : keyCode;
};

export const mapPresetToCode = (preset: string) => {
  const keyMap: { [key: string]: string } = {
    A: "KeyA",
    B: "KeyB",
    C: "KeyC",
    D: "KeyD",
    E: "KeyE",
    F: "KeyF",
    G: "KeyG",
    H: "KeyH",
    I: "KeyI",
    J: "KeyJ",
    K: "KeyK",
    L: "KeyL",
    M: "KeyM",
    N: "KeyN",
    O: "KeyO",
    P: "KeyP",
    Q: "KeyQ",
    R: "KeyR",
    S: "KeyS",
    T: "KeyT",
    U: "KeyU",
    V: "KeyV",
    W: "KeyW",
    X: "KeyX",
    Y: "KeyY",
    Z: "KeyZ",
    "0": "Digit0",
    "1": "Digit1",
    "2": "Digit2",
    "3": "Digit3",
    "4": "Digit4",
    "5": "Digit5",
    "6": "Digit6",
    "7": "Digit7",
    "8": "Digit8",
    "9": "Digit9",
  };

  return preset in keyMap ? keyMap[preset] : preset;
};
