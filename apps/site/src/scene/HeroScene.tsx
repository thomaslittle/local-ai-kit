import { brand } from "../brand.generated";
import { useEffect, useMemo, useRef, useState } from "react";
import { Canvas, useFrame, useThree } from "@react-three/fiber";
import { Preload } from "@react-three/drei/core/Preload.js";
import { RoundedPanel } from "./RoundedPanel";
import { Effects } from "./Effects";
import {
  Color,
  Vector3,
  DoubleSide,
  Group,
  InstancedMesh,
  MathUtils,
  Object3D,
  PMREMGenerator,
} from "three";
import { RoomEnvironment } from "three/examples/jsm/environments/RoomEnvironment.js";
import { LAYERS, type Layer } from "../data/layers";

type SceneProps = {
  stageEl: HTMLElement | null;
  labels: { current: (HTMLDivElement | null)[] };
  onHover: (layer: Layer | null) => void;
  onReady: () => void;
  onFailure: () => void;
  quality: "full" | "light";
};


const clamp = (value: number, min = 0, max = 1) => Math.min(max, Math.max(min, value));
const smoothstep = (edge0: number, edge1: number, value: number) => {
  const t = clamp((value - edge0) / (edge1 - edge0));
  return t * t * (3 - 2 * t);
};

type LayerShape = {
  width: number;
  height: number;
  depth: number;
  assembled: [number, number, number, number];
  exploded: [number, number, number, number];
};

const SHAPES: Record<string, LayerShape> = {
  app: {
    width: 3.7,
    height: 1.0,
    depth: 0.14,
    assembled: [0, 0.18, 0.48, 1.02],
    exploded: [-1.2, 2.65, 0.2, 1],
  },
  dashboard: {
    width: 2.2,
    height: 1.1,
    depth: 0.12,
    assembled: [0.98, 0.55, 0.72, 0.5],
    exploded: [1.95, 2.55, 0.35, 0.9],
  },
  sdk: {
    width: 4.4,
    height: 0.8,
    depth: 0.12,
    assembled: [0, 0.08, 0.4, 0.94],
    exploded: [0, 1.45, 0.1, 1],
  },
  transport: {
    width: 4.7,
    height: 0.65,
    depth: 0.1,
    assembled: [0, 0.03, 0.32, 0.88],
    exploded: [0, 0.55, 0.05, 1],
  },
  runtime: {
    width: 5.1,
    height: 0.85,
    depth: 0.1,
    assembled: [0, -0.02, 0.24, 0.82],
    exploded: [0, -0.35, 0, 1],
  },
  adapters: {
    width: 5.4,
    height: 0.7,
    depth: 0.1,
    assembled: [0, -0.06, 0.16, 0.76],
    exploded: [0, -1.35, 0, 1],
  },
  providers: {
    width: 5.6,
    height: 0.95,
    depth: 0.1,
    assembled: [0, -0.1, 0.08, 0.7],
    exploded: [0, -2.45, 0, 1],
  },
};

function StudioEnvironment() {
  const { gl, scene } = useThree();
  useEffect(() => {
    const pmrem = new PMREMGenerator(gl);
    const room = new RoomEnvironment();
    const environment = pmrem.fromScene(room, 0.04);
    room.dispose();
    scene.environment = environment.texture;
    return () => {
      scene.environment = null;
      environment.dispose();
      pmrem.dispose();
    };
  }, [gl, scene]);
  return null;
}

function SpreadDriver({
  stageEl,
  spreadRef,
}: {
  stageEl: HTMLElement | null;
  spreadRef: { value: number };
}) {
  const velocity = useRef(0);
  const scroll = useRef(0);

  useFrame((state, rawDelta) => {
    const delta = Math.min(rawDelta, 1 / 20);
    const time = state.clock.elapsedTime;

    if (stageEl) {
      const rect = stageEl.getBoundingClientRect();
      const viewport = window.innerHeight || 800;
      const start = viewport * 0.62;
      const end = viewport * 0.02;
      const next = clamp((start - rect.top) / Math.max(1, start - end));
      scroll.current += (next - scroll.current) * Math.min(1, delta * 8);
    }

    const auto = smoothstep(1.2, 2.6, time) * (0.5 - 0.5 * Math.cos(Math.max(0, time - 1.2) * 0.34));
    const target = Math.max(scroll.current, 0.06 + auto * 0.86);

    velocity.current += (target - spreadRef.value) * 26 * delta;
    velocity.current *= Math.exp(-7.5 * delta);
    spreadRef.value += velocity.current * delta;
    if (spreadRef.value < -0.02) {
      spreadRef.value = -0.02;
      velocity.current = 0;
    }
    if (spreadRef.value > 1.08) {
      spreadRef.value = 1.08;
      velocity.current = 0;
    }
  });

  return null;
}

