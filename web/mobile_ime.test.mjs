import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import vm from "node:vm";

const SENTINEL = "\u200b";
const bridgeSource = readFileSync(
  new URL("../src/internal/web_input.js", import.meta.url),
  "utf8",
).replaceAll("export function ", "function ");

class FakeEventTarget {
  constructor() {
    this.listeners = new Map();
  }

  addEventListener(type, listener, options = {}) {
    const listeners = this.listeners.get(type) || [];
    listeners.push({ listener, once: Boolean(options.once) });
    this.listeners.set(type, listeners);
  }

  dispatchEvent(event) {
    event.target ||= this;
    event.currentTarget = this;
    event.defaultPrevented ||= false;
    event.preventDefault ||= function preventDefault() {
      if (this.cancelable) {
        this.defaultPrevented = true;
      }
    };

    for (const entry of [...(this.listeners.get(event.type) || [])]) {
      entry.listener.call(this, event);

      if (entry.once) {
        const listeners = this.listeners.get(event.type) || [];
        this.listeners.set(
          event.type,
          listeners.filter((candidate) => candidate !== entry),
        );
      }
    }

    return !event.defaultPrevented;
  }
}

class FakeCanvas extends FakeEventTarget {
  constructor() {
    super();
    this.keys = [];
    this.keyboardEvents = [];
    this.focusEvents = [];
  }

  dispatchEvent(event) {
    if (event.type === "keydown" || event.type === "keyup") {
      this.keyboardEvents.push({
        altKey: Boolean(event.altKey),
        code: event.code,
        ctrlKey: Boolean(event.ctrlKey),
        key: event.key,
        location: event.location || 0,
        metaKey: Boolean(event.metaKey),
        repeat: Boolean(event.repeat),
        shiftKey: Boolean(event.shiftKey),
        type: event.type,
      });

      if (event.type === "keydown") {
        this.keys.push(event.key);
      }
    }

    if (event.type === "focus" || event.type === "blur") {
      this.focusEvents.push(event.type);
    }

    return super.dispatchEvent(event);
  }

  focus() {
    if (this.document.activeElement === this) {
      return;
    }

    const previous = this.document.activeElement;
    this.document.activeElement = this;

    if (previous && previous !== this.document.body) {
      previous.dispatchEvent(fakeEvent("blur"));
    }

    this.document.activeElement = this;
    this.dispatchEvent(fakeEvent("focus"));
  }

  getBoundingClientRect() {
    return { left: 0, top: 0 };
  }
}

class FakeInput extends FakeEventTarget {
  constructor(document) {
    super();
    this.document = document;
    this.attributes = new Map();
    this.style = {};
    this.value = "";
  }

  setAttribute(name, value) {
    this.attributes.set(name, value);
  }

  removeAttribute(name) {
    this.attributes.delete(name);
  }

  setSelectionRange(start, end) {
    this.selectionStart = start;
    this.selectionEnd = end;
  }

  focus() {
    if (this.document.activeElement === this) {
      return;
    }

    const previous = this.document.activeElement;
    this.document.activeElement = this;

    if (previous && previous !== this.document.body) {
      previous.dispatchEvent(fakeEvent("blur"));
    }

    this.dispatchEvent(fakeEvent("focus"));
  }

  blur() {
    if (this.document.activeElement !== this) {
      return;
    }

    this.document.activeElement = this.document.body;
    this.dispatchEvent(fakeEvent("blur"));
  }
}

class FakeDocument extends FakeEventTarget {
  constructor(canvas) {
    super();
    this.canvas = canvas;
    this.documentElement = {};
    this.readyState = "complete";
    this.body = new FakeEventTarget();
    this.body.appendChild = (child) => {
      this.input = child;
    };
    this.activeElement = this.body;
  }

  createElement(name) {
    assert.equal(name, "input");
    return new FakeInput(this);
  }

  querySelector(selector) {
    return selector === "canvas" ? this.canvas : null;
  }
}

