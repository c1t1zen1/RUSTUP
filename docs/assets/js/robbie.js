// Robbie, the RUSTUP teaching assistant: the dock, the speech bubble and the tip deck.
// The 3D body (robbie3d.js) plugs into RobbieTA.body once it loads; until then a still render stands in.
(function () {
  const base = document.body.dataset.base || ".";
  const page = window.ROBBIE_PAGE || {};
  const rust = Number(document.body.dataset.rust || 0);
  const rustNames = ["Factory fresh", "Rained on", "Streaky", "Half rust", "Shedding parts", "Mostly flakes"];
  const rustPct = [0, 15, 32, 50, 72, 95][rust] ?? 0;

  const dock = document.createElement("div");
  dock.className = "robbie-dock";
  dock.innerHTML = `<img class="fallback" src="${base}/assets/img/robbie-front.jpg" alt="Robbie, the course robot. Tap for help." tabindex="0" role="button">
    <span class="rust-chip" title="${rustNames[rust] || ""}: how much of Robbie is rust this session">Rust ${rustPct}%</span>`;
  document.body.appendChild(dock);

  let bubble = null, tipIdx = 0, eliIdx = 0, hypeIdx = 0;
  const defaults = {
    greeting: "Beep. I'm Robbie, your teaching assistant. Tap a button and I'll help.",
    tips: ["Read compiler errors from the top. The first one is usually the real one; the rest are its fan club."],
    eli5: ["Rust is a language that checks your work before it runs, like a friend who proofreads before you hit send."],
    hype: ["You compiled something today. That's more than most toasters ever do."]
  };
  const P = Object.assign({}, defaults, page);

  function close() { if (bubble) { bubble.remove(); bubble = null; } }
  function say(html, opts) {
    opts = opts || {};
    close();
    bubble = document.createElement("div");
    bubble.className = "bubble"; bubble.setAttribute("role", "status");
    bubble.innerHTML = `<button class="x" type="button" aria-label="Close">×</button><b class="who">ROBBIE</b><div class="msg">${html}</div>
      <div class="acts">
        <button type="button" data-a="tip">Give me a tip</button>
        <button type="button" data-a="eli5">Explain it simply</button>
        <button type="button" data-a="hype">Hype me up</button>
        <button type="button" data-a="dance">Dance</button>
      </div>`;
    document.body.appendChild(bubble);
    bubble.querySelector(".x").onclick = close;
    bubble.querySelectorAll(".acts button").forEach(b => b.onclick = () => act(b.dataset.a));
    if (opts.emote) emote(opts.emote);
    if (opts.ttl) { const me = bubble; setTimeout(() => { if (bubble === me) close(); }, opts.ttl); }
  }
  function act(a) {
    if (a === "tip") { say(P.tips[tipIdx++ % P.tips.length], { emote: "think" }); }
    if (a === "eli5") { say(P.eli5[eliIdx++ % P.eli5.length], { emote: "look" }); }
    if (a === "hype") { say(P.hype[hypeIdx++ % P.hype.length], { emote: "happy" }); }
    if (a === "dance") { say("Engaging groove subroutine. Please keep arms and legs inside the borrow checker.", { emote: "dance" }); }
  }
  function emote(name) { if (RobbieTA.body && RobbieTA.body.play) RobbieTA.body.play(name); }

  window.RobbieTA = {
    body: null, rust, rustName: rustNames[rust] || "",
    say, close, emote,
    tip() { act("tip"); },
    react(ok, msg) { say(msg || (ok ? "Correct! My cassette deck is playing a victory jingle." : "Not quite. Rust says no, kindly. Try again."), { emote: ok ? "happy" : "think", ttl: 7000 }); },
    greet() { say(P.greeting, { emote: "wave" }); }
  };

  dock.addEventListener("click", e => { if (e.target.closest("canvas, .fallback")) { bubble ? close() : RobbieTA.greet(); } });
  dock.addEventListener("keydown", e => { if ((e.key === "Enter" || e.key === " ") && e.target.classList.contains("fallback")) { e.preventDefault(); RobbieTA.greet(); } });
  document.addEventListener("keydown", e => { if (e.key === "Escape") close(); });

  // Say hello once per page per browser.
  try {
    const k = "rustup:greeted:" + (document.body.dataset.page || location.pathname);
    if (!sessionStorage.getItem(k)) { sessionStorage.setItem(k, "1"); setTimeout(() => RobbieTA.say(P.greeting, { emote: "wave", ttl: 9000 }), 1200); }
  } catch (e) {}
})();