function CameraRig() {
  const { camera, size } = useThree();
  const target = useRef(14);

  useFrame((_, delta) => {
    const aspect = size.width / Math.max(1, size.height);
    target.current = aspect < 1.05 ? 16.4 : aspect < 1.5 ? 13.8 : 12.4;
    camera.position.z += (target.current - camera.position.z) * Math.min(1, delta * 3);
    camera.lookAt(0, 0, 0);
  });

  return null;
}

function PacketSystem({ spreadRef }: { spreadRef: { value: number } }) {
  const mesh = useRef<InstancedMesh>(null);
  const count = 16;
  const dummy = useMemo(() => new Object3D(), []);
  const colors = useMemo(() => [new Color(brand.amberHover), new Color(brand.amber), new Color(brand.sceneBronze)], []);

  useFrame((state) => {
    if (!mesh.current) return;
    const spread = spreadRef.value;
    const top = MathUtils.lerp(0.4, 2.7, spread);
    const bottom = MathUtils.lerp(-0.2, -2.7, spread);
    const time = state.clock.elapsedTime;

    for (let index = 0; index < count; index += 1) {
      const phase = (time * 0.075 + index / count) % 1;
      const travel = 1 - Math.abs(phase * 2 - 1);
      const y = MathUtils.lerp(bottom, top, travel);
      const wobble = Math.sin(time * 0.7 + index * 2.1) * 0.28;
      dummy.position.set(wobble, y, 0.32 + Math.sin(index * 3.1) * 0.12);
      const scale = 0.018 + Math.sin(phase * Math.PI) * 0.018;
      dummy.scale.setScalar(scale);
      dummy.updateMatrix();
      mesh.current.setMatrixAt(index, dummy.matrix);
      mesh.current.setColorAt(index, colors[index % colors.length]);
    }
    mesh.current.instanceMatrix.needsUpdate = true;
    if (mesh.current.instanceColor) mesh.current.instanceColor.needsUpdate = true;
  });

  return (
    <instancedMesh ref={mesh} args={[undefined, undefined, count]} frustumCulled={false}>
      <sphereGeometry args={[1, 8, 8]} />
      <meshBasicMaterial toneMapped={false} transparent opacity={0.8} />
    </instancedMesh>
  );
}

function ProviderNodes({ spreadRef }: { spreadRef: { value: number } }) {
  const group = useRef<Group>(null);
  const nodes = useMemo(
    () => [
      { x: -2.55, tone: brand.amber },
      { x: -1.7, tone: brand.amberHover },
      { x: -0.85, tone: brand.sceneSage },
      { x: 0, tone: brand.amber },
      { x: 0.85, tone: brand.sceneTerracotta },
      { x: 1.7, tone: brand.sceneSage },
      { x: 2.55, tone: brand.amberHover },
    ],
    [],
  );

  useFrame((state) => {
    if (!group.current) return;
    group.current.children.forEach((child, index) => {
      const pulse = 1 + Math.sin(state.clock.elapsedTime * 1.4 + index * 1.3) * 0.12;
      child.scale.setScalar(pulse * MathUtils.lerp(0.55, 1, spreadRef.value));
    });
  });

  return (
    <group ref={group} position={[0, 0, 0.16]}>
      {nodes.map((node, index) => (
        <mesh key={node.x} position={[node.x, index % 2 === 0 ? -0.1 : 0.14, 0]}>
          <sphereGeometry args={[0.1, 16, 16]} />
          <meshBasicMaterial color={node.tone} toneMapped={false} />
        </mesh>
      ))}
    </group>
  );
}