class FakeKeyboardEvent {
  constructor(type, init) {
    Object.assign(this, init, { type });
  }
}

class FakeFocusEvent {
  constructor(type, init = {}) {
    Object.assign(this, init, { type });
  }
}

const fakeEvent = (type, init = {}) => ({
  bubbles: true,
  cancelable: false,
  composed: true,
  defaultPrevented: false,
  type,
  ...init,
  preventDefault() {
    if (this.cancelable) {
      this.defaultPrevented = true;
    }
  },
});

const delay = () => new Promise((resolve) => setTimeout(resolve, 0));

function createBridge({
  touch = true,
  coarsePointer = touch,
  platform = "MacIntel",
} = {}) {
  const canvas = new FakeCanvas();
  const document = new FakeDocument(canvas);
  canvas.document = document;
  const visualViewport = new FakeEventTarget();
  const window = new FakeEventTarget();
  const animationFrames = new Map();
  let nextAnimationFrame = 1;
  let now = 0;

  class FakeDate extends Date {
    static now() {
      return now;
    }
  }

  Object.assign(window, {
    cancelAnimationFrame(handle) {
      animationFrames.delete(handle);
    },
    clearTimeout,
    matchMedia: (query) => ({
      matches: query === "(pointer: coarse)" && coarsePointer,
    }),
    queueMicrotask,
    requestAnimationFrame(callback) {
      const handle = nextAnimationFrame++;
      animationFrames.set(handle, callback);
      return handle;
    },
    setTimeout,
    visualViewport,
  });

  const context = vm.createContext({
    AbortController,
    Date: FakeDate,
    FocusEvent: FakeFocusEvent,
    KeyboardEvent: FakeKeyboardEvent,
    console,
    document,
    navigator: { maxTouchPoints: touch ? 1 : 0, platform },
    window,
  });
  vm.runInContext(bridgeSource, context, { filename: "web/index.html" });
  const api = vm.runInContext(
    "({ hideMobileKeyboard, registerTextRegion, showMobileKeyboard })",
    context,
  );

  return {
    ...api,
    canvas,
    document,
    visualViewport,
    window,
    advanceTime(milliseconds) {
      now += milliseconds;
    },
    get input() {
      return document.input;
    },
    flushAnimationFrames() {
      const callbacks = [...animationFrames.values()];
      animationFrames.clear();

      for (const callback of callbacks) {
        callback();
      }
    },
  };
}

test("wasm module installs the bridge lazily and only once", () => {
  const bridge = createBridge();

  assert.equal(bridge.input, undefined);

  bridge.showMobileKeyboard();
  const input = bridge.input;
  bridge.showMobileKeyboard();

  assert.equal(bridge.input, input);
  assert.equal(bridge.document.activeElement, input);
});

test("desktop wasm focuses one editable DOM input without touch capabilities", () => {
  const bridge = createBridge({ touch: false });

  bridge.registerTextRegion(0, 0, 100, 100);
  assert.equal(bridge.input, undefined);
  assert.equal(bridge.canvas.listeners.has("pointerdown"), false);

  bridge.showMobileKeyboard();
  const input = bridge.input;
  bridge.showMobileKeyboard();

  assert.equal(bridge.input, input);
  assert.equal(bridge.document.activeElement, input);
  assert.equal(input.value, SENTINEL);
  assert.deepEqual(bridge.canvas.focusEvents, ["focus", "focus"]);
});

