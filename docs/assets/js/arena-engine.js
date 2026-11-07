// Robbie Arena engine: the final project's base rules, ported from Rust to JavaScript.
// Same map rules, same commands, same checks in the same order, same report text.
// It runs a mission as a sequence of frames so the page can animate it.
// (Exam-day features are not here on purpose. They're revealed on the day.)
(function (root) {
  "use strict";

  const TILES = { ".": "floor", "#": "wall", "P": "part" };
  const CHARS = { floor: ".", wall: "#", part: "P" };
  const LEFT = { N: "W", W: "S", S: "E", E: "N" };
  const RIGHT = { N: "E", E: "S", S: "W", W: "N" };
  const STEP = { N: [0, -1], E: [1, 0], S: [0, 1], W: [-1, 0] };
  const U32 = 4294967295;

  class McError extends Error {}

  // Rust's str::lines(): split on \n, drop a trailing \r, no empty last line.
  function lines(s) {
    const out = s.split("\n");
    if (out.length && out[out.length - 1] === "") out.pop();
    return out.map(l => (l.endsWith("\r") ? l.slice(0, -1) : l));
  }
  // Rust's trim_end / trim use Unicode White_Space; \s is close enough for maps and scripts.
  const trimEnd = s => s.replace(/\s+$/u, "");
  const trim = s => s.replace(/^\s+|\s+$/gu, "");

  function parseWorld(map) {
    const rows = [];
    lines(map).forEach((raw, i) => {
      const line = trimEnd(raw);
      if (line === "") return;
      const row = [];
      Array.from(line).forEach((ch, c) => {
        const t = TILES[ch];
        if (!t) throw new McError(`map line ${i + 1}, column ${c + 1}: unknown tile '${ch}'`);
        row.push(t);
      });
      if (rows.length && rows[0].length !== row.length) throw new McError(`map line ${i + 1} is a different length from the first row`);
      rows.push(row);
    });
    if (!rows.length) throw new McError("the map is empty");
    return { rows };
  }
  const width = w => w.rows[0].length;
  const height = w => w.rows.length;
  const tile = (w, x, y) => (w.rows[y] && w.rows[y][x]) || null;
  const render = w => w.rows.map(r => r.map(t => CHARS[t]).join("")).join("\n");
  const partsLeft = w => w.rows.flat().filter(t => t === "part").length;

  function parseScript(src) {
    const cmds = [];
    lines(src).forEach((raw, i) => {
      const line = trim(raw);
      if (line === "" || line.startsWith("#")) return;
      const words = line.toLowerCase().split(/\s+/u);
      const bad = () => Object.assign(new McError(`script line ${i + 1}: can't understand '${line}'`), { line: i + 1, text: line });
      let cmd;
      if (words.length === 1 && words[0] === "forward") cmd = { op: "forward", n: 1, line: i + 1 };
      else if (words.length === 2 && words[0] === "forward") {
        if (!/^\+?[0-9]+$/.test(words[1])) throw bad();
        const n = Number(words[1].replace("+", ""));
        if (!(n > 0 && n <= U32)) throw bad();
        cmd = { op: "forward", n, line: i + 1 };
      } else if (words.length === 1 && ["left", "right", "pick", "wait"].includes(words[0])) cmd = { op: words[0], line: i + 1 };
      else throw bad();
      cmds.push(cmd);
    });
    return cmds;
  }

  const isU = (v, max) => Number.isInteger(v) && v >= 0 && v <= max;

  // Mission::from_json + prepare, in the same order: map, then each bot's name, start, wall, facing, script.
  function prepare(text) {
    let m;
    try { m = JSON.parse(text); } catch (e) { throw new McError(`can't read mission: ${e.message}`); }
    const shape = msg => new McError(`can't read mission: ${msg}`);
    if (!m || typeof m !== "object" || Array.isArray(m)) throw shape("expected an object with map and bots");
    if (typeof m.map !== "string") throw shape("map must be a string");
    if (!Array.isArray(m.bots)) throw shape("bots must be a list");
    m.bots.forEach((b, i) => {
      if (!b || typeof b !== "object") throw shape(`bot ${i + 1} must be an object`);
      if (typeof b.name !== "string") throw shape(`bot ${i + 1} needs a name string`);
      if (!Array.isArray(b.start) || b.start.length !== 2 || !b.start.every(v => isU(v, Number.MAX_SAFE_INTEGER))) throw shape(`${b.name}'s start must be [x, y] with whole numbers from 0`);
      if (typeof b.facing !== "string") throw shape(`${b.name}'s facing must be a string`);
      if (!isU(b.battery, U32)) throw shape(`${b.name}'s battery must be a whole number from 0`);
      if (typeof b.script !== "string") throw shape(`${b.name}'s script must be a string`);
    });
    const world = parseWorld(m.map);
    const seen = new Set();
    const crew = [];
    for (const s of m.bots) {
      if (seen.has(s.name)) throw new McError(`two bots are called ${s.name}`);
      seen.add(s.name);
      const [x, y] = s.start;
      const t = tile(world, x, y);
      if (!t) throw new McError(`${s.name} starts off the map`);
      if (t === "wall") throw new McError(`${s.name} starts inside a wall`);
      const f = s.facing.toUpperCase();
      if (!["N", "E", "S", "W"].includes(f)) throw new McError(`${s.name} has an unknown facing '${s.facing}'`);
      let cmds;
      try { cmds = parseScript(s.script); } catch (e) {
        if (e.line) throw new McError(`${s.name}'s script line ${e.line}: can't understand '${e.text}'`);
        throw e;
      }
      crew.push({ bot: { name: s.name, x, y, facing: f, battery: s.battery, capacity: s.battery, parts: 0, steps: 0, bumps: 0, flat: false }, cmds });
    }
    return { world, crew };
  }

  // Bot::run, one command, yielding a frame after every tile so the page can animate it.
  function* run(bot, world, cmd) {
    if (bot.flat) return;
    if (cmd.op === "forward") {
      for (let k = 0; k < cmd.n; k++) {
        if (bot.battery === 0) { bot.flat = true; yield "flat"; return; }
        bot.battery -= 1;
        const [dx, dy] = STEP[bot.facing];
        const nx = bot.x + dx, ny = bot.y + dy;
        const ok = nx >= 0 && ny >= 0 && nx < width(world) && ny < height(world) && tile(world, nx, ny) !== "wall";
        if (ok) { bot.x = nx; bot.y = ny; bot.steps += 1; yield "move"; }
        else { bot.bumps += 1; yield "bump"; return; }
      }
    } else if (cmd.op === "left") { bot.facing = LEFT[bot.facing]; yield "turn"; }
    else if (cmd.op === "right") { bot.facing = RIGHT[bot.facing]; yield "turn"; }
    else if (cmd.op === "pick") {
      if (tile(world, bot.x, bot.y) === "part") { world.rows[bot.y][bot.x] = "floor"; bot.parts += 1; yield "pick"; }
      else yield "nothing";
    } else yield "wait";
  }

  // A whole mission as frames. "mission": one bot after another (run_mission).
  // "fleet": bots take turns one command at a time, one fair order run_fleet could take.
  function* frames(state, mode) {
    const { world, crew } = state;
    const one = function* (i) {
      const { bot, cmds } = crew[i];
      for (let c = 0; c < cmds.length; c++) {
        for (const what of run(bot, world, cmds[c])) yield { bot: i, cmd: c, what };
        if (bot.flat) break;
      }
    };
    if (mode === "fleet") {
      const its = crew.map((_, i) => {
        const { bot, cmds } = crew[i];
        let c = 0;
        return function* () {
          if (c >= cmds.length || bot.flat) return false;
          const at = c++;
          let any = false;
          for (const what of run(bot, world, cmds[at])) { any = true; yield { bot: i, cmd: at, what }; }
          if (!any) yield { bot: i, cmd: at, what: "ignored" };
          return true;
        };
      });
      let alive = true;
      while (alive) {
        alive = false;
        for (const step of its) {
          const r = yield* step();
          if (r) alive = true;
        }
      }
    } else {
      for (let i = 0; i < crew.length; i++) yield* one(i);
    }
  }

  function report(state) {
    const bots = state.crew.map(({ bot }) => ({
      name: bot.name, end: [bot.x, bot.y], facing: bot.facing, battery: bot.battery,
      parts: bot.parts, steps: bot.steps, bumps: bot.bumps, flat: bot.flat
    }));
    return { bots, parts_collected: bots.reduce((a, b) => a + b.parts, 0), parts_left: partsLeft(state.world), map: render(state.world) };
  }
  const pad = (v, n) => { const s = String(v); const len = Array.from(s).length; return len >= n ? s : s + " ".repeat(n - len); };
  function toText(r) {
    let out = "Mission report\n";
    for (const b of r.bots) {
      out += `  ${pad(b.name, 8)} at (${b.end[0]}, ${b.end[1]}) facing ${b.facing}   battery ${pad(b.battery, 3)} parts ${pad(b.parts, 3)} steps ${pad(b.steps, 3)} bumps ${b.bumps}${b.flat ? "   FLAT" : ""}\n`;
    }
    return out + `Parts collected: ${r.parts_collected}. Parts left: ${r.parts_left}.`;
  }
  const toJson = r => JSON.stringify(r, null, 2);

  // The CLI's map + script shortcut: Robbie, first tile that isn't a wall, facing East, battery 50.
  function missionFromMapAndScript(map, script) {
    const w = parseWorld(map);
    for (let y = 0; y < height(w); y++) for (let x = 0; x < width(w); x++) {
      if (tile(w, x, y) !== "wall") return JSON.stringify({ map, bots: [{ name: "Robbie", start: [x, y], facing: "E", battery: 50, script }] }, null, 2);
    }
    throw new McError("the map has no floor to start on");
  }

  function runAll(text, mode) {
    const state = prepare(text);
    for (const _ of frames(state, mode)) { /* run to the end */ }
    return report(state);
  }

  const api = { McError, parseWorld, parseScript, prepare, frames, report, toText, toJson, runAll, render, missionFromMapAndScript, width, height, tile };
  if (typeof module !== "undefined" && module.exports) module.exports = api;
  else root.ARENA = api;
})(typeof window !== "undefined" ? window : globalThis);
