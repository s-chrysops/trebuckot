use core::f32::consts;
use macroquad::math::{I64Vec2, Vec2, Vec3, vec2, vec3};
use crate::utils::*;

const GRAVITY: f32 = 9.81;

#[derive(Debug, PartialEq)]
pub enum TrebuchetState {
    Stage1,
    Stage2,
    Stage3,
}

struct TrebuchetArm {
    long_length:  f32,
    short_length: f32,
    center:       f32,
    mass:         f32,
    inertia:      f32,
    angle:        f32,
    velocity:     f32,
}

impl TrebuchetArm {
    fn new(long_length: f32, short_length: f32, mass: f32) -> Self {
        Self {
            long_length,
            short_length,
            center:       (long_length + short_length) / 2.0 - short_length,
            mass,
            inertia:      mass * (long_length + short_length).powi(2) / 12.0,
            angle:        0.0,
            velocity:     0.0,
        }
    }
}

struct TrebuchetWeight {
    length:     f32,
    mass:       f32,
    inertia:    f32,
    angle:      f32,
    velocity:   f32,
}

impl TrebuchetWeight {
    fn new(length: f32, mass: f32) -> Self {
        Self {
            length,
            mass,
            inertia:  1.0,
            angle:    0.0,
            velocity: 0.0,
        }
    }
}

struct TrebuchetSling {
    length:   f32,
    angle:    f32,
    velocity: f32,
}

impl TrebuchetSling {
    fn new(length: f32) -> Self {
        Self {
            length,
            angle:    0.0,
            velocity: 0.0,
        }
    }
}

#[derive(Default)]
pub struct TrebuchetBuilder {
    position: I64Vec2,
    height: Option<f32>,
    m_proj: Option<f32>,

    arm: Option<TrebuchetArm>,
    weight: Option<TrebuchetWeight>,
    sling: Option<TrebuchetSling>,
}

#[allow(dead_code)]
impl TrebuchetBuilder {
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn m_proj(mut self, m_proj: f32) -> Self {
        self.m_proj = Some(m_proj);
        self
    }

    pub fn arm(mut self, long_length: f32, short_length: f32, mass: f32) -> Self {
        self.arm = Some(TrebuchetArm::new(long_length, short_length, mass));
        self
    }

    pub fn weight(mut self, length: f32, mass: f32) -> Self {
        self.weight = Some(TrebuchetWeight::new(length, mass));
        self
    }

    pub fn sling(mut self, length: f32) -> Self {
        self.sling = Some(TrebuchetSling::new(length));
        self
    }

    pub fn build(self) -> Trebuchet {
        Trebuchet {
            position: self.position,
            height: self.height.unwrap_or(5.6),
            m_proj: self.m_proj.unwrap_or(0.3),
            arm: self.arm.unwrap_or(TrebuchetArm::new(8.0, 2.0, 12.0)),
            weight: self.weight.unwrap_or(TrebuchetWeight::new(2.0, 100.0)),
            sling: self.sling.unwrap_or(TrebuchetSling::new(8.0)),
            state: TrebuchetState::Stage1,
        }
    }
} 

pub struct Trebuchet {
    pub position: I64Vec2,
    pub height:   f32,
    m_proj:   f32,

    arm:    TrebuchetArm,
    weight: TrebuchetWeight,
    sling:  TrebuchetSling,

    pub state: TrebuchetState,
}

impl Trebuchet {
    pub fn init(position: I64Vec2) -> TrebuchetBuilder {
        TrebuchetBuilder { position, ..Default::default() }
    }

    pub fn armsling_point(&self) -> Vec2 {
        vec2(
            -self.arm.long_length * self.arm.angle.sin(),
            self.arm.long_length * self.arm.angle.cos(),
        )
    }

    pub fn armweight_point(&self) -> Vec2 {
        vec2(
            self.arm.short_length * self.arm.angle.sin(),
            -self.arm.short_length * self.arm.angle.cos(),
        )
    }