test("desktop IME owns candidate keys and commits composition exactly once", async () => {
  const bridge = createBridge({ touch: false });
  bridge.showMobileKeyboard();

  bridge.input.dispatchEvent(fakeEvent("compositionstart"));
  const process = fakeEvent("keydown", {
    cancelable: true,
    code: "KeyN",
    isComposing: true,
    key: "Process",
    keyCode: 229,
  });
  const candidate = fakeEvent("keydown", {
    cancelable: true,
    code: "ArrowDown",
    isComposing: true,
    key: "ArrowDown",
    keyCode: 229,
  });
  bridge.input.dispatchEvent(process);
  bridge.input.dispatchEvent(candidate);
  bridge.input.value = `${SENTINEL}中文`;
  bridge.input.dispatchEvent(
    fakeEvent("input", {
      inputType: "insertCompositionText",
      isComposing: true,
    }),
  );

  assert.deepEqual(bridge.canvas.keys, []);
  assert.equal(process.defaultPrevented, false);
  assert.equal(candidate.defaultPrevented, false);

  bridge.input.dispatchEvent(fakeEvent("compositionend", { data: "中文" }));
  bridge.input.value = `${SENTINEL}中文`;
  bridge.input.dispatchEvent(
    fakeEvent("input", { inputType: "insertFromComposition" }),
  );
  await delay();

  assert.deepEqual(bridge.canvas.keys, ["中", "文"]);
  assert.equal(bridge.input.value, SENTINEL);
});

test("desktop text and command keys use one path and preserve metadata", () => {
  const bridge = createBridge({ touch: false });
  bridge.showMobileKeyboard();

  const plain = fakeEvent("keydown", {
    cancelable: true,
    code: "KeyA",
    key: "a",
  });
  bridge.input.dispatchEvent(plain);
  bridge.input.dispatchEvent(
    fakeEvent("beforeinput", { cancelable: true, inputType: "insertText" }),
  );
  bridge.input.value = `${SENTINEL}a`;
  bridge.input.dispatchEvent(fakeEvent("input", { inputType: "insertText" }));

  assert.equal(plain.defaultPrevented, false);
  assert.deepEqual(bridge.canvas.keys, ["a"]);

  const command = fakeEvent("keydown", {
    cancelable: true,
    code: "KeyA",
    key: "a",
    location: 1,
    metaKey: true,
    repeat: true,
    shiftKey: true,
  });
  bridge.input.dispatchEvent(command);
  bridge.input.dispatchEvent(
    fakeEvent("keyup", {
      code: "KeyA",
      key: "a",
      location: 1,
      metaKey: true,
      shiftKey: true,
    }),
  );

  assert.equal(command.defaultPrevented, true);
  assert.deepEqual(bridge.canvas.keyboardEvents.slice(-2), [
    {
      altKey: false,
      code: "KeyA",
      ctrlKey: false,
      key: "a",
      location: 1,
      metaKey: true,
      repeat: true,
      shiftKey: true,
      type: "keydown",
    },
    {
      altKey: false,
      code: "KeyA",
      ctrlKey: false,
      key: "a",
      location: 1,
      metaKey: true,
      repeat: false,
      shiftKey: true,
      type: "keyup",
    },
  ]);

  const eventCount = bridge.canvas.keyboardEvents.length;
  const controlSpace = fakeEvent("keydown", {
    cancelable: true,
    code: "Space",
    ctrlKey: true,
    key: " ",
  });
  const capsLock = fakeEvent("keydown", {
    cancelable: true,
    code: "CapsLock",
    key: "CapsLock",
  });
  bridge.input.dispatchEvent(controlSpace);
  bridge.input.dispatchEvent(capsLock);

  assert.equal(controlSpace.defaultPrevented, false);
  assert.equal(capsLock.defaultPrevented, false);
  assert.equal(bridge.canvas.keyboardEvents.length, eventCount);
});

