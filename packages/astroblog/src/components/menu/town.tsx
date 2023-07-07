import * as town from "@nxblog/town/dist/target/wasm/town/town"
import { useEffect, useState } from 'react';

declare global {
  interface Window {
    go_town(s: string): void;
  }
}

const CANVAS_ID = "town-canvas";
export default () => {
  const [canvas, setCanvas] = useState<HTMLCanvasElement | null>(null);
  useEffect(() => {
    if (!canvas) return;
    window.go_town = (s) => {
      window.open(s, '_blank');
    }
    try {
      setTimeout(() => {
        // I don't know why, lordy lord save us; probably it overrides the whole canvas
        canvas.focus();
      });
      town.greet(canvas.id);
    } catch (e) {
      console.error(e);
    }
  }, [canvas]);
  return (
    <div className="town-menu-base">
      <canvas id={CANVAS_ID} ref={setCanvas}>
      </canvas>
    </div>
  );
}
