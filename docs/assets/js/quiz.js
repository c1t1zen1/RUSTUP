// Quiz engine: renders window.QUIZ, checks answers per question, scores, saves best score in this browser.
(function () {
  const Q = window.QUIZ; const host = document.getElementById("quiz");
  if (!Q || !host) return;
  const esc = s => s.replace(/&/g, "&amp;").replace(/</g, "&lt;");
  const state = Q.questions.map(() => ({ done: false, ok: false }));
  host.innerHTML = Q.questions.map((q, i) => `
    <div class="q" id="q${i}">
      <div class="qn">Question ${i + 1} of ${Q.questions.length}</div>
      <div class="prompt">${q.prompt}</div>
      ${q.code ? `<pre data-lang="rust" data-title="code">${esc(q.code)}</pre>` : ""}
      ${q.type === "mc"
        ? `<div class="opts" role="radiogroup">${q.options.map((o, k) => `<label><input type="radio" name="q${i}" id="q${i}o${k}" value="${k}"> <span>${esc(o)}</span></label>`).join("")}</div>`
        : `<label for="q${i}t" class="muted" style="display:block;margin-bottom:6px;font-size:.85rem">Type your answer</label><input type="text" id="q${i}t" autocomplete="off" spellcheck="false">`}
      <div class="cta-row" style="margin-top:12px"><button class="btn check" type="button">Check</button></div>
      <div class="why" hidden></div>
    </div>`).join("");

  // re-run code block enhancement for the freshly added <pre>s
  if (window.hljs) host.querySelectorAll("pre[data-lang]").forEach(pre => {
    const code = document.createElement("code"); code.className = "language-rust"; code.textContent = pre.textContent;
    pre.textContent = ""; pre.appendChild(code); try { hljs.highlightElement(code); } catch (e) {}
    const w = document.createElement("div"); w.className = "code"; pre.parentNode.insertBefore(w, pre); w.appendChild(pre);
  });

  const scoreEl = document.getElementById("score"), msgEl = document.getElementById("scoreMsg");
  function update() {
    const done = state.filter(s => s.done).length, right = state.filter(s => s.ok).length;
    const pct = Math.round(right / Q.questions.length * 100);
    scoreEl.textContent = `${right} / ${Q.questions.length}`;
    if (done < Q.questions.length) { msgEl.textContent = `${Q.questions.length - done} to go`; return; }
    const best = Math.max(pct, (window.RUSTUP_PROGRESS && RUSTUP_PROGRESS.get("quiz:" + Q.id)) || 0);
    window.RUSTUP_PROGRESS && RUSTUP_PROGRESS.mark("quiz:" + Q.id, best);
    if (pct === 100) { msgEl.textContent = `${pct}%. Perfect score. Robbie is doing a little dance.`; RobbieTA && RobbieTA.say("PERFECT! 100%! Engaging celebration protocol!", { emote: "dance", ttl: 9000 }); }
    else if (pct >= Q.pass) { msgEl.textContent = `${pct}%. Passed. Best so far: ${best}%.`; RobbieTA && RobbieTA.say(`You passed with ${pct}%. Read the explanations on the ones you missed, then onward!`, { emote: "happy", ttl: 9000 }); }
    else { msgEl.textContent = `${pct}%. Pass mark is ${Q.pass}%. Reread the handbook sections for the ones you missed, then try again.`; RobbieTA && RobbieTA.say("Not a pass yet, and that's fine. Every wrong answer here is one you won't get wrong in the final.", { emote: "think", ttl: 9000 }); }
  }
  Q.questions.forEach((q, i) => {
    const box = document.getElementById("q" + i);
    box.querySelector(".check").addEventListener("click", () => {
      if (state[i].done) return;
      let ok = false;
      if (q.type === "mc") {
        const picked = box.querySelector("input:checked");
        if (!picked) { box.querySelector(".why").hidden = false; box.querySelector(".why").textContent = "Pick an answer first."; return; }
        ok = +picked.value === q.answer;
        box.querySelectorAll("label").forEach((l, k) => { if (k === q.answer) l.classList.add("right"); else if (l.contains(picked)) l.classList.add("picked-wrong"); });
        box.querySelectorAll("input").forEach(x => x.disabled = true);
      } else {
        const v = box.querySelector("input").value.trim().toLowerCase().replace(/\s+/g, "");
        if (!v) { box.querySelector(".why").hidden = false; box.querySelector(".why").textContent = "Type an answer first."; return; }
        ok = q.answers.some(a => a.toLowerCase().replace(/\s+/g, "") === v);
        box.querySelector("input").disabled = true;
      }
      state[i] = { done: true, ok };
      box.classList.add(ok ? "correct" : "wrong");
      const why = box.querySelector(".why"); why.hidden = false;
      why.innerHTML = `<b>${ok ? "Correct." : "Not quite."}</b> ${q.why}${!ok && q.type === "text" ? ` (Answer: <code>${q.answers[0]}</code>)` : ""}`;
      box.querySelector(".check").disabled = true;
      if (window.RobbieTA && state.filter(s => s.done).length < Q.questions.length) RobbieTA.emote(ok ? "happy" : "think");
      update();
    });
  });
  document.getElementById("retry").addEventListener("click", () => location.reload());
  update();
})();