test("macOS Chinese input-method shortcuts remain native", () => {
  const bridge = createBridge({ touch: false });
  bridge.showMobileKeyboard();
  const eventCount = bridge.canvas.keyboardEvents.length;

  const controlShiftP = fakeEvent("keydown", {
    cancelable: true,
    code: "KeyP",
    ctrlKey: true,
    key: "P",
    shiftKey: true,
  });
  const shiftBackspace = fakeEvent("keydown", {
    cancelable: true,
    code: "Backspace",
    key: "Backspace",
    shiftKey: true,
  });
  const functionDelete = fakeEvent("keydown", {
    cancelable: true,
    code: "Delete",
    key: "Delete",
    shiftKey: true,
  });
  const convertScript = fakeEvent("keydown", {
    cancelable: true,
    code: "KeyC",
    ctrlKey: true,
    key: "c",
    metaKey: true,
    shiftKey: true,
  });
  bridge.input.dispatchEvent(controlShiftP);
  bridge.input.dispatchEvent(shiftBackspace);
  bridge.input.dispatchEvent(functionDelete);
  bridge.input.dispatchEvent(convertScript);

  assert.equal(controlShiftP.defaultPrevented, false);
  assert.equal(shiftBackspace.defaultPrevented, false);
  assert.equal(functionDelete.defaultPrevented, false);
  assert.equal(convertScript.defaultPrevented, false);
  assert.equal(bridge.canvas.keyboardEvents.length, eventCount);
});

test("standard desktop IME mode keys remain native", () => {
  const mac = createBridge({ touch: false });
  mac.showMobileKeyboard();
  const eisu = fakeEvent("keydown", {
    cancelable: true,
    code: "Lang2",
    key: "Eisu",
  });
  mac.input.dispatchEvent(eisu);

  const windows = createBridge({ platform: "Win32", touch: false });
  windows.showMobileKeyboard();
  const hangul = fakeEvent("keydown", {
    cancelable: true,
    code: "Lang1",
    key: "HangulMode",
  });
  windows.input.dispatchEvent(hangul);

  assert.equal(eisu.defaultPrevented, false);
  assert.equal(mac.canvas.keyboardEvents.length, 0);
  assert.equal(hangul.defaultPrevented, false);
  assert.equal(windows.canvas.keyboardEvents.length, 0);
});

test("macOS-only input-source shortcuts stay available to other desktops", () => {
  const bridge = createBridge({ platform: "Win32", touch: false });
  bridge.showMobileKeyboard();
  const controlShiftP = fakeEvent("keydown", {
    cancelable: true,
    code: "KeyP",
    ctrlKey: true,
    key: "P",
    shiftKey: true,
  });

  bridge.input.dispatchEvent(controlShiftP);

  assert.equal(controlShiftP.defaultPrevented, true);
  assert.equal(bridge.canvas.keyboardEvents.at(-1).key, "P");
});

test("desktop releases a forwarded modifier even after composition starts", () => {
  const bridge = createBridge({ touch: false });
  bridge.showMobileKeyboard();

  bridge.input.dispatchEvent(
    fakeEvent("keydown", {
      cancelable: true,
      code: "ShiftLeft",
      key: "Shift",
      shiftKey: true,
    }),
  );
  bridge.input.dispatchEvent(fakeEvent("compositionstart"));
  bridge.input.dispatchEvent(
    fakeEvent("keyup", {
      code: "ShiftLeft",
      isComposing: true,
      key: "Shift",
      keyCode: 229,
    }),
  );

  assert.deepEqual(bridge.canvas.keyboardEvents.slice(-2), [
    {
      altKey: false,
      code: "ShiftLeft",
      ctrlKey: false,
      key: "Shift",
      location: 0,
      metaKey: false,
      repeat: false,
      shiftKey: true,
      type: "keydown",
    },
    {
      altKey: false,
      code: "ShiftLeft",
      ctrlKey: false,
      key: "Shift",
      location: 0,
      metaKey: false,
      repeat: false,
      shiftKey: false,
      type: "keyup",
    },
  ]);
});

