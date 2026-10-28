import { useEffect, useEffectEvent, useRef } from 'react';
import { LAYERS, type Layer } from '../data/layers';
import { createStudio, spring } from './studio';

type Props = {
  stageEl: HTMLElement | null;
  labels: { current: (HTMLDivElement | null)[] };
  onHover: (layer: Layer | null) => void;
  onReady: () => void;
  onFailure: () => void;
  quality: 'full' | 'light';
};
export default function HeroScene({ stageEl, labels, onHover, onReady, onFailure, quality }: Props) {
  const canvas = useRef<HTMLCanvasElement>(null);
  const ready = useEffectEvent(onReady);
  const failed = useEffectEvent(onFailure);
  const hover = useEffectEvent(onHover);
  const placeLabels = useEffectEvent((points: { x: number; y: number; opacity: number }[]) => {
    points.forEach((point, index) => {
      const label = labels.current[index];
      if (label) { label.style.transform = `translate(${point.x}px, ${point.y}px) translateY(-50%)`; label.style.opacity = String(point.opacity); }
    });
  });
  useEffect(() => {
    const element = canvas.current;
    if (!element || !stageEl) return;
    let cancelled = false, animation = 0;
    let studio: Awaited<ReturnType<typeof createStudio>> | undefined;
    const desired = [0, 0], pointer: [number, number] = [0, 0], velocity = [0, 0];
    let active = -1;
    const move = (event: PointerEvent) => {
      const bounds = element.getBoundingClientRect();
      desired[0] = Math.max(-1, Math.min(1, (event.clientX - bounds.left) / bounds.width * 2 - 1));
      desired[1] = Math.max(-1, Math.min(1, (event.clientY - bounds.top) / bounds.height * 2 - 1));
      const next = studio?.hit(desired[0], -desired[1]) ?? -1;
      if (next !== active) { active = next; hover(LAYERS[active] ?? null); }
    };
    const leave = () => { desired.fill(0); active = -1; hover(null); };
    const lost = () => failed();
    const observer = new ResizeObserver(() => studio?.resize(stageEl.clientWidth, stageEl.clientHeight));
    observer.observe(stageEl);
    element.addEventListener('pointermove', move); element.addEventListener('pointerleave', leave); element.addEventListener('webglcontextlost', lost);
    void createStudio(element, { width: stageEl.clientWidth, height: stageEl.clientHeight, dpr: quality === 'full' ? Math.min(devicePixelRatio, 2) : 1, light: quality === 'light' }).then((created) => {
      if (cancelled) { created.dispose(); return; }
      studio = created;
      const start = performance.now(); let previous = start;
      const frame = (now: number) => {
        if (cancelled) return;
        const seconds = (now - start) / 1000, dt = Math.min((now - previous) / 1000, 0.03); previous = now;
        for (let i = 0; i < 2; i++) { velocity[i] += ((desired[i] - pointer[i]) * 120 - velocity[i] * 18) * dt; pointer[i] += velocity[i] * dt; }
        const scroll = Math.max(0, Math.min(1, -stageEl.getBoundingClientRect().top / Math.max(1, stageEl.clientHeight)));
        const spread = Math.max(0, Math.min(1, spring(seconds - 0.45) * 0.7 + scroll * 0.3));
        placeLabels(created.render({ seconds, spread, intro: Math.min(1, seconds / 2.6), pointer, active: active < 0 ? undefined : active }));
        if (seconds < 0.1) ready();
        animation = requestAnimationFrame(frame);
      };
      animation = requestAnimationFrame(frame);
    }).catch(() => { if (!cancelled) failed(); });
    return () => {
      cancelled = true; cancelAnimationFrame(animation); observer.disconnect();
      element.removeEventListener('pointermove', move); element.removeEventListener('pointerleave', leave); element.removeEventListener('webglcontextlost', lost);
      studio?.dispose();
    };
  }, [quality, stageEl]);
  return <canvas ref={canvas} role="img" aria-label="Five opaque screens from OriginLM in example mode: your app, SDK, transport, runtime and providers." />;
}
