#![allow(dead_code)]
use core::f32::consts;
use macroquad::math::*;
use crate::utils::*;

const GRAVITY: f32 = 9.81;
use TrebuchetMaterial as TM;

#[derive(Default)]
pub enum TrebuchetMaterial {
    #[default]
    Cardboard,
    Wood1,
    Wood2,
    Steel,
    Space,
}

#[derive(Debug, PartialEq)]
enum TrebuchetState {
    Stage1,
    Stage2,
    Stage3,
}

pub struct TrebuchetArm {
    long_length:  f32,
    short_length: f32,
    center:       f32,
    mass:         f32,
    inertia:      f32,
    angle:        f32,
    velocity:     f32,
    material:     TM,
}

impl TrebuchetArm {
    fn new(long_length: f32, short_length: f32, mass: f32, material: TM) -> Self {
        Self {
            long_length,
            short_length,
            center:       (long_length + short_length) / 2.0 - short_length,
            mass,
            inertia:      mass * (long_length + short_length).powi(2) / 12.0,
            angle:        0.0,
            velocity:     0.0,
            material,
        }
    }

    pub fn texture(&self) -> &str {
        match self.material {
            TM::Cardboard => "cardboard_arm",
            TM::Wood1 => "wood1_arm",
            TM::Wood2 => "wood2_arm",
            TM::Steel => "steel_arm",
            TM::Space => "space_arm",
        }
    }
}

pub struct TrebuchetWeight {
    length:   f32,
    mass:     f32,
    inertia:  f32,
    angle:    f32,
    velocity: f32,
    material: TM,
}

impl TrebuchetWeight {
    fn new(length: f32, mass: f32, material: TM) -> Self {
        Self {
            length,
            mass,
            inertia:  1.0,
            angle:    0.0,
            velocity: 0.0,
            material,
        }
    }

    pub fn texture(&self) -> &str {
        match self.material {
            TM::Cardboard => "cardboard_weight",
            TM::Wood1 => "wood1_weight",
            TM::Wood2 => "wood2_weight",
            TM::Steel => "steel_weight",
            TM::Space => "space_weight",
        }
    }
}

pub struct TrebuchetSling {
    length:   f32,
    angle:    f32,
    velocity: f32,
    material: TM,
}

impl TrebuchetSling {
    fn new(length: f32, material: TM) -> Self {
        Self {
            length,
            angle:    0.0,
            velocity: 0.0,
            material,
        }
    }

