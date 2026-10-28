import { useEffect, useMemo, type ReactNode } from "react";
import {
  BufferGeometry,
  LineBasicMaterial,
  LineLoop,
  Shape,
  Vector3,
} from "three";
import { RoundedBoxGeometry } from "three/examples/jsm/geometries/RoundedBoxGeometry.js";
import type { ThreeElements } from "@react-three/fiber";

type Props = Omit<ThreeElements["mesh"], "args"> & {
  args: [number, number, number];
  radius?: number;
  children?: ReactNode;
  edgeColor?: string;
  edgeOpacity?: number;
};

function roundedRect(width: number, height: number, radius: number): Shape {
  const x = -width / 2;
  const y = -height / 2;
  const r = Math.min(radius, width / 2, height / 2);
  const shape = new Shape();
  shape.moveTo(x + r, y);
  shape.lineTo(x + width - r, y);
  shape.quadraticCurveTo(x + width, y, x + width, y + r);
  shape.lineTo(x + width, y + height - r);
  shape.quadraticCurveTo(x + width, y + height, x + width - r, y + height);
  shape.lineTo(x + r, y + height);
  shape.quadraticCurveTo(x, y + height, x, y + height - r);
  shape.lineTo(x, y + r);
  shape.quadraticCurveTo(x, y, x + r, y);
  return shape;
}

/** Procedural rounded glass panel with a crisp vector outline. */
export function RoundedPanel({
  args,
  radius = 0.1,
  edgeColor = "#a8896f",
  edgeOpacity = 0.5,
  children,
  ...rest
}: Props) {
  const geometry = useMemo(
    () => new RoundedBoxGeometry(args[0], args[1], args[2], 3, radius),
    [args, radius],
  );
  const outline = useMemo(() => {
    const points = roundedRect(args[0] - 0.006, args[1] - 0.006, radius).getPoints(10);
    const vector = points.map((point) => new Vector3(point.x, point.y, args[2] / 2 + 0.03));
    const buffer = new BufferGeometry().setFromPoints(vector);
    const material = new LineBasicMaterial({ color: edgeColor, transparent: true, opacity: edgeOpacity, toneMapped: false });
    return new LineLoop(buffer, material);
  }, [args, radius, edgeColor, edgeOpacity]);

  useEffect(() => () => {
    geometry.dispose();
    outline.geometry.dispose();
    outline.material.dispose();
  }, [geometry, outline]);

  return (
    <mesh {...rest} geometry={geometry}>
      {children}
      <primitive object={outline} />
    </mesh>
  );
}