function LayerDetail({ layer, width, height, depth }: { layer: Layer; width: number; height: number; depth: number }) {
  const z = depth / 2 + 0.004;
  const tone = layer.id === "dashboard" ? brand.amberHover : brand.sceneText;

  if (layer.id === "providers") return null;

  if (layer.id === "dashboard") {
    return (
      <group position={[0, 0, z]}>
        {[0.42, 0.62, 0.34, 0.72, 0.5, 0.82].map((value, index) => (
          <mesh key={index} position={[-0.72 + index * 0.24, -0.45 + value * 0.22, 0]}>
            <planeGeometry args={[0.12, value * 0.42]} />
            <meshBasicMaterial color={index % 2 ? brand.sceneSage : tone} transparent opacity={0.7} toneMapped={false} />
          </mesh>
        ))}
        <mesh position={[0.1, 0.28, 0]}>
          <planeGeometry args={[1.5, 0.035]} />
          <meshBasicMaterial color={brand.paper} transparent opacity={0.18} toneMapped={false} />
        </mesh>
        <mesh position={[-0.18, 0.12, 0]}>
          <planeGeometry args={[1.0, 0.035]} />
          <meshBasicMaterial color={brand.paper} transparent opacity={0.12} toneMapped={false} />
        </mesh>
      </group>
    );
  }

  const rows = layer.id === "app" ? 2 : 3;
  return (
    <group position={[-width / 2 + 0.42, 0, z]}>
      {layer.id === "app" &&
        [0, 1, 2].map((dot) => (
          <mesh key={`dot-${dot}`} position={[0.12 + dot * 0.16, height / 2 - 0.22, 0]}>
            <circleGeometry args={[0.035, 12]} />
            <meshBasicMaterial color={dot === 0 ? brand.amberHover : brand.sceneMuted} toneMapped={false} />
          </mesh>
        ))}
      {Array.from({ length: rows }).map((_, index) => (
        <mesh key={index} position={[0.5 + (index % 2) * 0.5, 0.12 - index * 0.22, 0]}>
          <planeGeometry args={[0.7 + (index % 3) * 0.3, 0.045]} />
          <meshBasicMaterial color={index === 0 ? tone : brand.paper} transparent opacity={index === 0 ? 0.55 : 0.14} toneMapped={false} />
        </mesh>
      ))}
    </group>
  );
}

function GlassLayer({
  layer,
  index,
  spreadRef,
  active,
  labels,
  onHover,
}: {
  layer: Layer;
  index: number;
  spreadRef: { value: number };
  active: string | null;
  labels: { current: (HTMLDivElement | null)[] };
  onHover: (layer: Layer | null) => void;
}) {
  const shape = SHAPES[layer.id];
  const group = useRef<Group>(null);
  const hoverLift = useRef(0);
  const { camera, size } = useThree();
  const projected = useMemo(() => new Vector3(), []);
  const isHovered = active === layer.id;
  const isDashboard = layer.id === "dashboard";
  const isProviders = layer.id === "providers";

  useFrame((_, rawDelta) => {
    if (!group.current) return;
    const delta = Math.min(rawDelta, 1 / 20);
    const spread = spreadRef.value;

    const [ax, ay, az, as] = shape.assembled;
    const [ex, ey, ez, es] = shape.exploded;
    const easing = smoothstep(0, 1, spread);

    const x = MathUtils.lerp(ax, ex, easing);
    const y = MathUtils.lerp(ay, ey, easing);
    const z = MathUtils.lerp(az, ez, easing);

    hoverLift.current += ((isHovered ? 0.22 : 0) - hoverLift.current) * Math.min(1, delta * 10);

    group.current.position.set(x, y, z + hoverLift.current);
    const scale = MathUtils.lerp(as, es, easing) * (isHovered ? 1.025 : 1);
    group.current.scale.setScalar(scale);
    group.current.rotation.z = isDashboard ? -0.02 : 0;
    group.current.rotation.y = isHovered ? 0.06 : 0;

    const label = labels.current[index];
    if (label && layer.label) {
      group.current.updateWorldMatrix(true, false);
      projected.set(shape.width / 2 + 0.22, 0, 0);
      group.current.localToWorld(projected);
      projected.project(camera);
      const x = size.width - (size.width < 640 ? 12 : 26);
      const y = (-projected.y * 0.5 + 0.5) * size.height;
      const opacity = smoothstep(0.16, 0.4, spread);
      label.style.opacity = String(opacity);
      label.style.transform = `translate3d(${x.toFixed(1)}px, ${y.toFixed(1)}px, 0) translate(-100%, -50%)`;
    }
  });

  return (
    <group
      ref={group}
      position={[shape.assembled[0], shape.assembled[1], shape.assembled[2]]}
      onClick={(event) => {
        event.stopPropagation();
        onHover(layer);
      }}
      onPointerOver={(event) => {
        if (event.pointerType === "mouse") {
          event.stopPropagation();
          onHover(layer);
        }
      }}
      onPointerOut={(event) => {
        if (event.pointerType === "mouse") onHover(null);
      }}
    >
      <RoundedPanel args={[shape.width, shape.height, shape.depth]} radius={0.11}>
        <meshPhysicalMaterial
          color={brand.sceneGlass}
          metalness={0.8}
          roughness={0.14}
          transparent
          opacity={isDashboard ? 0.66 : 0.58}
          clearcoat={1}
          clearcoatRoughness={0.06}
          envMapIntensity={0.9}
          emissive={isHovered ? brand.amber : brand.sceneEmissive}
          emissiveIntensity={isHovered ? 0.1 : 0.16}
          side={DoubleSide}
        />
      </RoundedPanel>

      <LayerDetail layer={layer} width={shape.width} height={shape.height} depth={shape.depth} />

      {isProviders && <ProviderNodes spreadRef={spreadRef} />}

      {!isProviders && (
        <mesh position={[0, -shape.height / 2 + 0.05, 0.02]}>
          <planeGeometry args={[shape.width * 0.86, 0.012]} />
          <meshBasicMaterial color={isDashboard ? brand.amberHover : brand.amber} transparent opacity={isDashboard ? 0.5 : 0.24} />
        </mesh>
      )}

    </group>
  );
}

