import * as town from "@nxblog/town/dist/target/wasm/town/town"
import { useEffect, useMemo, useState } from 'react';

declare global {
  interface Window {
    go_town(s: string): void;
  }
}

const getCanvasPositionRelativeToTheWholeScreen = (canvas: HTMLCanvasElement) => {
  const rect = canvas.getBoundingClientRect();
  return {
    x: rect.x,
    y: rect.y,
  };
};

// keep the state; update it on any resize or scroll or window move
const useCanvasPositionRelativeToTheWholeScreen = (canvas: HTMLCanvasElement | null) => {
  const [position, setPosition] = useState({ x: 0, y: 0 });
  useEffect(() => {
    if (!canvas) return;
    const updatePosition = () => setPosition(getCanvasPositionRelativeToTheWholeScreen(canvas));
    updatePosition();
    window.addEventListener("resize", updatePosition);
    window.document.addEventListener("scroll", updatePosition);
    window.addEventListener("touchmove", updatePosition);
    window.addEventListener("touchend", updatePosition);
    // and also poll it every 100 ms
    const interval = setInterval(updatePosition, 100);
    return () => {
      window.removeEventListener("resize", updatePosition);
      window.document.removeEventListener("scroll", updatePosition);
      window.removeEventListener("touchmove", updatePosition);
      window.removeEventListener("touchend", updatePosition);
      clearInterval(interval);
    };
  }, [canvas]);
  return useMemo(() => position, [position.x, position.y]);
}

const useFocusReporter = (canvas: HTMLCanvasElement | null) => {
  useEffect(() => {
    if (!canvas) return;

    const reportFocusOrBlur = (b: boolean) => () => {
      town.report_window_focus(b);
    }
    const reportFocus = reportFocusOrBlur(true)
    const reportBlur = reportFocusOrBlur(false)

    reportFocus();
    window.addEventListener("focus", reportFocus);
    window.addEventListener("blur", reportBlur);
    canvas.addEventListener("focus", reportFocus);
    canvas.addEventListener("blur", reportBlur);
    return () => {
      window.removeEventListener("focus", reportFocus);
      window.removeEventListener("blur", reportBlur);
      canvas.removeEventListener("focus", reportFocus);
      canvas.removeEventListener("blur", reportBlur);
    };
  }, [canvas]);
}
const CANVAS_ID = "town-canvas";
export default () => {
  const [canvas, setCanvas] = useState<HTMLCanvasElement | null>(null);
  const [hiddenLink, setHiddenLink] = useState<HTMLAnchorElement | null>(null);
  const position = useCanvasPositionRelativeToTheWholeScreen(canvas);
  useFocusReporter(canvas);
  useEffect(() => {
    if (!canvas) return;
    town.report_canvas_screen_position(position.x, position.y);
  });
  useEffect(() => {
    if (!canvas) return;
    if (!hiddenLink) return;
    window.go_town = (s) => {
      const fallback = (s: string) => {
        hiddenLink.setAttribute('href', s);
        window.confirm(`Open ${s}?`) && hiddenLink.click();
      };
      // TODO https://github.com/bevyengine/bevy/issues/2068 even this doesn't work
      town.report_window_focus(false);
      try {
        // TODO same page for now, to *hide* the issue https://github.com/bevyengine/bevy/issues/2068
        const w = window.open(s, '_self');
        if (w === null) fallback(s);
      } catch (error) {
        fallback(s);
      }
    }
    try {
      town.greet(canvas.id);
    } catch (error) {
      // really what the fuck
      if (!error.message.startsWith("Using exceptions for control flow,")) {
        throw error;
      }
    }
    canvas.focus();
  }, [canvas, hiddenLink]);
  return (
    <div className="town-menu-base">
      <canvas id={CANVAS_ID} ref={setCanvas}>
      </canvas>
      <a className="hidden-link" ref={setHiddenLink}></a>
    </div>
  );
}
