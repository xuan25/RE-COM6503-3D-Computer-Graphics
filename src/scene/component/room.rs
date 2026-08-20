//! Port of `legacy/scene/component/Room.java`.

use super::{SceneBuilder, SceneResourceError, interfaces::Component};
use crate::{
    gmaths::Vec3,
    graphics::{
        material::Material,
        node::{BasicNode, CenterTransformable, MeshTransformable, ModelNode, Node, NodeLink},
    },
};
use std::{cell::RefCell, rc::Rc};

type BasicNodeRef = Rc<RefCell<BasicNode>>;
type ModelNodeRef = Rc<RefCell<ModelNode>>;

pub struct Room {
    /// The fields below intentionally mirror the named Java `Room` nodes.
    /// Each is also linked into `room_node`, so the retained references and
    /// the scene graph always address the same node.
    room_node: BasicNode,
    floor_node: ModelNodeRef,
    side_wall_node: BasicNodeRef,
    back_wall_node: ModelNodeRef,
    door_node: BasicNodeRef,
    paint_node: BasicNodeRef,
    room_light_node: BasicNodeRef,
    window_view_node: ModelNodeRef,
    custom_lighting_intensity: f32,
    light_material: Material,
    room_material: Material,
    light_nodes: Vec<Rc<RefCell<crate::graphics::lighting::PointLightNode>>>,
}
impl Room {
    pub const WALL_WIDTH: f32 = 30.;
    pub const WALL_HEIGHT: f32 = 15.;
    pub fn new(b: &SceneBuilder) -> Result<Self, SceneResourceError> {
        let room_material = Material::new(
            Vec3::new(1., 1., 1.),
            Vec3::new(1., 1., 1.),
            Vec3::new(0.2, 0.2, 0.2),
            16.,
        );
        let custom_lighting_intensity = 1.;
        let light_material = Self::light_material(custom_lighting_intensity);
        let mut root = BasicNode::new("Room");
        let floor = Rc::new(RefCell::new(ModelNode::new(
            "Floor",
            Some(b.model(room_material, "plane", &["floor", "floor_spec"])?),
        )));
        {
            let mut n = floor.borrow_mut();
            n.model_mut().unwrap().set_uv_scale(2.5, 2.5);
            n.set_mesh_scale(Self::WALL_WIDTH, 1., Self::WALL_WIDTH);
        }
        let back = Rc::new(RefCell::new(ModelNode::new(
            "BackWall",
            Some(b.model(room_material, "plane", &["wall", "wall_spec"])?),
        )));
        {
            let mut n = back.borrow_mut();
            n.model_mut().unwrap().set_uv_scale(1.2, 1.2);
            n.set_mesh_scale(Self::WALL_WIDTH, 1., Self::WALL_HEIGHT);
            n.set_mesh_rotation(90., 0., 0.);
            n.set_mesh_translation(0., Self::WALL_HEIGHT / 2., 0.);
            n.set_center_translation(0., 0., -Self::WALL_WIDTH / 2.);
        }
        let side = Rc::new(RefCell::new(BasicNode::new("WindowWall")));
        side.borrow_mut().set_center_rotation(0., 90., 0.);
        side.borrow_mut()
            .set_center_translation(-Self::WALL_WIDTH / 2., 0., 0.);
        let tile_width = Self::WALL_WIDTH / 3.;
        let tile_height = Self::WALL_HEIGHT / 3.;
        for i in 0..3 {
            for j in 0..3 {
                if i == 1 && j == 1 {
                    continue;
                }
                let tile = Rc::new(RefCell::new(ModelNode::new(
                    format!("SubWall ({i},{j})"),
                    Some(b.model(room_material, "plane", &["wall", "wall_spec"])?),
                )));
                {
                    let mut n = tile.borrow_mut();
                    n.model_mut()
                        .unwrap()
                        .set_uv_offset((i as f32 / 3.) * 1.2, (j as f32 / 3.) * 1.2);
                    n.model_mut().unwrap().set_uv_scale(1.2 / 3., 1.2 / 3.);
                    n.set_mesh_scale(tile_width, 1., tile_height);
                    n.set_mesh_translation(
                        tile_width * (i as f32 - 1.),
                        0.,
                        tile_height * ((2 - j) as f32 - 1.),
                    );
                    n.set_center_rotation(90., 0., 0.);
                    n.set_center_translation(0., Self::WALL_HEIGHT / 2., 0.);
                }
                side.borrow_mut().add_child(Box::new(NodeLink::new(tile)));
            }
        }
        let window_frame = Rc::new(RefCell::new(BasicNode::new("WindowFrame")));
        window_frame
            .borrow_mut()
            .set_center_translation(0., Self::WALL_HEIGHT / 2., 0.);
        for (name, sx, sy, x, y, rotate) in [
            (
                "FrameBottom",
                0.2,
                Self::WALL_WIDTH / 3. + 0.2,
                0.,
                -Self::WALL_HEIGHT / 6.,
                true,
            ),
            (
                "FrameTop",
                0.2,
                Self::WALL_WIDTH / 3. + 0.2,
                0.,
                Self::WALL_HEIGHT / 6.,
                true,
            ),
            (
                "FrameRight",
                0.2,
                Self::WALL_HEIGHT / 3. - 0.2,
                Self::WALL_WIDTH / 6.,
                0.,
                false,
            ),
            (
                "FrameLeft",
                0.2,
                Self::WALL_HEIGHT / 3. - 0.2,
                -Self::WALL_WIDTH / 6.,
                0.,
                false,
            ),
        ] {
            let bar = Rc::new(RefCell::new(ModelNode::new(
                name,
                Some(b.model(room_material, "cube", &["frames", "frames_spec"])?),
            )));
            {
                let mut n = bar.borrow_mut();
                // `createFrame` gives horizontal and vertical bars distinct
                // UV scales before sharing their corresponding Models.
                n.model_mut().unwrap().set_uv_scale(
                    0.2,
                    if rotate {
                        Self::WALL_WIDTH / 3.
                    } else {
                        Self::WALL_HEIGHT / 3.
                    },
                );
                n.set_mesh_scale(sx, sy, 0.4);
                n.set_center_translation(x, y, 0.);
                if rotate {
                    n.set_mesh_rotation(0., 0., 90.)
                }
            }
            window_frame
                .borrow_mut()
                .add_child(Box::new(NodeLink::new(bar)));
        }
        side.borrow_mut()
            .add_child(Box::new(NodeLink::new(window_frame)));
        let window_view = Rc::new(RefCell::new(ModelNode::new(
            "WindowView",
            Some(b.model_with_shader(
                "window_view",
                Material::new(
                    Vec3::new(1., 1., 1.),
                    Vec3::new(1., 1., 1.),
                    Vec3::new(1., 1., 1.),
                    16.,
                ),
                "plane",
                &["window", "snow"],
            )?),
        )));
        {
            let mut n = window_view.borrow_mut();
            n.set_mesh_scale(Self::WALL_WIDTH * 5., 1., Self::WALL_HEIGHT * 5.);
            n.set_mesh_rotation(90., 0., 0.);
            n.set_center_rotation(0., 90., 0.);
            n.set_center_translation(-80., 0., 0.);
        }
        // Door: two mirrored textured planes, placed on the rear wall exactly as the Java scene.
        let door = Rc::new(RefCell::new(BasicNode::new("Door")));
        door.borrow_mut().set_center_translation(
            -Self::WALL_WIDTH * 0.3 + Self::WALL_HEIGHT * 0.3 * 0.5,
            Self::WALL_HEIGHT * 0.7 * 0.5,
            -Self::WALL_WIDTH * 0.5 + 0.01,
        );
        let mut door_panels = Vec::new();
        for (name, scale_u, offset_u, x) in [
            ("LeftDoor", 0.79, 0.1, -Self::WALL_HEIGHT * 0.3 * 0.5),
            ("RightDoor", -0.79, -0.11, Self::WALL_HEIGHT * 0.3 * 0.5),
        ] {
            let node = Rc::new(RefCell::new(ModelNode::new(
                name,
                Some(b.model(room_material, "plane", &["door", "door_spec"])?),
            )));
            {
                let mut n = node.borrow_mut();
                n.model_mut().unwrap().set_uv_scale(scale_u, -0.98);
                n.model_mut().unwrap().set_uv_offset(offset_u, -0.01);
                n.set_mesh_scale(Self::WALL_HEIGHT * 0.3, 1., Self::WALL_HEIGHT * 0.7);
                n.set_mesh_rotation(90., 0., 0.);
                n.set_center_translation(x, 0., 0.);
            }
            door_panels.push(node);
        }
        let door_frame = Rc::new(RefCell::new(BasicNode::new("DoorFrame")));
        for (name, sx, sy, x, y, rotate) in [
            (
                "FrameBottom",
                0.2,
                Self::WALL_HEIGHT * 0.3 * 2. + 0.2,
                0.,
                -Self::WALL_HEIGHT * 0.7 * 0.5,
                true,
            ),
            (
                "FrameTop",
                0.2,
                Self::WALL_HEIGHT * 0.3 * 2. + 0.2,
                0.,
                Self::WALL_HEIGHT * 0.7 * 0.5,
                true,
            ),
            (
                "FrameRight",
                0.2,
                Self::WALL_HEIGHT * 0.7 - 0.2,
                Self::WALL_HEIGHT * 0.3,
                0.,
                false,
            ),
            (
                "FrameLeft",
                0.2,
                Self::WALL_HEIGHT * 0.7 - 0.2,
                -Self::WALL_HEIGHT * 0.3,
                0.,
                false,
            ),
        ] {
            let frame = Rc::new(RefCell::new(ModelNode::new(
                name,
                Some(b.model(room_material, "cube", &["frames", "frames_spec"])?),
            )));
            {
                let mut n = frame.borrow_mut();
                n.model_mut().unwrap().set_uv_scale(
                    0.2,
                    if rotate {
                        Self::WALL_HEIGHT * 0.3 * 2.
                    } else {
                        Self::WALL_HEIGHT * 0.7
                    },
                );
                n.set_mesh_scale(sx, sy, 0.4);
                n.set_center_translation(x, y, 0.);
                if rotate {
                    n.set_mesh_rotation(0., 0., 90.)
                }
            }
            door_frame
                .borrow_mut()
                .add_child(Box::new(NodeLink::new(frame)));
        }
        door.borrow_mut()
            .add_child(Box::new(NodeLink::new(door_frame)));
        for panel in door_panels {
            door.borrow_mut().add_child(Box::new(NodeLink::new(panel)));
        }
        // Painting plane, with a four-bar wooden frame.
        let paint = Rc::new(RefCell::new(BasicNode::new("PaintWithFrame")));
        paint.borrow_mut().set_center_translation(
            Self::WALL_WIDTH * 0.2,
            Self::WALL_HEIGHT * 0.6,
            -Self::WALL_WIDTH * 0.5 + 0.01,
        );
        let painting = Rc::new(RefCell::new(ModelNode::new(
            "Paint",
            Some(b.model(room_material, "plane", &["paint", "paint_spec"])?),
        )));
        {
            let mut n = painting.borrow_mut();
            n.model_mut().unwrap().set_uv_scale(0.7, 0.9);
            n.model_mut().unwrap().set_uv_offset(0.15, 0.045);
            n.set_mesh_scale(Self::WALL_HEIGHT * 0.3, 1., Self::WALL_HEIGHT * 0.4);
            n.set_mesh_rotation(90., 0., 0.);
        }
        paint
            .borrow_mut()
            .add_child(Box::new(NodeLink::new(painting)));
        let paint_frame = Rc::new(RefCell::new(BasicNode::new("PaintFrame")));
        for (name, sx, sy, x, y) in [
            (
                "FrameBottom",
                0.2,
                Self::WALL_HEIGHT * 0.3 + 0.2,
                0.,
                -Self::WALL_HEIGHT * 0.4 * 0.5,
            ),
            (
                "FrameTop",
                0.2,
                Self::WALL_HEIGHT * 0.3 + 0.2,
                0.,
                Self::WALL_HEIGHT * 0.4 * 0.5,
            ),
            (
                "FrameRight",
                0.2,
                Self::WALL_HEIGHT * 0.4 - 0.2,
                Self::WALL_HEIGHT * 0.3 * 0.5,
                0.,
            ),
            (
                "FrameLeft",
                0.2,
                Self::WALL_HEIGHT * 0.4 - 0.2,
                -Self::WALL_HEIGHT * 0.3 * 0.5,
                0.,
            ),
        ] {
            let frame = Rc::new(RefCell::new(ModelNode::new(
                name,
                Some(b.model(room_material, "cube", &["frames", "frames_spec"])?),
            )));
            {
                let mut n = frame.borrow_mut();
                n.model_mut().unwrap().set_uv_scale(
                    0.2,
                    if name == "FrameBottom" || name == "FrameTop" {
                        Self::WALL_HEIGHT * 0.3
                    } else {
                        Self::WALL_HEIGHT * 0.4
                    },
                );
                n.set_mesh_scale(sx, sy, 0.4);
                n.set_center_translation(x, y, 0.);
                if name == "FrameBottom" || name == "FrameTop" {
                    n.set_mesh_rotation(0., 0., 90.);
                }
            }
            paint_frame
                .borrow_mut()
                .add_child(Box::new(NodeLink::new(frame)));
        }
        paint
            .borrow_mut()
            .add_child(Box::new(NodeLink::new(paint_frame)));
        root.add_child(Box::new(NodeLink::new(floor.clone())));
        root.add_child(Box::new(NodeLink::new(side.clone())));
        root.add_child(Box::new(NodeLink::new(back.clone())));
        root.add_child(Box::new(NodeLink::new(door.clone())));
        root.add_child(Box::new(NodeLink::new(paint.clone())));
        let (light_group, light_nodes) = Self::create_room_light_group(b, light_material)?;
        root.add_child(Box::new(NodeLink::new(light_group.clone())));
        root.add_child(Box::new(NodeLink::new(window_view.clone())));
        Ok(Self {
            room_node: root,
            floor_node: floor,
            side_wall_node: side,
            back_wall_node: back,
            door_node: door,
            paint_node: paint,
            room_light_node: light_group,
            window_view_node: window_view,
            custom_lighting_intensity,
            light_material,
            room_material,
            light_nodes,
        })
    }

