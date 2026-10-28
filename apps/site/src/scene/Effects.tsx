import { Bloom, DepthOfField, EffectComposer } from "@react-three/postprocessing";

/** Bloom plus a restrained depth of field over the glass stack. */
export function Effects({ enabled }: { enabled: boolean }) {
  if (!enabled) return null;
  return (
    <EffectComposer multisampling={0}>
      <Bloom intensity={0.35} luminanceThreshold={0.82} luminanceSmoothing={0.25} mipmapBlur />
      <DepthOfField focusDistance={0.2} focalLength={0.06} bokehScale={0.5} height={480} />
    </EffectComposer>
  );
}
