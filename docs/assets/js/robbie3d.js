// Robbie v11 in Three.js: cel-shaded (toon ramp + ink outline) and rusting by session.
// Loads assets/models/robbie.glb (exported from Kmk's robbie-v11.blend with its own animations).
import * as THREE from "three";
import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
import { MeshoptDecoder } from "three/addons/libs/meshopt_decoder.module.js";

const dock = document.querySelector(".robbie-dock");
const base = document.body.dataset.base || ".";
const rustLevel = Number(document.body.dataset.rust || 0); // 0..5
if (dock && window.WebGLRenderingContext) start().catch(err => console.warn("Robbie 3D unavailable, using the still render.", err));

// Flat anime colors keyed by the Blender material names.
const PALETTE = {
  armor_new: 0xeae3d1, armor_old: 0xd9cfb8, blockout: 0xd9d2c0,
  aluminum_dull: 0xa9aeb2, aluminum_silver: 0xc9ced2, chrome: 0xd8dee3, steel_brushed: 0x9aa1a8, steel_black: 0x2b2f34,
  gunmetal: 0x4b525a, black_matte: 0x1e2124, rubber: 0x26282b, circuit: 0x2f5a3e,
  brass: 0xcfa549, bronze: 0xb67a3c, bronze_collar: 0xc9893f, bronze_old: 0x8f6334, rivet_rust: 0x8a4a24,
  dial_face: 0xf0e6c8, cam_glow: 0xff9a3c, lens_glow: 0xffa34a, lens_glass: 0x1b1d22,
  glass_clear: 0x9fc3cf, glass_smoked_amber: 0x6b4a1e, glass_smoked_grey: 0x3a3f45, glass_tinted_red: 0xb3261e
};
const NO_RUST = /glass|glow|lens|rubber|circuit|dial_face|black_matte/;
const JOINT = /hub|rotor|piston|wrist|elbow|knee|shoulder|hip|ankle|neck|wheel|tread|track|gear|axle|drum|joint|bolt|rivet|sprocket|FAST_|reel|strut|calf/i;

function toonRamp() {
  const d = new Uint8Array([70, 70, 70, 255, 160, 160, 160, 255, 255, 255, 255, 255]);
  const t = new THREE.DataTexture(d, 3, 1, THREE.RGBAFormat);
  t.minFilter = t.magFilter = THREE.NearestFilter; t.needsUpdate = true; return t;
}

const NOISE = `
float rh(vec3 p){ p = fract(p*0.3183099 + .1); p *= 17.0; return fract(p.x*p.y*p.z*(p.x+p.y+p.z)); }
float rn(vec3 x){ vec3 i=floor(x); vec3 f=fract(x); f=f*f*(3.0-2.0*f);
  return mix(mix(mix(rh(i+vec3(0,0,0)),rh(i+vec3(1,0,0)),f.x),mix(rh(i+vec3(0,1,0)),rh(i+vec3(1,1,0)),f.x),f.y),
             mix(mix(rh(i+vec3(0,0,1)),rh(i+vec3(1,0,1)),f.x),mix(rh(i+vec3(0,1,1)),rh(i+vec3(1,1,1)),f.x),f.y),f.z); }
float rfbm(vec3 p){ float a=.5, s=0.; for(int i=0;i<4;i++){ s+=a*rn(p); p*=2.03; a*=.5; } return s; }`;

// Rust schedule, matched to the Blender stills: share of the metal that has rusted at each session.
//   S1 0% · S2 15% · S3 32% · S4 50% · S5 72% (first armor falls off) · S6 95% (armor gone, chest silver hanging on, askew)
// Each threshold is the rust score above which a surface has turned. Scores were calibrated against this model,
// weighted by surface area, so the percentages hold. Rust starts at joints, on rain-wet upward faces, and in drip streaks.
const RUST_THRESHOLD = [99, 0.840, 0.731, 0.652, 0.570, 0.439];
const DROP = { 4: ["LEG_kneecap.R", "HAND_back.L"],
  5: ["LEG_kneecap.R", "HAND_back.L", "ARM_pauldron.L", "ARM_forearm_shell.R", "LEG_thigh_shell.L", "CHEST_side_plates", "LEG_shin_shell.R", "HEAD_visor_back"] };
