// Review Lab: read a grade.json, explain every flag, score the code review, and work out the course grade.
(function () {
  "use strict";
  const $ = s => document.querySelector(s);
  const esc = s => String(s).replace(/[&<>"']/g, c => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[c]));
  const store = window.RUSTUP_STORE || { get: (k, d) => d, set() {} };
  const say = (t, o) => { try { window.RobbieTA && window.RobbieTA.say(t, o || {}); } catch (e) { /* Robbie is optional */ } };

  // ---------- 1. grade.json ----------
  const SAMPLE = {
    robbie_grader: 1, project: "robbie-mc", kind: "final project", total: 67.9, out_of: 100, pending: ["code review"],
    lines: [
      { name: "Build", got: 15, max: 15, note: "builds" },
      { name: "Hidden tests", got: 40, max: 40, note: "final-hidden-tests.rs: 17 of 17 passed" },
      { name: "Your tests", got: 1, max: 10, note: "1 of 1 passed (10 needed)" },
      { name: "Clippy", got: 7, max: 10, note: "3 warnings" },
      { name: "Format", got: 0, max: 5, note: "run cargo fmt" },
      { name: "Docs", got: 4.9, max: 5, note: "cargo doc builds, 34 of 35 public items documented, README present" },
      { name: "Code review", got: 0, max: 15, note: "pending: see the flags below" },
      { name: "Stretch", got: 0, max: 0, note: "up to +5, scored by your teacher" }
    ],
    flags: [
      "clippy: src/cli.rs:25:4: warning: function `helper` is never used",
      "clippy: src/cli.rs:25:56: warning: unneeded `return` statement",
      "clippy: src/cli.rs:25:14: warning: writing `&Vec` instead of `&[_]` involves a new object where a slice will do",
      "review: src/world.rs:91 uses unwrap"
    ]
  };

  // Plain-English notes for the clippy warnings students meet most. First match wins.
  const CLIPPY = [
    [/is never used|never read|never constructed/, "Dead code. Delete it, or if only tests need it, move it under #[cfg(test)]."],
    [/unused variable/, "A variable nobody reads. Remove it, or start its name with _ if you keep it on purpose."],
    [/unused import/, "A use line nothing needs. Delete it."],
    [/unused `?Result`? that must be used|must be used/, "A Result was thrown away, so an error could vanish. Handle it with ? or a match."],
    [/unneeded `return`/, "The last expression of a function is already its return value. Drop return and the semicolon."],
    [/`&Vec` instead of `&\[_\]`|`&String` instead of `&str`/, "Take a slice: &[T] instead of &Vec<T>, &str instead of &String. It accepts more callers and costs nothing."],
    [/redundant clone|clone.*which implements the `Copy` trait|using `clone` on/, "A clone that isn't needed. Use the value directly; Copy types copy themselves."],
    [/can be collapsed|collapsible/, "Two ifs that can be one. Join them with && or a let chain."],
    [/very complex type/, "A type that's hard to read. Give it a name with a type alias, like type Crew = Vec<(Bot, Vec<Command>)>."],
    [/length comparison to zero|`len\(\) == 0`|is_empty/, "Use is_empty() instead of comparing len() with 0."],
    [/needless borrow|immediately dereferenced/, "An & the compiler doesn't need. Remove it."],
    [/redundant field names/, "Write Bot { name } instead of Bot { name: name }."],
    [/single-character string constant/, "Use a char pattern: split('\\n') instead of split(\"\\n\")."],
    [/useless use of `format!`/, "format! with no placeholders. Use .to_string() instead."],
    [/only used to index|could be written as a `for` loop|needless_range_loop/, "Loop over the items, or .iter().enumerate(), instead of indexing by number."],
    [/looks like `matches!`|`matches!`/, "This match only asks yes or no. matches!(x, Pattern) says it in one line."],
    [/single pattern|`if let`/, "A match with one interesting arm reads better as if let."],
    [/`Default` implementation/, "A new() with no arguments should come with a Default impl. derive(Default) often does it."],
    [/manual implementation of|manually reimplementing/, "There's a standard method for this. The clippy message names it; use that."],
    [/large size difference between variants/, "One enum variant is much bigger than the others. Box the big one."],
    [/this `match` (?:arm|has identical arm)|identical/, "Two arms do the same thing. Join their patterns with |."]
  ];
  const REVIEW = {
    unwrap: "Panics if the value is None or Err. On anything that came from input, return an McError with ? instead. If it truly can't fail, a comment saying why is the minimum.",
    expect: "Same as unwrap, with a message. Fine for a promise you've proved; not for input you were handed.",
    "panic!": "A deliberate crash. The spec says bad input is an McError, never a panic.",
    "todo!": "Unfinished. Any hidden test that reaches this line fails.",
    "unimplemented!": "Unfinished. Any hidden test that reaches this line fails.",
    unsafe: "The final project needs no unsafe. If you keep it, it needs a // SAFETY: comment that proves every rule holds."
  };

  function explain(flag) {
    let m;
    if ((m = flag.match(/^clippy: (\S+?:\d+(?::\d+)?): warning: (.*)$/))) {
      const tip = (CLIPPY.find(([re]) => re.test(m[2])) || [null, "Read the full message with cargo clippy; it ends with a link to the lint's own page."])[1];
      return { kind: "Clippy", where: m[1], what: m[2], tip };
    }
    if ((m = flag.match(/^review: (\S+?:\d+) uses (.+)$/))) return { kind: "Review", where: m[1], what: "uses " + m[2], tip: REVIEW[m[2]] || "Worth a look in the code review." };
    if (/^error/.test(flag)) return { kind: "Build error", where: "", what: flag, tip: "This stops the grading. Run cargo build and fix the first error first; later ones often go away with it." };
    return { kind: "Note", where: "", what: flag, tip: "" };
  }

  let grade = null;
  function showGrade(g) {
    grade = g;
    const out = $("#gradeout");
    if (!g || !Array.isArray(g.lines)) { out.innerHTML = '<p class="errbox">That isn\'t a grade.json from robbie-grader. Look for grade.json in your project folder after the grader runs.</p>'; return; }
    const lines = g.lines.map(l => {
      const max = Number(l.max) || 0, got = Number(l.got) || 0;
      const pct = max ? Math.min(100, got / max * 100) : 0;
      return `<div class="scoreline"><b>${esc(l.name)}</b><span class="pts">${got} / ${max || "+5"}</span>${max ? `<span class="bar"><i style="width:${pct}%"></i></span>` : ""}<span class="why">${esc(l.note || "")}</span></div>`;
    }).join("");
    const flags = (g.flags || []).map(explain);
    const groups = ["Build error", "Clippy", "Review", "Note"].map(k => {
      const fs = flags.filter(f => f.kind === k);
      if (!fs.length) return "";
      return `<h3 style="margin-top:12px">${k === "Review" ? "For the code review" : k + (fs.length > 1 && k !== "Clippy" ? "s" : "")}${k === "Clippy" ? " warnings" : ""} <span class="muted">(${fs.length})</span></h3>` +
        fs.map(f => `<div class="flag">${f.where ? `<code>${esc(f.where)}</code> ` : ""}${esc(f.what)}${f.tip ? `<span class="tip">${esc(f.tip)}</span>` : ""}</div>`).join("");
    }).join("");
    const pending = (g.pending || []).length ? `<p class="muted" style="margin:0">Still to come: ${esc(g.pending.join(" and "))}.</p>` : "";
    out.innerHTML = `<div class="row" style="justify-content:space-between;align-items:baseline"><h3>${esc(g.kind || "project")}</h3><span style="font:400 var(--step-3)/1 var(--f-display)">${Number(g.total)} <span class="muted" style="font-size:var(--step-0)">/ ${Number(g.out_of) || 100}</span></span></div>${pending}<div>${lines}</div>${groups || '<p class="muted">No flags. Clean build, clean clippy, nothing for the reviewer to chase.</p>'}`;
    if ($("#path") && g.project && !$("#path").dataset.touched) $("#path").value = String(g.project);
    updateReview();
    const warn = flags.filter(f => f.kind !== "Note").length;
    say(warn ? `${warn} thing${warn > 1 ? "s" : ""} to look at. Each one has a note under it. Start from the top.` : "Not a single flag. I'm so proud my bolts are rattling.", { emote: warn ? "think" : "happy", ttl: 8000 });
  }

  function readFile(file) {
    const r = new FileReader();
    r.onload = () => { try { showGrade(JSON.parse(String(r.result))); } catch (e) { showGrade(null); } };
    r.readAsText(file);
  }

  // ---------- 2. code review rubric ----------
  const CRITERIA = [
    { id: "errors", name: "Errors, not panics", max: 4, text: "Every problem in a map, script, mission or argument is an McError. No unwrap, expect or panic! on input in src/." },
    { id: "own", name: "Ownership", max: 3, text: "Borrows where it can (&str, &[T], &World), clones only with a reason, and run_fleet holds the lock for one command at a time." },
    { id: "shape", name: "Structure", max: 3, text: "Each idea lives in one place: tiles in world.rs, commands in command.rs, Bot::run is one match. The exam-day features went in cleanly." },
    { id: "names", name: "Naming and readability", max: 3, text: "Names say what things are. No dead code or commented-out blocks. Each function fits on one screen." },
    { id: "tests", name: "Tests that test rules", max: 2, text: "tests/mine.rs checks the rules (bumps, flat, each error), not only the examples, and each test name says which rule." }
  ];
  const picks = Object.assign({}, store.get("review:picks", {}));
  const notes = Object.assign({}, store.get("review:notes", {}));

  function buildRubric() {
    $("#crit").innerHTML = CRITERIA.map(c => `<div class="crit" data-id="${c.id}"><b>${c.name} <span class="muted">/ ${c.max}</span></b>
      <span class="pick" role="group" aria-label="${c.name} score">${Array.from({ length: c.max + 1 }, (_, n) => `<button type="button" data-n="${n}" aria-pressed="false">${n}</button>`).join("")}</span>
      <p>${c.text}</p>
      <label class="field" style="grid-column:1/-1">Comment (optional)<input type="text" data-note="${c.id}" placeholder="What, where and why, e.g. world.rs:40 unwraps the first row"></label></div>`).join("");
    document.querySelectorAll(".crit .pick button").forEach(b => b.addEventListener("click", () => {
      picks[b.closest(".crit").dataset.id] = Number(b.dataset.n); store.set("review:picks", picks); updateReview();
    }));
    document.querySelectorAll("[data-note]").forEach(i => { i.value = notes[i.dataset.note] || ""; i.addEventListener("input", () => { notes[i.dataset.note] = i.value.trim(); store.set("review:notes", notes); updateReview(); }); });
    ["#stretch", "#path", "#examday"].forEach(s => $(s).addEventListener("input", () => { if (s === "#path") $("#path").dataset.touched = "1"; updateReview(); }));
    $("#examday").addEventListener("change", updateReview);
    $("#copyfb").addEventListener("click", () => copy($("#feedback"), $("#copyfb")));
    $("#copycmd").addEventListener("click", () => copy($("#cmd"), $("#copycmd")));
  }

  function updateReview() {
    document.querySelectorAll(".crit").forEach(el => el.querySelectorAll("button").forEach(b => b.setAttribute("aria-pressed", String(picks[el.dataset.id] === Number(b.dataset.n)))));
    const done = CRITERIA.every(c => picks[c.id] != null);
    const review = CRITERIA.reduce((a, c) => a + (picks[c.id] || 0), 0);
    const stretch = Math.max(0, Math.min(5, Math.round(Number($("#stretch").value) || 0)));
    $("#revtotal").textContent = `${review} / 15${done ? "" : " so far"}`;
    const path = ($("#path").value || "path/to/robbie-mc").trim();
    const q = /[\s'"$`\\]/.test(path) ? `'${path.replace(/'/g, "'\\''")}'` : path;
    $("#cmd").textContent = `cargo run --release --manifest-path grader/Cargo.toml -- ${q} \\\n  --hidden teacher/grading/final-hidden-tests.rs \\\n` +
      ($("#examday").checked ? "  --hidden teacher/grading/final-examday-tests.rs \\\n" : "") + `  --review ${review} --stretch ${stretch}`;
    $("#feedback").textContent = `Code review: ${review} / 15${stretch ? `, stretch +${stretch}` : ""}\n` +
      CRITERIA.map(c => `- ${c.name}: ${picks[c.id] != null ? picks[c.id] : "?"} / ${c.max}${notes[c.id] ? `. ${notes[c.id]}` : ""}`).join("\n");
    if (grade && Array.isArray(grade.lines)) {
      const old = grade.lines.filter(l => /code review|stretch/i.test(l.name)).reduce((a, l) => a + (Number(l.got) || 0), 0);
      const t = Math.round((Number(grade.total) - old + review + stretch) * 10) / 10;
      $("#newtotal").textContent = `With this review, ${grade.project ? "this project" : "the project"} comes to ${t} / ${Number(grade.out_of) || 100}${(grade.pending || []).includes("hidden tests") ? " (the hidden tests haven't run yet)" : ""}.`;
    } else $("#newtotal").textContent = "";
  }

  function copy(el, btn) {
    const done = () => { const t = btn.textContent; btn.textContent = "Copied"; setTimeout(() => (btn.textContent = t), 1400); };
    const sel = () => { const r = document.createRange(); r.selectNodeContents(el); const s = getSelection(); s.removeAllRanges(); s.addRange(r); };
    try { navigator.clipboard.writeText(el.textContent).then(done, sel); } catch (e) { sel(); }
  }

  // ---------- 3. course grade ----------
  const PARTS = [
    { id: "quiz", name: "Quizzes", weight: 15, items: ["s1", "s2", "s3", "s4", "s5", "s6"].map((s, i) => ({ key: "quiz:" + s, label: `Quiz ${i + 1}`, max: 100, unit: "%" })) },
    { id: "hw", name: "Homework", weight: 25, items: [1, 2, 3, 4, 5].map(n => ({ key: "hw" + n, label: `Homework ${n}`, max: 10, unit: "/ 10" })) },
    { id: "cp", name: "Checkpoints", weight: 20, items: [{ key: "quiz:s3-cp1", label: "Checkpoint 1", max: 100, unit: "%" }, { key: "quiz:s5-cp2", label: "Checkpoint 2", max: 100, unit: "%" }] },
    { id: "final", name: "Final project", weight: 40, items: [{ key: "final", label: "Final project", max: 105, unit: "/ 100", outOf: 100 }] }
  ];
  const vals = Object.assign({}, store.get("grade:vals", {}));

  function buildCalc() {
    $("#calc").innerHTML = PARTS.map(p => `<fieldset class="panel" style="margin:0"><legend style="font:400 var(--step-1) var(--f-display);padding:0 6px">${p.name} <span class="muted" style="font:500 .8rem var(--f-mono)">${p.weight}%</span></legend>
      <div class="row">${p.items.map(it => `<label class="field" style="width:7.5rem">${it.label} <span style="text-transform:none">${it.unit}</span><input type="number" inputmode="decimal" min="0" max="${it.max}" step="any" data-k="${it.key}" value="${vals[it.key] ?? ""}"></label>`).join("")}</div></fieldset>`).join("");
    document.querySelectorAll("[data-k]").forEach(i => i.addEventListener("input", () => { if (i.value === "") delete vals[i.dataset.k]; else vals[i.dataset.k] = Number(i.value); store.set("grade:vals", vals); calc(); }));
    $("#fillquiz").addEventListener("click", () => {
      const P = window.RUSTUP_PROGRESS; let n = 0;
      PARTS.forEach(p => p.items.forEach(it => { if (it.key.startsWith("quiz:")) { const v = P && P.get(it.key); if (typeof v === "number") { vals[it.key] = v; n++; } } }));
      store.set("grade:vals", vals); buildCalcValues(); calc();
      $("#fillmsg").textContent = n ? `Filled ${n} best score${n > 1 ? "s" : ""} saved in this browser.` : "No quiz scores saved in this browser yet. Take a quiz and they'll show up here.";
    });
    $("#certname").value = store.get("grade:name", "");
    $("#certname").addEventListener("input", () => { store.set("grade:name", $("#certname").value); calc(); });
    calc();
  }
  function buildCalcValues() { document.querySelectorAll("[data-k]").forEach(i => { i.value = vals[i.dataset.k] ?? ""; }); }

  function calc() {
    let have = 0, sum = 0; const missing = [];
    const rows = PARTS.map(p => {
      const got = p.items.filter(it => typeof vals[it.key] === "number" && !isNaN(vals[it.key]));
      if (got.length < p.items.length) missing.push(p.name);
      if (!got.length) return `<tr><td>${p.name}</td><td>${p.weight}%</td><td class="muted">not yet</td></tr>`;
      const pct = got.reduce((a, it) => a + Math.max(0, Math.min(it.max, vals[it.key])) / (it.outOf || it.max), 0) / got.length * 100;
      have += p.weight; sum += pct * p.weight / 100;
      return `<tr><td>${p.name}</td><td>${p.weight}%</td><td>${Math.round(pct * 10) / 10}%${got.length < p.items.length ? ` <span class="muted">(${got.length} of ${p.items.length})</span>` : ""}</td></tr>`;
    }).join("");
    const grade = have ? Math.min(100, sum / have * 100) : 0;
    const g = Math.round(grade * 10) / 10;
    $("#calcrows").innerHTML = rows;
    $("#coursegrade").textContent = have ? `${g}%` : "–";
    $("#gradenote").textContent = !have ? "Fill in what you have so far." : missing.length ? `So far, from what's filled in. Still to come: ${missing.join(", ")}.` : (g >= 90 ? "Every part is in. Certificate earned." : `Every part is in. ${Math.round((90 - g) * 10) / 10} points short of the certificate.`);
    const earned = have && !missing.length && g >= 90;
    $("#cert").hidden = !earned;
    if (earned) {
      const name = $("#certname").value.trim();
      $("#certwho").textContent = name || "Your name here";
      $("#certscore").textContent = `${g}%`;
      if (!calc.said) { calc.said = true; say("Rustacean certificate! I'd shake your hand, but I'd leave a mark.", { emote: "happy", ttl: 9000 }); }
    }
  }

  function init() {
    const drop = $("#drop"), file = $("#file");
    file.addEventListener("change", () => file.files[0] && readFile(file.files[0]));
    ["dragenter", "dragover"].forEach(e => drop.addEventListener(e, ev => { ev.preventDefault(); drop.classList.add("over"); }));
    ["dragleave", "drop"].forEach(e => drop.addEventListener(e, ev => { ev.preventDefault(); drop.classList.remove("over"); }));
    drop.addEventListener("drop", ev => { const f = ev.dataTransfer && ev.dataTransfer.files[0]; if (f) readFile(f); });
    $("#paste").addEventListener("click", () => { try { showGrade(JSON.parse($("#json").value)); } catch (e) { showGrade(null); } });
    $("#sample").addEventListener("click", () => { $("#json").value = JSON.stringify(SAMPLE, null, 2); showGrade(SAMPLE); });
    buildRubric(); updateReview(); buildCalc();
  }
  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", init); else init();
})();
