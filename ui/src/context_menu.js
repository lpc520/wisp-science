export function isDevMode() {
  return window.__WISP_DEV__ === true;
}

export function textareaCommand(kind, id) {
  const el = document.getElementById(id);
  if (!el || el.tagName !== "TEXTAREA") return;
  el.focus();
  if (kind === "selectAll") {
    el.select();
    return;
  }
  if (kind === "cut") {
    document.execCommand("cut");
    return;
  }
  if (kind === "copy") {
    document.execCommand("copy");
    return;
  }
  if (kind === "paste") {
    navigator.clipboard.readText().then((text) => {
      const start = el.selectionStart ?? el.value.length;
      const end = el.selectionEnd ?? start;
      const v = el.value;
      el.value = v.slice(0, start) + text + v.slice(end);
      const pos = start + text.length;
      el.selectionStart = pos;
      el.selectionEnd = pos;
      el.dispatchEvent(new Event("input", { bubbles: true }));
    }).catch(() => {});
  }
}
