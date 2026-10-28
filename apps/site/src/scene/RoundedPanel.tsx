import { useEffect, useMemo, type ReactNode } from "react";
import { RoundedBoxGeometry } from "three/examples/jsm/geometries/RoundedBoxGeometry.js";
import type { ThreeElements } from "@react-three/fiber";

type Props = Omit<ThreeElements["mesh"], "args"> & {
  args: [number, number, number];
  radius?: number;
  children?: ReactNode;
};

/** Procedural rounded panel: three's own RoundedBoxGeometry, no stock geometry assets. */
export function RoundedPanel({ args, radius = 0.1, children, ...rest }: Props) {
  const geometry = useMemo(() => new RoundedBoxGeometry(args[0], args[1], args[2], 3, radius), [args, radius]);
  useEffect(() => () => geometry.dispose(), [geometry]);
  return (
    <mesh {...rest} geometry={geometry}>
      {children}
    </mesh>
  );
}