function Rig({
  stageEl,
  labels,
  spreadRef,
  quality,
  onHover,
}: {
  stageEl: HTMLElement | null;
  labels: { current: (HTMLDivElement | null)[] };
  spreadRef: { value: number };
  quality: "full" | "light";
  onHover: (layer: Layer | null) => void;
}) {
  const group = useRef<Group>(null);
  const [active, setActive] = useState<string | null>(null);

  useFrame((state) => {
    if (!group.current) return;
    const time = state.clock.elapsedTime;
    group.current.rotation.y = -0.19 + Math.sin(time * 0.16) * 0.05;
    group.current.rotation.x = 0.055 + Math.sin(time * 0.21) * 0.012;
    group.current.position.x = -0.5;
    group.current.position.y = Math.sin(time * 0.3) * 0.035;
  });

  const handleHover = (layer: Layer | null) => {
    setActive(layer?.id ?? null);
    onHover(layer);
  };

  return (
    <>
      <SpreadDriver stageEl={stageEl} spreadRef={spreadRef} />
      <CameraRig />
      <StudioEnvironment />
      <group ref={group}>
        {LAYERS.map((layer, index) => (
          <GlassLayer
            key={layer.id}
            layer={layer}
            index={index}
            spreadRef={spreadRef}
            active={active}
            labels={labels}
            onHover={handleHover}
          />
        ))}
        <PacketSystem spreadRef={spreadRef} />
      </group>
      <Preload all />
      <Effects enabled={quality === "full"} />
    </>
  );
}

function ContextLoss({ onFailure }: { onFailure: () => void }) {
  const { gl } = useThree();
  useEffect(() => {
    const canvas = gl.domElement;
    canvas.addEventListener("webglcontextlost", onFailure);
    return () => canvas.removeEventListener("webglcontextlost", onFailure);
  }, [gl, onFailure]);
  return null;
}

export default function HeroScene({ stageEl, labels, onHover, onReady, onFailure, quality }: SceneProps) {
  const spreadRef = useMemo(() => ({ value: 0 }), []);

  return (
    <Canvas
      dpr={[1, quality === "full" ? 1.75 : 1.4]}
      gl={{ antialias: true, alpha: true, powerPreference: "high-performance" }}
      camera={{ position: [0, 0, 14], fov: 30, near: 0.1, far: 60 }}
      onCreated={({ gl }) => {
        gl.setClearColor(0x000000, 0);
        onReady();
      }}
      onPointerMissed={() => onHover(null)}
      aria-label="Exploded 3D view of the OriginLM stack: your app and the dashboard on top, then the SDK, transport, local runtime, adapters and the AI providers already installed."
      role="img"
    >
      <ContextLoss onFailure={onFailure} />
      <ambientLight intensity={0.9} />
      <directionalLight position={[5, 8, 8]} intensity={1.6} color={brand.sceneLight} />
      <directionalLight position={[-6, 2, 4]} intensity={1.1} color={brand.sceneSage} />
      <pointLight position={[0, -5, 4]} intensity={18} color={brand.amber} distance={16} />
      <pointLight position={[5, 2, 6]} intensity={9} color={brand.sceneLight} distance={16} />
      <Rig stageEl={stageEl} labels={labels} spreadRef={spreadRef} quality={quality} onHover={onHover} />
    </Canvas>
  );
}
