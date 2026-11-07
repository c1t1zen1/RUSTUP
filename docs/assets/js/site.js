// RUSTUP shared page behaviour: top bar, progress, code blocks, Rust Playground runner.
(function () {
  const C = window.RUSTUP;
  const root = document.documentElement;
  const base = document.body.dataset.base || ".";

  // ---------- storage (per browser only; never required) ----------
  const store = {
    get(k, d) { try { const v = localStorage.getItem("rustup:" + k); return v == null ? d : JSON.parse(v); } catch (e) { return d; } },
    set(k, v) { try { localStorage.setItem("rustup:" + k, JSON.stringify(v)); } catch (e) {} }
  };
  window.RUSTUP_STORE = store;

  // ---------- theme ----------
  const savedTheme = store.get("theme", null);
  if (savedTheme) root.setAttribute("data-theme", savedTheme);
  function toggleTheme() {
    const cur = root.getAttribute("data-theme") ||
      (matchMedia("(prefers-color-scheme: light)").matches ? "light" : "dark");
    const next = cur === "light" ? "dark" : "light";
    root.setAttribute("data-theme", next); store.set("theme", next);
  }

  // ---------- progress ----------
  window.RUSTUP_PROGRESS = {
    all() { return store.get("progress", {}); },
    mark(key, val) { const p = this.all(); p[key] = val; store.set("progress", p); },
    get(key) { return this.all()[key]; }
  };

  // ---------- top bar ----------
  const bar = document.querySelector("header.topbar");
  if (bar) {
    const crumbs = bar.dataset.crumbs ? JSON.parse(bar.dataset.crumbs) : [];
    bar.innerHTML = `<div class="wrap">
      <a class="brand" href="${base}/index.html">RUST<span>UP</span></a>
      <nav class="crumbs" aria-label="Breadcrumb">${crumbs.map(c => c.href ? `<a href="${c.href}">${c.t}</a>` : `<span>${c.t}</span>`).join("<span aria-hidden='true'>/</span>")}</nav>
      <span class="spacer"></span>
      <button class="btn ghost" id="themeBtn" type="button" aria-label="Switch light or dark theme">Theme</button>
    </div>`;
    bar.querySelector("#themeBtn").addEventListener("click", toggleTheme);
  }

  // ---------- code blocks ----------
  // <pre data-run> runs on the Playground; <pre data-lang="toml|sh|text"> is display only.
  const PLAY = "https://play.rust-lang.org/";
  function playgroundLink(code) {
    return PLAY + "?version=stable&mode=debug&edition=2024&code=" + encodeURIComponent(code);
  }
  async function runOnPlayground(code) {
    // Same public endpoint mdBook uses for its Run buttons.
    const res = await fetch(PLAY + "evaluate.json", {
      method: "POST", headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ version: "stable", optimize: "0", code, edition: "2024" })
    });
    if (!res.ok) throw new Error("HTTP " + res.status);
    return res.json();
  }
  document.querySelectorAll("pre[data-run], pre[data-lang]").forEach(pre => {
    const runnable = pre.hasAttribute("data-run");
    const lang = pre.dataset.lang || "rust";
    const raw = pre.textContent.replace(/^\n/, "").replace(/\s+$/, "");
    const wrap = document.createElement("div");
    wrap.className = "code";
    const label = pre.dataset.title || (runnable ? "main.rs" : lang);
    wrap.innerHTML = `<div class="bar"><span>${label}</span><span class="spacer"></span>
      <button type="button" class="copy">Copy</button>
      ${runnable ? `<button type="button" class="open">Playground</button><button type="button" class="run">Run</button>` : ""}</div>`;
    pre.parentNode.insertBefore(wrap, pre);
    const codeEl = document.createElement("code");
    codeEl.className = "language-" + (lang === "sh" ? "bash" : lang);
    codeEl.textContent = raw;
    pre.textContent = ""; pre.appendChild(codeEl);
    wrap.appendChild(pre);
    if (window.hljs) { try { hljs.highlightElement(codeEl); } catch (e) {} }
    const out = document.createElement("div"); out.className = "out"; out.hidden = true; wrap.appendChild(out);
    wrap.querySelector(".copy").addEventListener("click", e => {
      const b = e.currentTarget;
      navigator.clipboard && navigator.clipboard.writeText(raw).then(() => { b.textContent = "Copied"; setTimeout(() => b.textContent = "Copy", 1200); })
        .catch(() => { const r = document.createRange(); r.selectNodeContents(codeEl); const s = getSelection(); s.removeAllRanges(); s.addRange(r); b.textContent = "Selected"; });
    });
    if (!runnable) return;
    const open = wrap.querySelector(".open");
    const a = document.createElement("a"); // real link so it works everywhere
    a.href = playgroundLink(raw); a.target = "_blank"; a.rel = "noopener"; a.textContent = "Playground";
    a.className = "open"; a.style.cssText = "font:600 .72rem var(--f-mono);padding:5px 10px;border-radius:6px;border:1px solid var(--line);background:var(--bg-2);color:var(--ink);text-decoration:none";
    open.replaceWith(a);
    wrap.querySelector(".run").addEventListener("click", async () => {
      out.hidden = false; out.className = "out"; out.textContent = "Compiling on the Rust Playground...";
      window.RobbieTA && RobbieTA.emote("look");
      try {
        const r = await runOnPlayground(raw);
        if (r.error) { out.className = "out err"; out.textContent = r.error; window.RobbieTA && RobbieTA.react(false, "The compiler left you a note. Read it top to bottom: it usually says exactly what to fix."); }
        else { out.className = "out ok"; out.textContent = (r.result || "(no output)").replace(/\s+$/, ""); window.RobbieTA && RobbieTA.emote("happy"); }
      } catch (e) {
        out.className = "out"; out.innerHTML = `This page couldn't reach the Playground directly. <a href="${playgroundLink(raw)}" target="_blank" rel="noopener">Open this code in the Playground</a> and press Run there.`;
      }
    });
  });

  // ---------- hero video: <figure class="hero-img" data-video="..."> plays a muted loop over the still ----------
  const reduceMotion = matchMedia("(prefers-reduced-motion: reduce)").matches;
  document.querySelectorAll(".hero-img[data-video]").forEach(fig => {
    if (reduceMotion) return;
    const still = fig.querySelector("img");
    const v = document.createElement("video");
    v.muted = true; v.loop = true; v.autoplay = true; v.playsInline = true; v.preload = "metadata";
    v.setAttribute("aria-hidden", "true"); if (still) v.poster = still.currentSrc || still.src;
    v.src = fig.dataset.video; v.style.opacity = "0"; v.style.transition = "opacity .8s";
    v.addEventListener("canplay", () => { v.style.opacity = "1"; v.play().catch(() => {}); }, { once: true });
    v.addEventListener("error", () => v.remove(), { once: true });
    fig.insertBefore(v, fig.querySelector("figcaption"));
  });

  // ---------- scroll reveal (runs after the page's own scripts have drawn their cards) ----------
  function setupReveal() {
    if (reduceMotion || !("IntersectionObserver" in window)) return;
    const els = document.querySelectorAll("main h2, .sess, .mat, .rust-strip figure, .how > div, .spec, .callout, .q, .tablewrap");
    const io = new IntersectionObserver(entries => entries.forEach(en => { if (en.isIntersecting) { en.target.classList.add("in"); io.unobserve(en.target); } }), { rootMargin: "0px 0px -8% 0px" });
    els.forEach((el, i) => {
      if (el.closest(".slide")) return;
      const r = el.getBoundingClientRect(); if (r.top < innerHeight) return; // already on screen: leave it alone
      el.classList.add("reveal"); el.style.transitionDelay = (i % 3) * 70 + "ms"; io.observe(el);
    });
  }
  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", setupReveal); else setTimeout(setupReveal, 0);

  // ---------- mark page visits ----------
  const pageKey = document.body.dataset.page;
  if (pageKey) RUSTUP_PROGRESS.mark("seen:" + pageKey, Date.now());
})();