    pub fn sling_point(&self) -> Vec2 {
        vec2(
            (-self.arm.long_length * self.arm.angle.sin())
                - (self.sling.length * (self.arm.angle + self.sling.angle).sin()),
            (self.arm.long_length * self.arm.angle.cos())
                + (self.sling.length * (self.arm.angle + self.sling.angle).cos()),
        )
    }

    pub fn weight_point(&self) -> Vec2 {
        vec2(
            (self.arm.short_length * self.arm.angle.sin())
                + (self.weight.length * (self.arm.angle + self.weight.angle).sin()),
            (-self.arm.short_length * self.arm.angle.cos())
                - (self.weight.length * (self.arm.angle + self.weight.angle).cos()),
        )
    }

    pub fn projectile_position (&self) -> I64Vec2 {
        to_i64coords(self.sling_point() + vec2(0.0, self.height)) + self.position
    }

    pub fn v_projectile(&self) -> Vec2 {
        vec2(
            -self.arm.long_length * self.arm.angle.cos() * self.arm.velocity 
                - self.sling.length * (self.arm.angle + self.sling.angle).cos() 
                * (self.arm.velocity + self.sling.velocity),
            -self.arm.long_length * self.arm.angle.sin() * self.arm.velocity 
                - self.sling.length * (self.arm.angle + self.sling.angle).sin() 
                * (self.arm.velocity + self.sling.velocity),
        )
    }

    pub fn reset(&mut self) {
        let common_triangle  = self.height / self.arm.long_length;
        self.arm.velocity    = 0.0;
        self.weight.velocity = 0.0;
        self.sling.velocity  = 0.0;
        self.arm.angle       = consts::PI - common_triangle.acos();
        self.weight.angle    = common_triangle.acos() - consts::PI;
        self.sling.angle     = consts::PI - common_triangle.asin();
        self.state           = TrebuchetState::Stage1;
        self.m_proj          = 0.3;
    }

    pub fn run(&mut self, dt: f32) {
        let stage: Box<dyn Fn(f32, Vec3, Vec3) -> Vec3x2> = match self.state {

            TrebuchetState::Stage1 => {
                let Vec3x2{q: _, w: Vec3{x: aw_prime, ..}} = self.stage_1 ( 
                    dt,
                    vec3(self.arm.angle, self.weight.angle, self.sling.angle),
                    vec3(self.arm.velocity, self.weight.velocity, self.sling.velocity)
                );
                if self.ground_force(aw_prime) <= 0.0 {
                    self.state = TrebuchetState::Stage2;
                }
                Box::new(|dt: f32, x: Vec3, y: Vec3| self.stage_1(dt, x, y))
            }

            TrebuchetState::Stage2 => {
                if to_angle(self.v_projectile()) <= consts::FRAC_PI_4 {
                    self.m_proj = 0.01;
                    self.state = TrebuchetState::Stage3;
                }
                Box::new(|dt: f32, x: Vec3, y: Vec3| self.stage_2(dt, x, y))
            }

            TrebuchetState::Stage3 => {
                Box::new(|dt: f32, x: Vec3, y: Vec3| self.stage_2(dt, x, y))
            }
        };
        
        let rk4_results = rk4 (
            vec3(self.arm.angle, self.weight.angle, self.sling.angle),
            vec3(self.arm.velocity, self.weight.velocity, self.sling.velocity),
            dt, 
            stage
        );
        Vec3 {x: self.arm.angle, y: self.weight.angle, z: self.sling.angle}         = rk4_results.q;
        Vec3 {x: self.arm.velocity, y: self.weight.velocity, z:self.sling.velocity} = rk4_results.w;
    }

    fn ground_force(&self, aw_prime: f32) -> f32 {
        self.m_proj * (GRAVITY + (self.sling.length * ((self.arm.angle + self.sling.angle).cos() 
            * self.sling.velocity * (self.sling.velocity + 2.0 * self.arm.velocity) 
            / (self.arm.angle + self.sling.angle).sin() + ((self.arm.angle + self.sling.angle).cos() 
            / (self.arm.angle + self.sling.angle).sin() + self.arm.long_length * self.arm.angle.cos() 
            / (self.sling.length * (self.arm.angle + self.sling.angle).sin())) 
            * self.arm.velocity.powi(2)) - self.arm.long_length * self.sling.angle.sin() 
            * self.arm.velocity.powi(2) - self.arm.long_length * (self.sling.angle.cos() 
            - self.arm.angle.sin() / (self.arm.angle + self.sling.angle).sin()) * aw_prime) 
            / (self.arm.angle + self.sling.angle).sin())
    }

