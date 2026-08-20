//! Port of `legacy/scene/animator/RobotAnimator.java`.

use super::{
    interfaces::Animator,
    utils::{Bezier, ease_in_out_cubic, simplify_rotation, travel_direction},
};
use crate::scene::component::{Robot, interfaces::Component};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Copy, PartialEq)]
struct RobotState {
    x: f32,
    z: f32,
    rotation: f32,
    body_pitch: f32,
    head_pitch: f32,
    head_yaw: f32,
    eye_pitch: f32,
    antenna_pitch: f32,
    sub_antenna_pitch: f32,
}

/// Direct equivalent of Java's nested `RobotAnimator.WayPoint` class.
/// Routes use named coordinates instead of anonymous tuples so the original
/// animation abstraction remains visible in the Rust port.
#[derive(Clone, Copy, PartialEq)]
struct WayPoint {
    x: f32,
    z: f32,
}

impl WayPoint {
    const fn new(x: f32, z: f32) -> Self {
        Self { x, z }
    }
}

impl RobotState {
    const fn new(
        x: f32,
        z: f32,
        rotation: f32,
        body_pitch: f32,
        head_pitch: f32,
        head_yaw: f32,
        eye_pitch: f32,
        antenna_pitch: f32,
        sub_antenna_pitch: f32,
    ) -> Self {
        Self {
            x,
            z,
            rotation,
            body_pitch,
            head_pitch,
            head_yaw,
            eye_pitch,
            antenna_pitch,
            sub_antenna_pitch,
        }
    }
}
pub struct RobotAnimator {
    robot: Rc<RefCell<Robot>>,
    start: RobotState,
    end: RobotState,
    timestamp: f64,
    animating: bool,
    last_pose: i32,
    /// Java's `bezierPoints`; an empty vector is the Rust `null` equivalent.
    waypoints: Vec<WayPoint>,
}
impl RobotAnimator {
    pub const PRE_DURATION: f64 = 1.0;
    pub const TRAVEL_DURATION: f64 = 3.0;
    pub const POST_DURATION: f64 = 1.0;
    pub const TRAVEL_LEAN_DURATION: f64 = 1.0;
    pub const TRAVEL_LEAN_ANGLE: f32 = 15.0;
    pub fn new(robot: Rc<RefCell<Robot>>) -> Self {
        let state = RobotState::new(-8., -10., 0., 0., 0., 0., 0., 90., 90.);
        let mut r = robot.borrow_mut();
        Self::apply(&mut r, state);
        r.node_mut().update();
        drop(r);
        Self {
            robot,
            start: state,
            end: state,
            timestamp: 0.,
            animating: false,
            last_pose: 1,
            waypoints: Vec::new(),
        }
    }
    fn apply(robot: &mut Robot, s: RobotState) {
        robot.set_robot_position(s.x, s.z);
        robot.set_body_rotation(s.rotation);
        robot.set_body_pitch(s.body_pitch);
        robot.set_head_pitch(s.head_pitch);
        robot.set_head_yaw(s.head_yaw);
        robot.set_eye_pitch(s.eye_pitch);
        robot.set_antenna_pitch(s.antenna_pitch);
        robot.set_sub_antenna_pitch(s.sub_antenna_pitch)
    }
    fn animate_to(&mut self, end: RobotState) -> bool {
        if self.animating || end == self.end {
            return false;
        }
        self.start = self.end;
        self.end = end;
        self.timestamp = 0.;
        self.animating = true;
        true
    }
    fn set_route(&mut self, pose: i32, routes: &[(i32, Vec<WayPoint>)]) -> bool {
        if self.animating {
            return false;
        }
        self.waypoints = routes
            .iter()
            .find(|(from, _)| *from == self.last_pose)
            .map(|(_, route)| route.clone())
            .unwrap_or_default();
        self.last_pose = pose;
        true
    }
    pub fn pose_demo(&mut self) -> bool {
        if self.animating {
            return false;
        }
        self.waypoints.clear();
        self.last_pose = -1;
        self.animate_to(RobotState::new(
            -10., -10., 0., 20., 20., 30., 30., 30., 30.,
        ))
    }
    pub fn pose1(&mut self) -> bool {
        if !self.set_route(
            1,
            &[
                (3, vec![WayPoint::new(5., -5.)]),
                (4, vec![WayPoint::new(-10., 0.)]),
            ],
        ) {
            return false;
        }
        self.animate_to(RobotState::new(-8., -10., 0., 0., 0., 0., 0., 90., 90.))
    }
    pub fn pose2(&mut self) -> bool {
        if !self.set_route(
            2,
            &[
                (4, vec![WayPoint::new(10., 0.)]),
                (5, vec![WayPoint::new(-5., -10.)]),
            ],
        ) {
            return false;
        }
        self.animate_to(RobotState::new(4., -7., 90., 20., -10., 30., -30., 0., 30.))
    }
    pub fn pose3(&mut self) -> bool {
        if !self.set_route(
            3,
            &[
                (1, vec![WayPoint::new(5., -5.)]),
                (5, vec![WayPoint::new(-5., 10.), WayPoint::new(5., 10.)]),
            ],
        ) {
            return false;
        }
        self.animate_to(RobotState::new(6., -1., 30., 0., 30., 0., 30., 90., 90.))
    }
    pub fn pose4(&mut self) -> bool {
        if !self.set_route(
            4,
            &[
                (1, vec![WayPoint::new(-10., 0.)]),
                (2, vec![WayPoint::new(10., 0.)]),
            ],
        ) {
            return false;
        }
        self.animate_to(RobotState::new(0., 10., 220., 0., 0., -40., -20., 0., 0.))
    }
    pub fn pose5(&mut self) -> bool {
        if !self.set_route(
            5,
            &[
                (2, vec![WayPoint::new(-5., -10.)]),
                (3, vec![WayPoint::new(5., 10.), WayPoint::new(-5., 10.)]),
            ],
        ) {
            return false;
        }
        self.animate_to(RobotState::new(-10., 0., -90., 0., -20., 0., 0., 90., 60.))
    }
    fn interpolate(a: RobotState, b: RobotState, t: f32) -> RobotState {
        let l = |x, y| x + (y - x) * t;
        RobotState::new(
            l(a.x, b.x),
            l(a.z, b.z),
            l(a.rotation, b.rotation),
            l(a.body_pitch, b.body_pitch),
            l(a.head_pitch, b.head_pitch),
            l(a.head_yaw, b.head_yaw),
            l(a.eye_pitch, b.eye_pitch),
            l(a.antenna_pitch, b.antenna_pitch),
            l(a.sub_antenna_pitch, b.sub_antenna_pitch),
        )
    }
    fn path(&self, ratio: f64) -> (f32, f32, f32) {
        if self.waypoints.is_empty() {
            let x = self.start.x + (self.end.x - self.start.x) * ratio as f32;
            let z = self.start.z + (self.end.z - self.start.z) * ratio as f32;
            return (
                x,
                z,
                travel_direction(self.start.x, self.start.z, self.end.x, self.end.z),
            );
        }
        let mut xs = vec![self.start.x as f64];
        let mut zs = vec![self.start.z as f64];
        for point in &self.waypoints {
            xs.push(point.x as f64);
            zs.push(point.z as f64)
        }
        xs.push(self.end.x as f64);
        zs.push(self.end.z as f64);
        let x = Bezier::new(&xs, ratio);
        let z = Bezier::new(&zs, ratio);
        (
            x.value as f32,
            z.value as f32,
            travel_direction(0., 0., x.slope as f32, z.slope as f32),
        )
    }
}
impl Animator for RobotAnimator {
    fn forward(&mut self, seconds: f64) {
        if !self.animating {
            return;
        }
        self.timestamp += seconds;
        let mut robot = self.robot.borrow_mut();
        if self.timestamp < Self::PRE_DURATION {
            let ratio = ease_in_out_cubic(self.timestamp / Self::PRE_DURATION) as f32;
            let (_, _, direction) = self.path(0.0);
            let direction = simplify_rotation(self.start.rotation, direction);
            Self::apply(
                &mut robot,
                RobotState::new(
                    self.start.x,
                    self.start.z,
                    self.start.rotation + (direction - self.start.rotation) * ratio,
                    self.start.body_pitch * (1. - ratio),
                    self.start.head_pitch * (1. - ratio),
                    self.start.head_yaw * (1. - ratio),
                    self.start.eye_pitch * (1. - ratio),
                    self.start.antenna_pitch + (90. - self.start.antenna_pitch) * ratio,
                    self.start.sub_antenna_pitch + (90. - self.start.sub_antenna_pitch) * ratio,
                ),
            );
        } else if self.timestamp < Self::PRE_DURATION + Self::TRAVEL_DURATION {
            let stage = self.timestamp - Self::PRE_DURATION;
            // `anmRatio` is a Java float.  Bezier receives that widened float,
            // rather than the original double-precision easing result.
            let ratio = ease_in_out_cubic(stage / Self::TRAVEL_DURATION) as f32;
            let (x, z, direction) = self.path(ratio as f64);
            let lean = if stage < Self::TRAVEL_LEAN_DURATION {
                ease_in_out_cubic(stage / Self::TRAVEL_LEAN_DURATION) as f32
            } else if Self::TRAVEL_DURATION - stage < Self::TRAVEL_LEAN_DURATION {
                ease_in_out_cubic((Self::TRAVEL_DURATION - stage) / Self::TRAVEL_LEAN_DURATION)
                    as f32
            } else {
                1.
            };
            Self::apply(
                &mut robot,
                RobotState::new(
                    x,
                    z,
                    direction,
                    Self::TRAVEL_LEAN_ANGLE * lean,
                    -Self::TRAVEL_LEAN_ANGLE * 0.5 * lean,
                    0.,
                    -Self::TRAVEL_LEAN_ANGLE * 0.5 * lean,
                    90.,
                    90.,
                ),
            );
        } else if self.timestamp < Self::PRE_DURATION + Self::TRAVEL_DURATION + Self::POST_DURATION
        {
            let ratio = ease_in_out_cubic(
                (self.timestamp - Self::PRE_DURATION - Self::TRAVEL_DURATION) / Self::POST_DURATION,
            ) as f32;
            let (_, _, direction) = self.path(1.0);
            let direction = simplify_rotation(self.end.rotation, direction);
            Self::apply(
                &mut robot,
                RobotState::new(
                    self.end.x,
                    self.end.z,
                    direction + (self.end.rotation - direction) * ratio,
                    self.end.body_pitch * ratio,
                    self.end.head_pitch * ratio,
                    self.end.head_yaw * ratio,
                    self.end.eye_pitch * ratio,
                    90. + (self.end.antenna_pitch - 90.) * ratio,
                    90. + (self.end.sub_antenna_pitch - 90.) * ratio,
                ),
            );
        } else {
            Self::apply(&mut robot, self.end);
            self.animating = false;
        }
        robot.node_mut().update();
    }
}
