/**
 * Minimal frontend bootstrap for Rust Pool Sim (WASM).
 *
 * Expects `wasm-pack build --target web --out-dir pkg` to have been run at
 * the workspace root so that `../pkg/rust_pool_sim.js` and the associated
 * `.wasm` file are available.
 */

import init, { new_game_state_single_ball, tick } from "../pkg/rust_pool_sim.js";

(async () => {
  try {
    await init();

    const canvas = document.getElementById("pool-canvas");
    if (!(canvas instanceof HTMLCanvasElement)) {
      console.error("Canvas element with id 'pool-canvas' not found or invalid.");
      return;
    }

    const ctx = canvas.getContext("2d");
    if (!ctx) {
      console.error("Failed to acquire 2D rendering context.");
      return;
    }

    let state = new_game_state_single_ball();
    let lastTime = performance.now();

    function renderFrame(time) {
      const dtMs = time - lastTime;
      lastTime = time;

      // Convert to seconds; clamp to avoid large jumps on tab switch.
      const dt = Math.min(dtMs / 1000, 0.05);

      tick(state, dt);

      const width = canvas.width;
      const height = canvas.height;

      ctx.clearRect(0, 0, width, height);

      // For now we draw only the first ball if present.
      const ballsLen = typeof state.balls_len === "function" ? state.balls_len() : 0;

      if (ballsLen > 0 && typeof state.ball === "function") {
        const ball = state.ball(0);
        const x = typeof ball.x === "function" ? ball.x() : ball.x;
        const y = typeof ball.y === "function" ? ball.y() : ball.y;
        const r = typeof ball.radius === "function" ? ball.radius() : ball.radius;

        if (
          Number.isFinite(x) &&
          Number.isFinite(y) &&
          Number.isFinite(r) &&
          r > 0
        ) {
          ctx.save();
          ctx.fillStyle = "#2ee6a9";
          ctx.beginPath();
          ctx.arc(x, y, r, 0, Math.PI * 2);
          ctx.fill();
          ctx.restore();
        }
      }

      window.requestAnimationFrame(renderFrame);
    }

    window.requestAnimationFrame(renderFrame);
  } catch (err) {
    console.error("WASM initialization failed", err);
    console.error(
      "Suggested action: revert to last known good commit and rerun scripts/validate_setup.sh"
    );
  }
})();