    fn stage_1(&self, dt: f32, angles: Vec3, velocities: Vec3) -> Vec3x2 {
        let lal = self.arm.long_length;
        let las = self.arm.short_length;
        let cga = self.arm.center;
        let lw = self.weight.length;
        let ls = self.sling.length;

        let ma = self.arm.mass;
        let mw = self.weight.mass;
        let mp = self.m_proj;

        let ia = self.arm.inertia;
        let iw = self.weight.inertia;

        let aq = angles.x;
        let wq = angles.y;
        let sq = angles.z;

        let aw = velocities.x;
        let ww = velocities.y;
        let sw = velocities.z;

        #[rustfmt::skip]
        let m11 = -mp * lal.powi(2)
            * (-1.0 + 2.0 * aq.sin() * sq.cos() / (aq + sq).sin()) + ia + iw + ma * cga.powi(2) 
            + mp * lal.powi(2) * aq.sin().powi(2) / (aq + sq).sin().powi(2) + mw * (las.powi(2) 
            + lw.powi(2)+ 2.0 * las * lw * wq.cos());
        let m12 = iw + lw * mw * (lw + las * wq.cos());
        let m21 = iw + lw * mw * (lw + las * wq.cos());
        let m22 = iw + mw * lw.powi(2);

        #[rustfmt::skip]
        let r1 = GRAVITY * cga * ma * aq.sin() + lal * ls * mp * (sq.sin() * (aw + sw).powi(2) 
            + sq.cos() * ((aq + sq).cos() * sw * (sw + 2.0 * aw) / (aq + sq).sin() 
            + ((aq + sq).cos() / (aq + sq).sin() + lal * aq.cos() / (ls * (aq + sq).sin())) 
            * aw.powi(2))) + lal * mp * aq.sin() * (lal * sq.sin() * aw.powi(2) - ls
            * ((aq + sq).cos() * sw * (sw + 2.0 * aw) / (aq  + sq).sin() + ((aq + sq).cos()
            / (aq + sq).sin() + lal * aq.cos() / (ls * (aq + sq).sin())) * aw.powi(2))) 
            / (aq + sq).sin() - GRAVITY * mw * (las * aq.sin() + lw * (aq + wq).sin()) - las * lw 
            * mw * wq.sin() * (aw.powi(2) - (aw + ww).powi(2));
        let r2 = -lw * mw * (GRAVITY * (aq + wq) + las * wq.sin() * aw.powi(2));

        let aw_prime = (r1 * m22 - r2 * m12) / (m11 * m22 - m12 * m21);
        let ww_prime = -(r1 * m21 - r2 * m11) / (m11 * m22 - m12 * m21);
        #[rustfmt::skip]
        let sw_prime = -(aq + sq).cos() * sw * (sw + 2.0 * aw) / (aq + sq).sin() 
            - ((aq + sq).cos() / (aq + sq).sin() + lal * aq.cos() / (ls * (aq + sq).sin())) 
            * aw.powi(2) - (lal * aq.sin() + ls * (aq + sq).sin()) * aw_prime 
            / (ls * (aq + sq).sin());
                
        Vec3x2 {
            q: vec3(aw + aw_prime * dt, ww + ww_prime * dt, sw + sw_prime * dt),
            w: vec3(aw_prime, ww_prime, sw_prime)
        }
    }

