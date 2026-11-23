// One GPUI window per preview document. Native HTML controls provide browser
// focus, editing, and accessibility actions; Rust owns every block value.
let form;

const login = `
  <h1>Sign in</h1>
  <p>Try the form with a sample email and password.</p>
  <form id="login" novalidate autocomplete="off">
    <label for="email">Email address</label>
    <input id="email" type="email" required aria-describedby="email-error">
    <small id="email-error"></small>
    <label for="password">Password</label>
    <div class="password-row">
      <input id="password" type="password" required aria-describedby="password-error">
      <button id="reveal" type="button">Show password</button>
    </div>
    <small id="password-error"></small>
    <button type="submit">Sign in</button>
    <output id="status"></output>
  </form>`;

const settings = `
  <h1>Settings</h1>
  <p id="workspace"></p>
  <nav aria-label="Settings sections">
    <button type="button" data-section="general">General</button>
    <button type="button" data-section="appearance">Appearance</button>
    <button type="button" data-section="notifications">Notifications</button>
  </nav>
  <form id="settings" novalidate autocomplete="off">
    <section id="general" aria-label="General">
      <label for="name">Workspace name</label>
      <input id="name" required aria-describedby="name-error">
      <small id="name-error"></small>
      <label for="description">Description</label>
      <input id="description">
    </section>
    <section id="appearance" aria-label="Appearance" hidden>
      <label><input id="compact" type="checkbox" role="switch"> Compact spacing</label>
    </section>
    <section id="notifications" aria-label="Notifications" hidden>
      <label><input id="mentions" type="checkbox" role="switch"> Mentions</label>
      <label><input id="digest" type="checkbox" role="switch"> Weekly digest</label>
    </section>
    <output id="status"></output>
    <div><button id="discard" type="button" aria-label="Discard changes">Discard</button>
      <button id="apply" type="submit">Apply changes</button></div>
  </form>`;

const styles = `
  #block-controls { color: #18181b; font: 14px/1.45 system-ui, sans-serif; }
  #block-controls [hidden] { display: none; }
  #block-controls button, #block-controls input { font: inherit; color: inherit; }
  #block-controls button { background: #fff; border: 1px solid #a1a1aa; border-radius: 6px; padding: .35rem .65rem; }
  #block-controls button:not(:disabled) { cursor: pointer; }
  #block-controls button:disabled { opacity: .55; }
  #block-controls :focus-visible { outline: 3px solid #2563eb; outline-offset: 2px; }
  #form-mode { position: fixed; top: .5rem; right: .5rem; z-index: 2; }
  #block-form { position: fixed; inset: 0; overflow: auto; z-index: 1; padding: 3rem 1rem 1rem; background: #fafafa; }
  #block-controls:not(.editing) #block-form { inset: 0 auto auto 0; width: 1px; height: 1px; padding: 0; clip-path: inset(50%); overflow: hidden; }
  #block-form > div { max-width: 32rem; margin: auto; }
  #block-form h1 { margin: 0; font-size: 1.5rem; }
  #block-form p { margin: .35rem 0 1rem; }
  #block-form form, #block-form section { display: grid; gap: .6rem; }
  #block-form nav, #block-form .password-row { display: flex; flex-wrap: wrap; gap: .5rem; margin-bottom: .5rem; }
  #block-form nav [aria-current=page] { border-color: #27272a; font-weight: 600; }
  #block-form input:not([type=checkbox]) { min-width: 0; padding: .45rem; border: 1px solid #a1a1aa; border-radius: 5px; }
  #block-form .password-row input { flex: 1; }
  #block-form small { color: #991b1b; }
  #block-form small:empty { display: none; }
  #block-form button[type=submit] { background: #27272a; color: white; }
  body.dark #block-controls { color-scheme: dark; color: #fafafa; }
  body.dark #block-form { background: #18181b; }
  body.dark #block-controls button { background: #27272a; }
  body.dark #block-form nav [aria-current=page] { border-color: #fafafa; }
  body.dark #block-form small { color: #fca5a5; }
`;