const SHINY = /^CHEST_box_cap|^CHEST_button/;

function rustify(mat, level, jointBoost) {
  const thr = RUST_THRESHOLD[level] ?? 99;
  mat.onBeforeCompile = shader => {
    shader.uniforms.uThr = { value: thr };
    shader.uniforms.uJoint = { value: jointBoost };
    shader.uniforms.uLevel = { value: level };
    shader.vertexShader = shader.vertexShader
      .replace("#include <common>", "#include <common>\nvarying vec3 vRP;\nvarying vec3 vWN;")
      .replace("#include <begin_vertex>", "#include <begin_vertex>\nvRP = (modelMatrix * vec4(position,1.0)).xyz;\nvWN = normalize(mat3(modelMatrix) * objectNormal);");
    shader.fragmentShader = shader.fragmentShader
      .replace("#include <common>", "#include <common>\nvarying vec3 vRP;\nvarying vec3 vWN;\nuniform float uThr;\nuniform float uJoint;\nuniform float uLevel;" + NOISE)
      .replace("#include <color_fragment>", `#include <color_fragment>
        if (uThr < 5.0) {
          vec3 p = vRP * 4.0;
          float drip = rfbm(vec3(vRP.x * 26.0, vRP.y * 3.0, vRP.z * 26.0));
          float score = rfbm(p) + clamp(vWN.y, 0.0, 1.0) * 0.24 + uJoint + drip * 0.18;
          float rust = step(uThr, score);
          float stain = step(uThr - 0.09, score) * (1.0 - rust);
          // three flat cel tones of rust, picked by big blotches plus fine grain, so it reads as anime paint
          float v = rfbm(p * 1.7 + 3.1) * 0.7 + rn(vRP * 60.0) * 0.3;
          vec3 rustCol = v < 0.40 ? vec3(0.07, 0.02, 0.008) : (v < 0.52 ? vec3(0.20, 0.055, 0.015) : vec3(0.45, 0.13, 0.03));
          diffuseColor.rgb *= mix(vec3(1.0), vec3(0.92, 0.84, 0.74), uLevel / 5.0);
          diffuseColor.rgb = mix(diffuseColor.rgb, diffuseColor.rgb * vec3(0.78, 0.60, 0.45), stain);
          diffuseColor.rgb = mix(diffuseColor.rgb, rustCol, rust);
        }`);
  };
  mat.customProgramCacheKey = () => "rust" + thr.toFixed(3) + jointBoost.toFixed(2);
}

function outlineMaterial(thickness) {
  const m = new THREE.MeshBasicMaterial({ color: 0x14161a, side: THREE.BackSide });
  m.onBeforeCompile = s => {
    s.vertexShader = s.vertexShader.replace("#include <begin_vertex>", `#include <begin_vertex>\ntransformed += normalize(normal) * ${thickness.toFixed(4)};`);
  };
  return m;
}