    fn stage_2(&self, dt: f32, angles: Vec3, velocities: Vec3) -> Vec3x2 {
        let lal = self.arm.long_length;
        let las = self.arm.short_length;
        let cga = self.arm.center;
        let lw = self.weight.length;
        let ls = self.sling.length;

        let ma = self.arm.mass;
        let mw = self.weight.mass;
        let mp = self.m_proj;

        let ia = self.arm.inertia;
        let iw = self.weight.inertia;

        let aq = angles.x;
        let wq = angles.y;
        let sq = angles.z;

        let aw = velocities.x;
        let ww = velocities.y;
        let sw = velocities.z;

        #[rustfmt::skip]
        let m11 = ia + iw + ma * cga.powi(2) + mp 
            * (lal.powi(2) + ls.powi(2) + 2.0 * lal * ls * sq.cos()) 
            + mw * (las.powi(2) + lw.powi(2) + 2.0 * las * lw * wq.cos());
        let m12 = iw + lw * mw * (lw + las * wq.cos());
        let m13 = ls * mp * (ls + lal * sq.cos());
        let m21 = iw + lw * mw * (lw + las * wq.cos());
        let m22 = iw + mw * lw.powi(2);
        let m31 = ls * mp * (ls + lal * sq.cos());
        let m33 = mp * ls.powi(2);

        #[rustfmt::skip]
        let r1 = GRAVITY * cga * ma * aq.sin() + GRAVITY * mp 
            * (lal * aq.sin() + ls * (aq + sq).sin()) - GRAVITY * mw 
            * (las * aq.sin() + lw * (aq + wq).sin()) - lal * ls * mp * sq.sin() 
            * (aw.powi(2) - (aw + sw).powi(2)) 
            - las * lw * mw * wq.sin() 
            * (aw.powi(2) - (aw + ww).powi(2));
        let r2 = -lw * mw * (GRAVITY * (aq + wq).sin() + las * wq.sin() * aw.powi(2));
        let r3 = ls * mp * (GRAVITY * (aq + sq).sin() - lal * sq.sin() * aw.powi(2));

        let aw_prime = -(r1 * m22 * m33 - r2 * m12 * m33 - r3 * m13 * m22) 
            / (m13 * m22 * m31 - m33 * (m11 * m22 - m12 * m21));
        let ww_prime = (r1 * m21 * m33 - r2 * (m11 * m33 - m13 * m31) - r3 * m13 * m21) 
            / (m13 * m22 * m31 - m33 * (m11 * m22 - m12 * m21));
        let sw_prime = (r1 * m22 * m31 - r2 * m12 * m31 - r3 * (m11 * m22 - m12 * m21)) 
            / (m13 * m22 * m31 - m33 * (m11 * m22 - m12 * m21));
        
        Vec3x2 {
            q: vec3(aw + aw_prime * dt, ww + ww_prime * dt, sw + sw_prime * dt),
            w: vec3(aw_prime, ww_prime, sw_prime)
        }
    }
}

// Container for set of angular positions and velocities or their derivatives that can be multiplied
struct Vec3x2{
    q: Vec3,
    w: Vec3,
}
impl std::ops::Mul<Vec3x2> for f32 {
    type Output = Vec3x2;
    fn mul(self, rhs: Vec3x2) -> Vec3x2 {
        Vec3x2 {
            q: Vec3 {
                x: self.mul(rhs.q.x),
                y: self.mul(rhs.q.y),
                z: self.mul(rhs.q.z),
            },
            w: Vec3 {
                x: self.mul(rhs.w.x),
                y: self.mul(rhs.w.y),
                z: self.mul(rhs.w.z),
            }
        }
    }
}

fn rk4<T>(x: Vec3, y: Vec3, dt: f32, f: T) -> Vec3x2
where
    T: Fn(f32, Vec3, Vec3) -> Vec3x2
{
    let k1 = dt * f(dt, x, y);
    let k2 = dt * f(dt, x + k1.q * 0.5, y + k1.w * 0.5);
    let k3 = dt * f(dt, x + k1.q * 0.5, y + k2.w * 0.5);
    let k4 = dt * f(dt, x + k3.q, y + k3.w);
    Vec3x2{   
        q: (x + (k1.q + 2.0 * k2.q + 2.0 * k3.q + k4.q) / 6.0),
        w: (y + (k1.w + 2.0 * k2.w + 2.0 * k3.w + k4.w) / 6.0),
    }
}