test("desktop text commits preserve held modifiers for following commands", () => {
  const bridge = createBridge({ touch: false });
  bridge.showMobileKeyboard();

  bridge.input.dispatchEvent(
    fakeEvent("keydown", {
      cancelable: true,
      code: "ShiftLeft",
      key: "Shift",
      shiftKey: true,
    }),
  );
  bridge.input.dispatchEvent(
    fakeEvent("keydown", {
      cancelable: true,
      code: "KeyA",
      key: "A",
      shiftKey: true,
    }),
  );
  bridge.input.value = `${SENTINEL}A`;
  bridge.input.dispatchEvent(fakeEvent("input", { inputType: "insertText" }));
  bridge.input.dispatchEvent(
    fakeEvent("keydown", {
      cancelable: true,
      code: "ArrowLeft",
      key: "ArrowLeft",
      shiftKey: true,
    }),
  );

  const committedText = bridge.canvas.keyboardEvents.find(
    (event) => event.type === "keydown" && event.key === "A",
  );
  const selection = bridge.canvas.keyboardEvents.find(
    (event) => event.type === "keydown" && event.key === "ArrowLeft",
  );

  assert.equal(committedText.shiftKey, true);
  assert.equal(selection.shiftKey, true);
});

test("desktop bridge reclaims focus during activation unless hidden", () => {
  const bridge = createBridge({ touch: false });
  bridge.showMobileKeyboard();

  bridge.canvas.focus();
  assert.equal(bridge.document.activeElement, bridge.canvas);
  bridge.flushAnimationFrames();
  assert.equal(bridge.document.activeElement, bridge.input);

  bridge.canvas.focus();
  bridge.hideMobileKeyboard();
  bridge.flushAnimationFrames();
  assert.equal(bridge.document.activeElement, bridge.canvas);
  assert.deepEqual(bridge.canvas.focusEvents.slice(-2), ["blur", "focus"]);
});

test("desktop field request reactivates the bridge after a stale blur", () => {
  const bridge = createBridge({ touch: false });
  bridge.showMobileKeyboard();
  bridge.advanceTime(251);

  bridge.input.blur();
  bridge.flushAnimationFrames();

  assert.equal(bridge.document.activeElement, bridge.document.body);

  bridge.showMobileKeyboard();

  assert.equal(bridge.document.activeElement, bridge.input);
  assert.equal(bridge.input.attributes.has("inert"), false);
});

test("desktop transient blur clears a held system modifier before refocus", () => {
  const bridge = createBridge({ touch: false });
  bridge.showMobileKeyboard();

  bridge.input.dispatchEvent(
    fakeEvent("keydown", {
      cancelable: true,
      code: "MetaLeft",
      key: "Meta",
      metaKey: true,
    }),
  );
  bridge.input.blur();

  assert.equal(bridge.canvas.focusEvents.at(-1), "blur");

  bridge.flushAnimationFrames();

  assert.equal(bridge.document.activeElement, bridge.input);
  assert.deepEqual(bridge.canvas.focusEvents.slice(-2), ["blur", "focus"]);

  bridge.input.value = `${SENTINEL}a`;
  bridge.input.dispatchEvent(fakeEvent("input", { inputType: "insertText" }));

  const committedText = bridge.canvas.keyboardEvents.find(
    (event) => event.type === "keydown" && event.key === "a",
  );
  assert.equal(committedText.metaKey, false);
});

test("composition-owned keys stay in the IME and commit exactly once", async () => {
  const bridge = createBridge();
  bridge.showMobileKeyboard();

  bridge.input.dispatchEvent(fakeEvent("compositionstart"));
  bridge.input.dispatchEvent(
    fakeEvent("keydown", {
      cancelable: true,
      isComposing: true,
      key: "Backspace",
      keyCode: 229,
    }),
  );
  bridge.input.dispatchEvent(
    fakeEvent("keydown", {
      cancelable: true,
      isComposing: true,
      key: "Enter",
      keyCode: 229,
    }),
  );
  bridge.input.value = `${SENTINEL}中文`;
  bridge.input.dispatchEvent(
    fakeEvent("input", {
      inputType: "insertCompositionText",
      isComposing: true,
    }),
  );

  assert.deepEqual(bridge.canvas.keys, []);

  bridge.input.dispatchEvent(fakeEvent("compositionend", { data: "中文" }));
  bridge.input.value = `${SENTINEL}中文`;
  bridge.input.dispatchEvent(
    fakeEvent("input", { inputType: "insertFromComposition" }),
  );
  await delay();

  assert.deepEqual(bridge.canvas.keys, ["中", "文"]);
  assert.equal(bridge.input.value, SENTINEL);
});