    /// Rust counterpart of `Room.createRoomLightGroup(SceneBuilder)`.
    fn create_room_light_group(
        b: &SceneBuilder,
        light_material: Material,
    ) -> Result<
        (
            Rc<RefCell<BasicNode>>,
            Vec<Rc<RefCell<crate::graphics::lighting::PointLightNode>>>,
        ),
        SceneResourceError,
    > {
        let light_group = Rc::new(RefCell::new(BasicNode::new("RoomLightGroup")));
        let mut light_nodes = Vec::new();
        for x in [-6., 6.] {
            for z in [-6., 6.] {
                let light_model = b.light_source_model(light_material)?;
                let l = b.light_library.borrow_mut().create_point_light(
                    format!("RoomLight ({x},{z})"),
                    light_material,
                    light_model,
                );
                {
                    let mut light = l.borrow_mut();
                    light.set_mesh_translation(x, 0., z);
                    light.set_center_translation(0., Self::WALL_HEIGHT * 0.8, 0.);
                }
                light_group
                    .borrow_mut()
                    .add_child(Box::new(NodeLink::new(l.clone())));
                light_nodes.push(l)
            }
        }
        Ok((light_group, light_nodes))
    }
    pub fn window_view_node(&self) -> Rc<RefCell<ModelNode>> {
        self.window_view_node.clone()
    }
    pub fn light_group_node(&self) -> Rc<RefCell<BasicNode>> {
        self.room_light_node.clone()
    }
    /// Equivalent to the `roomLightRotationSlider` callback in
    /// `MuseumControlPanel.java`.  The Java point lights are scene nodes, while
    /// the Rust renderer stores their GPU data in `LightLibrary`; keep both
    /// representations in lockstep here.
    pub fn set_light_group_rotation(&mut self, degrees: f32) {
        {
            let mut group = self.room_light_node.borrow_mut();
            group.set_center_rotation(0., degrees, 0.);
            group.update();
        }
    }
    pub fn set_custom_lighting_intensity(&mut self, i: f32) {
        self.custom_lighting_intensity = i;
        self.light_material = Self::light_material(i);
        for l in &self.light_nodes {
            let mut l = l.borrow_mut();
            l.set_material(self.light_material);
        }
    }

    fn light_material(intensity: f32) -> Material {
        Material::new(
            Vec3::new(0.2 * intensity, 0.15 * intensity, 0.1 * intensity),
            Vec3::new(intensity, 0.9 * intensity, 0.7 * intensity),
            Vec3::new(intensity, 0.9 * intensity, 0.7 * intensity),
            Material::DEFAULT_SHININESS,
        )
    }
}
impl Component for Room {
    fn node(&self) -> &dyn Node {
        &self.room_node
    }
    fn node_mut(&mut self) -> &mut dyn Node {
        &mut self.room_node
    }
}
