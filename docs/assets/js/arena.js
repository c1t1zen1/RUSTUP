// Robbie Arena page: edit a mission, watch it run, compare with your Rust.
(function () {
  "use strict";
  const A = window.ARENA;
  const $ = s => document.querySelector(s);
  const COLORS = ["var(--rust)", "var(--led)", "var(--good)", "var(--warn)", "var(--bronze)", "var(--bad)"];
  const KEY = "rustup-arena-v1";

  const PRESETS = {
    studio: { tab: "json", label: "Studio crew (missions/studio.json)", json: `{
  "map": "##########\\n#..P...#.#\\n#.##.#.P.#\\n#P...#...#\\n#..#...#P#\\n##########",
  "bots": [
    { "name": "Robbie", "start": [1, 1], "facing": "E", "battery": 30,
      "script": "# along the top, grab the first part\\nforward 2\\npick\\nforward 3\\nright\\nforward\\nleft\\nforward\\npick" },
    { "name": "Zippy", "start": [1, 3], "facing": "N", "battery": 12,
      "script": "pick\\nright\\nforward 3\\nleft\\nforward 5\\nright\\nforward 9" },
    { "name": "Dusty", "start": [6, 3], "facing": "E", "battery": 3,
      "script": "forward 2\\nright\\nforward 1\\npick\\nleft\\nforward 2" }
  ]
}` },
    tour: { tab: "plain", label: "Warehouse tour (map + script)",
      map: "##########\n#........#\n#.#P##.#.#\n#..P...P.#\n##########",
      script: "# Robbie starts top left, facing East\nforward 2\nright\nforward 2\npick\nleft\nforward 4\npick\nright\nforward" },
    duel: { tab: "json", label: "Two bots, one part", json: `{
  "map": "#######\\n#..P..#\\n#######",
  "bots": [
    { "name": "Robbie", "start": [1, 1], "facing": "E", "battery": 10, "script": "forward 2\\npick" },
    { "name": "Zippy", "start": [5, 1], "facing": "W", "battery": 10, "script": "forward 2\\npick" }
  ]
}` },
    broken: { tab: "json", label: "Broken on purpose", json: `{
  "map": "#####\\n#..P#\\n#####",
  "bots": [
    { "name": "Robbie", "start": [1, 1], "facing": "E", "battery": 10, "script": "forward 2\\njump\\npick" }
  ]
}` }
  };

  let tab = "json", state = null, it = null, timer = null, done = false, last = null, log = [];

  const store = {
    get() { try { return JSON.parse(localStorage.getItem(KEY) || "null"); } catch (e) { return null; } },
    set(v) { try { localStorage.setItem(KEY, JSON.stringify(v)); } catch (e) { /* a convenience only */ } }
  };

  function missionText() {
    if (tab === "json") return $("#json").value;
    return A.missionFromMapAndScript($("#map").value, $("#script").value);
  }
  function save() { store.set({ tab, json: $("#json").value, map: $("#map").value, script: $("#script").value, mode: $("#mode").value }); }

  function setTab(t) {
    tab = t;
    document.querySelectorAll("[data-tab]").forEach(b => b.setAttribute("aria-selected", String(b.dataset.tab === t)));
    $("#pane-json").hidden = t !== "json";
    $("#pane-plain").hidden = t !== "plain";
  }

  function loadPreset(name) {
    const p = PRESETS[name];
    if (!p) return;
    setTab(p.tab);
    if (p.json) $("#json").value = p.json;
    if (p.map) { $("#map").value = p.map; $("#script").value = p.script; }
    reset();
  }

  function showError(msg) {
    $("#err").hidden = !msg;
    $("#err").textContent = msg ? "error: " + msg : "";
  }

  function stop() { if (timer) { clearTimeout(timer); timer = null; } $("#play").textContent = "Play"; }

  function reset() {
    stop(); save();
    log = []; last = null; done = false;
    try {
      state = A.prepare(missionText());
      it = A.frames(state, $("#mode").value);
      showError("");
    } catch (e) {
      state = null; it = null;
      showError(e.message);
    }
    draw();
  }

  function describe(f) {
    const c = state.crew[f.bot].cmds[f.cmd];
    const cmd = c.op === "forward" ? `forward ${c.n}` : c.op;
    const what = { move: "moves", bump: "bumps!", flat: "goes flat", turn: "turns", pick: "picks up a part", nothing: "finds nothing to pick", wait: "waits", ignored: "is flat, ignores it" }[f.what];
    return { who: state.crew[f.bot].bot.name, line: c.line, cmd, what };
  }

  function step() {
    if (!it || done) return false;
    const r = it.next();
    if (r.done) { done = true; stop(); draw(); return false; }
    last = r.value;
    const d = describe(last);
    log.unshift(d);
    if (log.length > 8) log.pop();
    draw();
    return true;
  }

  function play() {
    if (timer) { stop(); return; }
    if (!it || done) reset();
    if (!it) return;
    $("#play").textContent = "Pause";
    const tick = () => {
      if (!step()) { stop(); return; }
      timer = setTimeout(tick, Number($("#speed").value));
    };
    tick();
  }

  function runToEnd() {
    stop();
    if (!it || done) reset();
    if (!it) return;
    let guard = 0;
    while (step() && guard++ < 200000) { /* run */ }
  }

  function draw() {
    const board = $("#board");
    board.textContent = "";
    if (!state) { board.hidden = true; $("#bots").textContent = ""; $("#now").textContent = ""; $("#report").textContent = ""; $("#log").textContent = ""; return; }
    board.hidden = false;
    const w = A.width(state.world), h = A.height(state.world);
    board.style.setProperty("--cols", w);
    board.style.setProperty("--rows", h);
    board.setAttribute("aria-label", `Map, ${w} by ${h} tiles`);
    const frag = document.createDocumentFragment();
    for (let y = 0; y < h; y++) for (let x = 0; x < w; x++) {
      const t = A.tile(state.world, x, y);
      const d = document.createElement("div");
      d.className = "tile " + t;
      if (t === "part") d.innerHTML = '<svg viewBox="0 0 24 24" aria-hidden="true"><path d="M12 2l2 3 3.5-.6.6 3.5L21 10l-1.5 2 1.5 2-2.9 2.1-.6 3.5-3.5-.6-2 3-2-3-3.5.6-.6-3.5L3 14l1.5-2L3 10l2.9-2.1.6-3.5 3.5.6z" fill="currentColor"/><circle cx="12" cy="12" r="3.2" fill="var(--bg-2)"/></svg>';
      frag.appendChild(d);
    }
    board.appendChild(frag);
    const here = {};
    state.crew.forEach(({ bot }, i) => {
      const k = bot.x + "," + bot.y; const n = here[k] = (here[k] || 0) + 1;
      const b = document.createElement("div");
      b.className = "bot" + (bot.flat ? " flat" : "") + (last && last.bot === i && last.what === "bump" ? " bump" : "") + (last && last.bot === i ? " active" : "");
      b.style.setProperty("--x", bot.x); b.style.setProperty("--y", bot.y);
      b.style.setProperty("--c", COLORS[i % COLORS.length]);
      b.style.setProperty("--nudge", (n - 1) * 6 + "px");
      b.dataset.facing = bot.facing;
      b.title = `${bot.name} at (${bot.x}, ${bot.y}) facing ${bot.facing}`;
      b.innerHTML = `<i aria-hidden="true"></i><span>${escapeHtml(Array.from(bot.name)[0] || "?")}</span>`;
      board.appendChild(b);
    });

    $("#bots").innerHTML = state.crew.map(({ bot }, i) => {
      const pct = bot.capacity ? Math.round(bot.battery / bot.capacity * 100) : 0;
      return `<li style="--c:${COLORS[i % COLORS.length]}"><b>${escapeHtml(bot.name)}</b>
        <span class="batt" role="img" aria-label="battery ${bot.battery} of ${bot.capacity}"><i style="width:${pct}%"></i></span>
        <span class="stats">(${bot.x}, ${bot.y}) ${bot.facing} · battery ${bot.battery} · parts ${bot.parts} · steps ${bot.steps} · bumps ${bot.bumps}${bot.flat ? ' · <em>FLAT</em>' : ""}</span></li>`;
    }).join("");

    $("#now").textContent = done ? "Mission over." : (last ? (() => { const d = log[0]; return `${d.who}, line ${d.line}: ${d.cmd}. ${d.who} ${d.what}`; })() : (state.crew.length ? "Ready. Press Play or Step." : "No bots in this mission. That's allowed."));
    $("#log").innerHTML = log.map(d => `<li><b>${escapeHtml(d.who)}</b> line ${d.line} · ${escapeHtml(d.cmd)} · ${escapeHtml(d.what)}</li>`).join("");

    if (done) {
      const r = A.report(state);
      $("#report").textContent = $("#fmt").value === "json" ? A.toJson(r) : A.toText(r);
      $("#reportbox").dataset.state = "done";
    } else {
      $("#report").textContent = "The report appears when the mission ends. Run to the end to skip ahead.";
      $("#reportbox").dataset.state = "";
    }
  }

  function escapeHtml(s) { return String(s).replace(/[&<>"']/g, c => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[c])); }

  // Compare a pasted `cargo run` output with the Arena's own run, ignoring trailing spaces.
  function compare() {
    const out = $("#cmpout");
    const mine = $("#mine").value.replace(/\r/g, "").split("\n").map(l => l.replace(/\s+$/, "")).join("\n").trim();
    if (!mine) { out.dataset.state = "bad"; out.textContent = "Paste what your cargo run printed first."; return; }
    let want;
    try {
      const text = missionText();
      const r = A.runAll(text, "mission");
      want = mine.startsWith("{") ? A.toJson(r) : A.toText(r);
    } catch (e) { want = "error: " + e.message; }
    want = want.split("\n").map(l => l.replace(/\s+$/, "")).join("\n").trim();
    if (mine === want) { out.dataset.state = "good"; out.textContent = "It matches, line for line. Your robbie-mc follows the spec on this mission."; return; }
    const a = mine.split("\n"), b = want.split("\n");
    let i = 0; while (i < a.length && i < b.length && a[i] === b[i]) i++;
    out.dataset.state = "bad";
    out.textContent = `Line ${i + 1} differs.\nYours:  ${a[i] ?? "(nothing)"}\nArena:  ${b[i] ?? "(nothing)"}`;
  }

  function copyReport() {
    const text = $("#report").textContent;
    const btn = $("#copy");
    const ok = () => { btn.textContent = "Copied"; setTimeout(() => (btn.textContent = "Copy"), 1400); };
    try {
      navigator.clipboard.writeText(text).then(ok, () => selectReport());
    } catch (e) { selectReport(); }
  }
  function selectReport() {
    const r = document.createRange(); r.selectNodeContents($("#report"));
    const s = getSelection(); s.removeAllRanges(); s.addRange(r);
  }

  function init() {
    const sel = $("#preset");
    sel.innerHTML = '<option value="">Load an example…</option>' + Object.entries(PRESETS).map(([k, p]) => `<option value="${k}">${p.label}</option>`).join("");
    sel.addEventListener("change", () => { loadPreset(sel.value); sel.value = ""; });
    document.querySelectorAll("[data-tab]").forEach(b => b.addEventListener("click", () => { setTab(b.dataset.tab); reset(); }));
    $("#run").addEventListener("click", runToEnd);
    $("#step").addEventListener("click", () => { stop(); if (!it || done) reset(); step(); });
    $("#play").addEventListener("click", play);
    $("#reset").addEventListener("click", reset);
    $("#mode").addEventListener("change", reset);
    $("#fmt").addEventListener("change", draw);
    $("#check").addEventListener("click", compare);
    $("#copy").addEventListener("click", copyReport);
    ["#json", "#map", "#script"].forEach(s => $(s).addEventListener("input", () => { stop(); clearTimeout($(s)._t); $(s)._t = setTimeout(reset, 400); }));

    const saved = store.get();
    if (saved && typeof saved.json === "string") {
      $("#json").value = saved.json; $("#map").value = saved.map || ""; $("#script").value = saved.script || "";
      if (saved.mode === "fleet") $("#mode").value = "fleet";
      setTab(saved.tab === "plain" ? "plain" : "json");
      reset();
    } else {
      $("#map").value = PRESETS.tour.map; $("#script").value = PRESETS.tour.script;
      loadPreset("studio");
    }
  }
  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", init); else init();
})();
