//! Physical plates and a returning signal, matching the website's exploded stack.
use fframes::three_d::*;
use fframes::media::ImageData;

pub struct LayerStage {
    scene: Scene,
    plates: Vec<NodeId>,
    packet: NodeId,
    renderer: Renderer,
}
impl std::fmt::Debug for LayerStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("OriginLM layers") }
}
impl LayerStage {
    pub fn new() -> Self {
        let mut scene = Scene::new().environment(Environment::studio()).background(Background::Transparent);
        let plates = (0..6).map(|i| {
            scene.add(Node::mesh(
                Geometry::rounded_box(3.7 + i as f32 * 0.35, 0.14, 1.45, 0.06, 3),
                Material::metal(0x4a3831, 0.22).clearcoat(1.0),
            ).at(0., (2.5 - i as f32)*0.7, 0.))
        }).collect();
        let packet = scene.add(Node::mesh(Geometry::sphere(0.07, 12, 8), Material::glow(Rgb::hex(0xeba31a) * 5.0)));
        scene.add(Node::light(Light::sun(Rgb::hex(0xfff2dc), 3.0, vec3(3.,6.,5.))));
        scene.add(Node::light(Light::point(Rgb::hex(0xeba31a), 12.).at(0.,-3.,3.)));
        Self {scene,plates,packet,renderer:Renderer::new(1480,740).tone_mapping(ToneMapping::Neutral).bloom(Bloom::default())}
    }
    pub fn image(&self,t:f32)->ImageData<'static> {
        let mut scene=self.scene.clone();
        let spread=0.55+0.45*(1.-(t*std::f32::consts::TAU/6.).cos())*0.5;
        for (i,id) in self.plates.iter().enumerate() {
            scene[*id].position.y=(2.5-i as f32)*0.85*spread;
        }
        scene[self.packet].position=vec3(0., (t*std::f32::consts::TAU/6.).sin()*2.5*0.85*spread, 0.85);
        let camera=Camera::perspective(32.).orbit(vec3(0.,0.,0.),11.,28.,18.);
        self.renderer.render(&scene,&camera)
    }
}
