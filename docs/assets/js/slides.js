// Slide deck: arrow keys or buttons move, N toggles speaker notes, the URL hash remembers the slide.
(function () {
  const slides = [...document.querySelectorAll(".slide")];
  if (!slides.length) return;
  const nav = document.querySelector(".deck-nav");
  const bar = nav.querySelector(".prog i"), count = nav.querySelector(".count");
  let i = 0;
  function show(n, fromHash) {
    i = Math.max(0, Math.min(slides.length - 1, n));
    slides.forEach((s, k) => s.classList.toggle("on", k === i));
    bar.style.width = ((i + 1) / slides.length * 100) + "%";
    count.textContent = `${i + 1} / ${slides.length}`;
    if (!fromHash) history.replaceState(null, "", "#slide-" + (i + 1));
    const s = slides[i];
    if (s.dataset.robbie && window.RobbieTA) RobbieTA.say(s.dataset.robbie, { emote: s.dataset.emote || "think", ttl: 9000 });
    window.scrollTo({ top: 0 });
  }
  nav.querySelector(".prev").onclick = () => show(i - 1);
  nav.querySelector(".next").onclick = () => show(i + 1);
  nav.querySelector(".notes").onclick = () => document.body.classList.toggle("show-notes");
  document.addEventListener("keydown", e => {
    if (e.target.closest("input, textarea")) return;
    if (e.key === "ArrowRight" || e.key === "PageDown") show(i + 1);
    if (e.key === "ArrowLeft" || e.key === "PageUp") show(i - 1);
    if (e.key === "n" || e.key === "N") document.body.classList.toggle("show-notes");
  });
  const m = location.hash.match(/slide-(\d+)/);
  show(m ? +m[1] - 1 : 0, true);
  window.addEventListener("hashchange", () => { const m = location.hash.match(/slide-(\d+)/); if (m) show(+m[1] - 1, true); });
})();