    pub fn texture(&self) -> (&str, &str) {
        match self.material {
            TM::Cardboard => ("cardboard_sling_close", "cardboard_sling_open"),
            TM::Wood1 => ("wood1_sling_close", "wood1_sling_open"),
            TM::Wood2 => ("wood2_sling_close", "wood2_sling_open"),
            TM::Steel => ("steel_sling_close", "steel_sling_open"),
            TM::Space => ("space_sling_close", "space_sling_open"),
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

    pub fn arm(mut self, long_length: f32, short_length: f32, mass: f32, material: TM) -> Self {
        self.arm = Some(TrebuchetArm::new(long_length, short_length, mass, material));
        self
    }

    pub fn weight(mut self, length: f32, mass: f32, material: TM) -> Self {
        self.weight = Some(TrebuchetWeight::new(length, mass, material));
        self
    }

    pub fn sling(mut self, length: f32, material: TM) -> Self {
        self.sling = Some(TrebuchetSling::new(length, material));
        self
    }

    pub fn center(mut self, center: f32) -> Self {
        let mut arm = self.arm.expect("No arm constructed");
        arm.center = center;
        self.arm = Some(arm);
        self
    }

    pub async fn build(self) -> Result<Trebuchet, crate::GameError> {
        Ok(Trebuchet {
            position: self.position,
            height: self.height.unwrap_or(1.0),
            m_proj: self.m_proj.unwrap_or(0.3),
            arm: self.arm.unwrap_or(TrebuchetArm::new(1.6, 0.4, 0.25, TM::Cardboard)),
            weight: self.weight.unwrap_or(TrebuchetWeight::new(0.5, 5.0, TM::Cardboard)),
            sling: self.sling.unwrap_or(TrebuchetSling::new(1.6, TM::Cardboard)),
            state: TrebuchetState::Stage1,
        })
    }
} 

pub struct Trebuchet {
    pub position: I64Vec2,
    pub height:   f32,
    m_proj:       f32,

    pub arm:    TrebuchetArm,
    pub weight: TrebuchetWeight,
    pub sling:  TrebuchetSling,

    state:    TrebuchetState,
}

impl Trebuchet {
    pub fn init(position: I64Vec2) -> TrebuchetBuilder {
        TrebuchetBuilder { position, ..Default::default() }
    }

    pub fn armsling_point(&self) -> Vec2 {
        Vec2::from_angle(self.arm.angle).rotate(Vec2::Y * self.arm.long_length)
    }

    pub fn armweight_point(&self) -> Vec2 {
        Vec2::from_angle(self.arm.angle).rotate(Vec2::NEG_Y * self.arm.short_length)
    }

    pub fn sling_point(&self) -> Vec2 {
        Vec2::from_angle(self.arm.angle + self.sling.angle).rotate(Vec2::Y * self.sling.length)
            + self.armsling_point()
    }

    pub fn weight_point(&self) -> Vec2 {
        Vec2::from_angle(self.arm.angle + self.weight.angle).rotate(Vec2::NEG_Y * self.weight.length)
            + self.armweight_point()
    }

    pub fn projectile_position (&self) -> I64Vec2 {
        to_i64coords(self.sling_point() + Vec2::Y * self.height) + self.position
    }

    pub fn v_projectile(&self) -> Vec2 {
        Vec2::from_angle(self.arm.angle + self.sling.angle).rotate(Vec2::NEG_X * self.sling.length) 
            * (self.arm.velocity + self.sling.velocity)
            + self.armsling_point().perp() * self.arm.velocity
    }

    pub fn w_projectile(&self) -> f32 {
        self.arm.velocity + self.sling.velocity
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

    pub fn run(&mut self, dt: f32) -> bool {
        let mat = mat3a(
            vec3a(self.arm.angle, self.weight.angle, self.sling.angle),
            vec3a(self.arm.velocity, self.weight.velocity, self.sling.velocity),
            Vec3A::ZERO
        );

        let stage: Box<dyn Fn(f32, Mat3A) -> Mat3A> = match self.state {
            TrebuchetState::Stage1 => {
                let Mat3A{x_axis: _, y_axis: Vec3A{x: aw_prime, ..}, ..} = self.stage_1 (dt, mat);
                if self.ground_force(aw_prime) <= 0.0 {
                    self.state = TrebuchetState::Stage2;
                }
                Box::new(|dt: f32, x: Mat3A| self.stage_1(dt, x))
            }

            TrebuchetState::Stage2 => {
                if to_angle(self.v_projectile()) <= consts::FRAC_PI_4 {
                    self.m_proj = 0.01;
                    self.state = TrebuchetState::Stage3;
                }
                Box::new(|dt: f32, x: Mat3A| self.stage_2(dt, x))
            }

            TrebuchetState::Stage3 => {
                Box::new(|dt: f32, x: Mat3A| self.stage_2(dt, x))
            }
        };
        
        let rk4_results = rk5(mat, dt, stage);
        Vec3A {x: self.arm.angle, y: self.weight.angle, z: self.sling.angle}         = rk4_results.x_axis;
        Vec3A {x: self.arm.velocity, y: self.weight.velocity, z:self.sling.velocity} = rk4_results.y_axis;

        self.state == TrebuchetState::Stage3
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

    fn stage_1(&self, dt: f32, mat: Mat3A) -> Mat3A {
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

        let aq = mat.x_axis.x;
        let wq = mat.x_axis.y;
        let sq = mat.x_axis.z;

        let aw = mat.y_axis.x;
        let ww = mat.y_axis.y;
        let sw = mat.y_axis.z;

        #[rustfmt::skip]
        let m = Mat2::from_cols_array(&[
            -mp * lal.powi(2) * (-1.0 + 2.0 * aq.sin() * sq.cos() / (aq + sq).sin()) + ia + iw + ma 
            * cga.powi(2) + mp * lal.powi(2) * aq.sin().powi(2) / (aq + sq).sin().powi(2) + mw 
            * (las.powi(2) + lw.powi(2)+ 2.0 * las * lw * wq.cos()),
            iw + lw * mw * (lw + las * wq.cos()),
            iw + lw * mw * (lw + las * wq.cos()),
            iw + mw * lw.powi(2),
        ]);

        #[rustfmt::skip]
        let r = vec2(
            GRAVITY * cga * ma * aq.sin() + lal * ls * mp * (sq.sin() * (aw + sw).powi(2) 
            + sq.cos() * ((aq + sq).cos() * sw * (sw + 2.0 * aw) / (aq + sq).sin() 
            + ((aq + sq).cos() / (aq + sq).sin() + lal * aq.cos() / (ls * (aq + sq).sin())) 
            * aw.powi(2))) + lal * mp * aq.sin() * (lal * sq.sin() * aw.powi(2) - ls
            * ((aq + sq).cos() * sw * (sw + 2.0 * aw) / (aq  + sq).sin() + ((aq + sq).cos()
            / (aq + sq).sin() + lal * aq.cos() / (ls * (aq + sq).sin())) * aw.powi(2))) 
            / (aq + sq).sin() - GRAVITY * mw * (las * aq.sin() + lw * (aq + wq).sin()) 
            - las * lw * mw * wq.sin() * (aw.powi(2) - (aw + ww).powi(2)),
            -lw * mw * (GRAVITY * (aq + wq) + las * wq.sin() * aw.powi(2))
        );

        let arm_weight_accel = m.inverse().mul_vec2(r);
        #[rustfmt::skip]
        let sw_prime = -(aq + sq).cos() * sw * (sw + 2.0 * aw) / (aq + sq).sin() 
            - ((aq + sq).cos() / (aq + sq).sin() + lal * aq.cos() / (ls * (aq + sq).sin())) 
            * aw.powi(2) - (lal * aq.sin() + ls * (aq + sq).sin()) * arm_weight_accel.x 
            / (ls * (aq + sq).sin());
        let accelerations: Vec3A = arm_weight_accel.extend(sw_prime).into();
                
        mat3a(
            mat.y_axis + accelerations * dt,
            accelerations,
            Vec3A::ZERO,
        )
    }

    fn stage_2(&self, dt: f32, mat: Mat3A) -> Mat3A {
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

        let aq = mat.x_axis.x;
        let wq = mat.x_axis.y;
        let sq = mat.x_axis.z;

        let aw = mat.y_axis.x;
        let ww = mat.y_axis.y;
        let sw = mat.y_axis.z;

        #[rustfmt::skip]
        let m = Mat3A::from_cols_array(&[
            ia + iw + ma * cga.powi(2) + mp * (lal.powi(2) + ls.powi(2) + 2.0 * lal * ls 
            * sq.cos()) + mw * (las.powi(2) + lw.powi(2) + 2.0 * las * lw * wq.cos()),
            iw + lw * mw * (lw + las * wq.cos()),
            ls * mp * (ls + lal * sq.cos()),
            iw + lw * mw * (lw + las * wq.cos()),
            iw + mw * lw.powi(2),
            0.0,
            ls * mp * (ls + lal * sq.cos()),
            0.0,
            mp * ls.powi(2),
        ]);

        #[rustfmt::skip]
        let r = vec3a(
            GRAVITY * cga * ma * aq.sin() + GRAVITY * mp * (lal * aq.sin() + ls * (aq + sq).sin()) 
            - GRAVITY * mw * (las * aq.sin() + lw * (aq + wq).sin()) - lal * ls * mp * sq.sin() 
            * (aw.powi(2) - (aw + sw).powi(2)) - las * lw * mw * wq.sin() * (aw.powi(2) 
            - (aw + ww).powi(2)),
            -lw * mw * (GRAVITY * (aq + wq).sin() + las * wq.sin() * aw.powi(2)),
            ls * mp * (GRAVITY * (aq + sq).sin() - lal * sq.sin() * aw.powi(2)),
        );

        let accelerations = m.inverse().mul_vec3a(r);
        
        mat3a(
            mat.y_axis + accelerations * dt,
            accelerations,
            Vec3A::ZERO
        )
    }
}

#[cfg(test)]
mod test {
    use macroquad::math::Mat3;

    #[test]
    fn mat_mul(){
        let col = [
            1.0, 1.0, 1.0,
            2.0, 2.0, 2.0,
            0.0, 0.0, 0.0,
        ];
        let add = [
            1.0, 1.0, 1.0,
            1.0, 1.0, 1.0,
            0.0, 0.0, 0.0,
        ];
        let m = Mat3::from_cols_array(&col);
        println!("{:?}", m * 2.0 + Mat3::from_cols_array(&add))
    }
}