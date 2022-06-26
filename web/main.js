import init, { start } from "./egui_experiments.js";
try {
  await init();
} catch {
  console.error("Failed to start: " + error);
  document.getElementById("center_text").innerHTML = `
                <p>
                    An error occurred during loading:
                </p>
                <p style="font-family:Courier New">
                    ${error}
                </p>
                <p style="font-size:14px">
                    Make sure you use a modern browser with WebGL and WASM enabled.
                </p>`;
} finally {
  console.debug("wasm loaded. starting app…");
  start("the_canvas_id");
  console.debug("app started.");
  document.getElementById("center_text").remove();
}