const field = (id) => form.querySelector("#" + id);

function setText(id, text) {
  const el = field(id);
  if (el.textContent !== text) el.textContent = text;
}

function setValue(id, value) {
  const input = field(id);
  // Reassigning while typing moves the native selection and disrupts IME.
  if (input.value !== value) input.value = value;
}

function setError(id, error) {
  field(id).setAttribute("aria-invalid", String(Boolean(error)));
  setText(id + "-error", error);
}

export function mountForm(kind, action) {
  const host = document.createElement("div");
  host.id = "block-controls";
  host.innerHTML = `<style>${styles}</style>
    <button type="button" id="form-mode" aria-controls="block-form">Form controls</button>
    <div id="block-form"><div>${kind === "login" ? login : settings}</div></div>`;
  document.body.append(host);
  form = host.querySelector("#block-form");
  const mode = host.querySelector("#form-mode");
  const canvas = document.querySelector("canvas");
  const ime = document.querySelector("body > textarea");
  // GPUI uses this hidden textarea for native canvas input. Keep it out of
  // sequential navigation; a canvas pointer gesture re-enables it for IME.
  ime.tabIndex = -1;
  ime.setAttribute("aria-label", "Canvas keyboard input");
  ime.inert = true;
  canvas.setAttribute("role", "img");
  canvas.setAttribute("aria-label", kind === "login" ? "Sign-in preview" : "Settings preview");
  canvas.addEventListener(
    "pointerdown",
    () => {
      ime.inert = false;
    },
    true,
  );
  const setEditing = (open) => {
    host.classList.toggle("editing", open);
    mode.textContent = open ? "Show canvas" : "Form controls";
    ime.inert = true;
  };
  mode.onclick = () => {
    const open = !host.classList.contains("editing");
    setEditing(open);
    if (open) form.querySelector("nav button, input").focus();
  };
  form.addEventListener("focusin", () => setEditing(true));
  host.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && !event.isComposing && host.classList.contains("editing")) {
      event.preventDefault();
      setEditing(false);
      mode.focus();
    }
  });
  ime.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && !event.defaultPrevented && !event.isComposing) {
      mode.focus();
      ime.inert = true;
    }
  });
  form.addEventListener("input", ({ target }) => {
    if (target instanceof HTMLInputElement) {
      action(target.id, target.type === "checkbox" ? String(target.checked) : target.value);
    }
  });
  form.addEventListener("click", ({ target }) => {
    if (!(target instanceof HTMLButtonElement) || target.type !== "button") return;
    if (target.dataset.section) action("section", target.dataset.section);
    else action(target.id, "");
  });
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    action(kind === "login" ? "submit" : "apply", "");
    form.querySelector('input[aria-invalid="true"]')?.focus();
  });
}

export function updateLogin(email, password, masked, emailError, passwordError, validated) {
  setValue("email", email);
  const type = masked ? "password" : "text";
  if (field("password").type !== type) field("password").type = type;
  setValue("password", password);
  setText("reveal", masked ? "Show password" : "Hide password");
  setError("email", emailError);
  setError("password", passwordError);
  setText("status", validated ? "Sample credentials validated locally." : "");
}

export function updateSettings(
  section,
  name,
  description,
  appliedName,
  compact,
  mentions,
  digest,
  dirty,
  valid,
  status,
) {
  for (const button of form.querySelectorAll("nav button")) {
    const selected = button.dataset.section === section;
    if (selected) button.setAttribute("aria-current", "page");
    else button.removeAttribute("aria-current");
    field(button.dataset.section).hidden = !selected;
  }
  setText("workspace", appliedName);
  setValue("name", name);
  setError("name", valid ? "" : "Enter a workspace name.");
  setValue("description", description);
  field("compact").checked = compact;
  field("mentions").checked = mentions;
  field("digest").checked = digest;
  field("discard").disabled = !dirty;
  field("apply").disabled = !dirty || !valid;
  setText("status", status);
}