async function start() {
  const W = () => dock.clientWidth || 150, H = () => dock.clientHeight || 190;
  const renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true, powerPreference: "low-power" });
  renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
  renderer.setSize(W(), H());
  renderer.outputColorSpace = THREE.SRGBColorSpace;
  const canvas = renderer.domElement;
  canvas.setAttribute("role", "button"); canvas.setAttribute("tabindex", "0");
  canvas.setAttribute("aria-label", "Robbie, the course robot. Tap for help.");

  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(26, W() / H(), 0.05, 50);
  scene.add(new THREE.HemisphereLight(0xffffff, 0x445566, 1.2));
  const key = new THREE.DirectionalLight(0xffffff, 2.4); key.position.set(2.5, 4, 3.5); scene.add(key);
  const rim = new THREE.DirectionalLight(0x9fe8ff, 0.8); rim.position.set(-3, 2, -2); scene.add(rim);

  const loader = new GLTFLoader().setMeshoptDecoder(MeshoptDecoder);
  let gltf;
  try { gltf = await loader.loadAsync(`${base}/assets/models/robbie.glb`); }
  catch (err) {
    // Hosts that won't serve .glb files (like a private preview) get the same model packed as a base64 module.
    const mod = await import(new URL(`${base}/assets/models/robbie-glb.js`, location.href).href);
    const bin = Uint8Array.from(atob(mod.default), c => c.charCodeAt(0));
    gltf = await loader.parseAsync(bin.buffer, "");
  }
  const model = gltf.scene;
  const ramp = toonRamp();
  const level = Math.max(0, Math.min(5, Math.round(rustLevel)));
  const matCache = new Map();
  const outlines = [];
  // armor that has rusted clean off by this session
  // GLTFLoader strips dots from node names ("LEG_kneecap.R" becomes "LEG_kneecapR"), so look up both spellings
  const part = n => model.getObjectByName(n) || model.getObjectByName(n.replace(/[\[\]\.:\/]/g, ""));
  (DROP[level] || []).forEach(n => { const p = part(n); if (p) p.visible = false; });
  // Session 6: the chest silver survives, but it's been knocked loose
  if (level >= 5) {
    const cap = part("CHEST_box_cap"); if (cap) { cap.rotateX(0.24); cap.rotateY(-0.16); }
    const flap = part("CHEST_flap"); if (flap) flap.rotateX(-0.66);
  }
  model.traverse(o => {
    if (!o.isMesh) return;
    const src = (Array.isArray(o.material) ? o.material[0] : o.material);
    const name = (src && src.name || "").replace(/^G_/, "").replace(/\.\d+$/, "");
    const node = (/^Mesh|_\d+$/.test(o.name) && o.parent) ? o.parent.name : o.name;
    const joint = SHINY.test(node) ? -5 : (JOINT.test(node) || JOINT.test(o.name) ? 0.16 : 0.0);
    const exempt = NO_RUST.test(name);
    const k = name + "|" + joint + "|" + exempt;
    let m = matCache.get(k);
    if (!m) {
      const col = PALETTE[name] != null ? PALETTE[name] : (src && src.color ? src.color.getHex() : 0xdddddd);
      m = new THREE.MeshToonMaterial({ color: col, gradientMap: ramp });
      if (/glow/.test(name)) { m.emissive = new THREE.Color(col); m.emissiveIntensity = 0.9; }
      if (!exempt) rustify(m, level, joint);
      matCache.set(k, m);
    }
    o.material = m;
    o.castShadow = false;
    if (!o.isSkinnedMesh) outlines.push(o);
  });
  const ol = outlineMaterial(0.006);
  outlines.forEach(o => { const s = new THREE.Mesh(o.geometry, ol); s.renderOrder = -1; o.add(s); });
  scene.add(model);

  // frame the full body, looking slightly up like a little desk buddy
  const box = new THREE.Box3().setFromObject(model);
  const size = box.getSize(new THREE.Vector3()), ctr = box.getCenter(new THREE.Vector3());
  const dist = (size.y * 0.62) / Math.tan(THREE.MathUtils.degToRad(camera.fov / 2));
  camera.position.set(ctr.x + dist * 0.38, ctr.y + size.y * 0.04, ctr.z + dist * 0.92);
  camera.lookAt(ctr.x, ctr.y - size.y * 0.02, ctr.z);
  model.rotation.y = -0.25;

  // NPC idle: hang the model from a pivot at his feet so the whole body can sway, bob and turn toward the pointer
  const rig = new THREE.Group();
  rig.position.set(ctr.x, box.min.y, ctr.z);
  model.position.set(-ctr.x, -box.min.y, -ctr.z);
  scene.remove(model); rig.add(model); scene.add(rig);
  const baseY = rig.position.y, H0 = size.y;
  let lookYaw = 0, lookPitch = 0, aimYaw = 0, aimPitch = 0, lastMove = 0;
  window.addEventListener("pointermove", e => {
    const r = canvas.getBoundingClientRect();
    const dx = (e.clientX - (r.left + r.width / 2)) / innerWidth, dy = (e.clientY - (r.top + r.height * 0.25)) / innerHeight;
    aimYaw = THREE.MathUtils.clamp(dx * 0.9, -0.45, 0.25); aimPitch = THREE.MathUtils.clamp(dy * 0.25, -0.08, 0.1);
    lastMove = performance.now();
  }, { passive: true });

  // animations exported from the .blend
  const mixer = new THREE.AnimationMixer(model);
  const clips = {}; gltf.animations.forEach(c => clips[c.name.replace(/^RO_/, "")] = c);
  let current = null;
  function play(name, once) {
    const clip = clips[name]; if (!clip) return false;
    const a = mixer.clipAction(clip);
    a.reset(); a.setLoop(once ? THREE.LoopOnce : THREE.LoopRepeat); a.clampWhenFinished = true;
    if (current && current !== a) a.crossFadeFrom(current, 0.35, false);
    a.play(); current = a; return true;
  }
  mixer.addEventListener("finished", () => play("idle"));
  play("idle");
  const EMOTES = { wave: "wave", happy: "jump", dance: "dance", think: "personality", look: "look_around", spin: "turn_in_place" };

  const fallback = dock.querySelector(".fallback");
  dock.insertBefore(canvas, fallback); fallback.remove();
  canvas.addEventListener("keydown", e => { if (e.key === "Enter" || e.key === " ") { e.preventDefault(); window.RobbieTA && RobbieTA.greet(); } });

  window.RobbieTA && (RobbieTA.body = {
    play(n) { if (!play(EMOTES[n] || n, n !== "dance")) play("idle"); if (n === "dance") setTimeout(() => play("idle"), 6000); },
    clips: Object.keys(clips)
  });

  // gentle life: glance around now and then
  setInterval(() => { if (current === mixer.clipAction(clips.idle) && Math.random() < 0.35) play("look_around", true); }, 14000);

  const clock = new THREE.Clock();
  let visible = true;
  document.addEventListener("visibilitychange", () => { visible = !document.hidden; });
  window.addEventListener("resize", () => { renderer.setSize(W(), H()); camera.aspect = W() / H(); camera.updateProjectionMatrix(); });
  const reduce = matchMedia("(prefers-reduced-motion: reduce)").matches;
  renderer.setAnimationLoop(() => {
    if (!visible) return;
    const dt = Math.min(clock.getDelta(), 0.05);
    mixer.update(reduce ? 0 : dt);
    if (!reduce) {
      const t = clock.elapsedTime;
      // drift back to centre when the pointer has been still for a few seconds
      if (performance.now() - lastMove > 4000) { aimYaw *= 0.98; aimPitch *= 0.98; }
      lookYaw += (aimYaw - lookYaw) * Math.min(1, dt * 2.2);
      lookPitch += (aimPitch - lookPitch) * Math.min(1, dt * 2.2);
      rig.rotation.z = Math.sin(t * 0.85) * 0.028 + Math.sin(t * 0.31 + 1.7) * 0.014;
      rig.rotation.x = Math.sin(t * 0.6 + 0.4) * 0.012 + lookPitch;
      rig.rotation.y = Math.sin(t * 0.22) * 0.07 + lookYaw;
      rig.position.y = baseY + (Math.sin(t * 1.7) * 0.5 + 0.5) * H0 * 0.006;
    }
    renderer.render(scene, camera);
  });
}