test("canceling composition does not delete application text", () => {
  const bridge = createBridge();
  bridge.showMobileKeyboard();
  bridge.input.dispatchEvent(fakeEvent("compositionstart"));
  bridge.input.value = "";
  bridge.input.dispatchEvent(fakeEvent("compositionend", { data: "" }));

  assert.deepEqual(bridge.canvas.keys, []);
});

test("composition commit is flushed before a following blur can reset it", () => {
  const bridge = createBridge();
  bridge.showMobileKeyboard();
  bridge.input.dispatchEvent(fakeEvent("compositionstart"));
  bridge.input.value = `${SENTINEL}語`;

  bridge.input.dispatchEvent(fakeEvent("compositionend", { data: "語" }));
  bridge.hideMobileKeyboard();

  assert.deepEqual(bridge.canvas.keys, ["語"]);
});

test("beforeinput forwards soft-keyboard delete and enter actions once", async () => {
  const bridge = createBridge();
  bridge.showMobileKeyboard();

  bridge.input.dispatchEvent(
    fakeEvent("keydown", { cancelable: true, key: "Backspace" }),
  );
  const backward = fakeEvent("beforeinput", {
    cancelable: true,
    inputType: "deleteContentBackward",
  });
  bridge.input.dispatchEvent(backward);

  const forward = fakeEvent("beforeinput", {
    cancelable: true,
    inputType: "deleteContentForward",
  });
  bridge.input.dispatchEvent(forward);

  const enter = fakeEvent("beforeinput", {
    cancelable: true,
    inputType: "insertParagraph",
  });
  bridge.input.dispatchEvent(enter);
  await delay();

  assert(backward.defaultPrevented);
  assert(forward.defaultPrevented);
  assert(enter.defaultPrevented);
  assert.deepEqual(bridge.canvas.keys, ["Backspace", "Delete", "Enter"]);
});

test("non-cancelable mobile deletion suppresses the following input duplicate", async () => {
  const bridge = createBridge();
  bridge.showMobileKeyboard();

  bridge.input.dispatchEvent(
    fakeEvent("beforeinput", {
      inputType: "deleteContentBackward",
    }),
  );
  bridge.input.value = "";
  bridge.input.dispatchEvent(
    fakeEvent("input", { inputType: "deleteContentBackward" }),
  );

  bridge.input.dispatchEvent(
    fakeEvent("beforeinput", {
      inputType: "deleteContentForward",
    }),
  );
  await delay();

  assert.deepEqual(bridge.canvas.keys, ["Backspace", "Delete"]);
});

test("refocus frame does not reset an active composition", () => {
  const bridge = createBridge();
  bridge.showMobileKeyboard();
  bridge.input.dispatchEvent(fakeEvent("compositionstart"));
  bridge.input.value = `${SENTINEL}pin`;

  bridge.flushAnimationFrames();

  assert.equal(bridge.input.value, `${SENTINEL}pin`);
});

test("visual keyboard resize preserves registered text regions", () => {
  const bridge = createBridge();
  bridge.registerTextRegion(0, 0, 100, 100);
  bridge.hideMobileKeyboard();
  bridge.visualViewport.dispatchEvent(fakeEvent("resize"));

  bridge.canvas.dispatchEvent(
    fakeEvent("pointerdown", {
      clientX: 10,
      clientY: 10,
      pointerId: 1,
      pointerType: "touch",
    }),
  );
  bridge.canvas.dispatchEvent(
    fakeEvent("pointerup", {
      clientX: 10,
      clientY: 10,
      pointerId: 1,
      pointerType: "touch",
    }),
  );

  assert.equal(bridge.document.activeElement, bridge.input);
});
