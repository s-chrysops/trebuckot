use super::*;

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
}

impl TrebuchetArm {
    fn new(parameters: (f32, f32, f32), common_triangle: f32) -> Self {
        Self {
            long_length:  parameters.0,
            short_length: parameters.1,
            center:       (parameters.0 + parameters.1) / 2.0 - parameters.1,
            mass:         parameters.2,
            inertia:      parameters.2 * (parameters.0 + parameters.1).powi(2) / 12.0,
            angle:        consts::PI - common_triangle.acos(),
        }
    }
}

struct TrebuchetWeight {
    length:  f32,
    mass:    f32,
    inertia: f32,
    angle:   f32,
}

impl TrebuchetWeight {
    fn new(parameters: (f32, f32), common_triangle: f32) -> Self {
        Self {
            length:  parameters.0,
            mass:    parameters.1,
            inertia: 1.0,
            angle:   common_triangle.acos() - consts::PI,
        }
    }
}

struct TrebuchetSling {
    length: f32,
    angle:  f32,
}

impl TrebuchetSling {
    fn new(length: f32, common_triangle: f32) -> Self {
        Self {
            length,
            angle: consts::PI - common_triangle.asin(),
        }
    }
}

//#[derive(Default)]
pub struct Trebuchet {
    pub position: I64Vec2,
    height:   f32,
    m_proj:   f32,

    arm:    TrebuchetArm,
    weight: TrebuchetWeight,
    sling:  TrebuchetSling,

    d_arm:    f32,
    d_sling:  f32,
    d_weight: f32,

    pub state: TrebuchetState,
}

impl Trebuchet {
    pub fn new(
        position: I64Vec2,
        height: f32,
        arm: (f32, f32, f32),
        weight: (f32, f32),
        sling: f32,
        m_proj: f32,
    ) -> Self {
        let common_triangle = height / arm.0;
        Self {
            position,
            height,
            m_proj,
            arm: TrebuchetArm::new(arm, common_triangle),
            weight: TrebuchetWeight::new(weight, common_triangle),
            sling: TrebuchetSling::new(sling, common_triangle),
            d_arm: 0.0,
            d_sling: 0.0,
            d_weight: 0.0,
            state: TrebuchetState::Stage1,
        }
    }

    fn armsling_point(&self) -> Vec2 {
        vec2(
            -self.arm.long_length * self.arm.angle.sin(),
            self.arm.long_length * self.arm.angle.cos(),
        )
    }

    fn armweight_point(&self) -> Vec2 {
        vec2(
            self.arm.short_length * self.arm.angle.sin(),
            -self.arm.short_length * self.arm.angle.cos(),
        )
    }

    fn sling_point(&self) -> Vec2 {
        vec2(
            (-self.arm.long_length * self.arm.angle.sin())
                - (self.sling.length * (self.arm.angle + self.sling.angle).sin()),
            (self.arm.long_length * self.arm.angle.cos())
                + (self.sling.length * (self.arm.angle + self.sling.angle).cos()),
        )
    }

    fn weight_point(&self) -> Vec2 {
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
            -self.arm.long_length * self.arm.angle.cos() * self.d_arm 
                - self.sling.length * (self.arm.angle + self.sling.angle).cos() * (self.d_arm + self.d_sling),
            -self.arm.long_length * self.arm.angle.sin() * self.d_arm 
                - self.sling.length * (self.arm.angle + self.sling.angle).sin() * (self.d_arm + self.d_sling)
        )
    }

    pub fn draw(&self, game: &Game) {
        let base = i64coords_to_screen(game, self.position);
        let pivot = vec2(base.x, base.y + self.height);
        let arm_s = self.armsling_point() + pivot;
        let arm_w = self.armweight_point() + pivot;
        let s = self.sling_point() + pivot;
        let w = self.weight_point() + pivot;

        draw_line(base.x, base.y, pivot.x, pivot.y, 0.1, BROWN);
        draw_line(arm_s.x, arm_s.y, arm_w.x, arm_w.y, 0.1, YELLOW);
        draw_line(s.x, s.y, arm_s.x, arm_s.y, 0.01, GRAY);
        draw_line(w.x, w.y, arm_w.x, arm_w.y, 0.1, BLACK);

        // let p = self.v_projectile() + s;
        // draw_line(s.x, s.y, p.x, p.y, 0.05, PINK);
    }

    pub fn run(&mut self, dt: f32) {
        match self.state {
            TrebuchetState::Stage1 => {
                self.stage_1(dt);
            }
            TrebuchetState::Stage2 => {
                self.stage_2(dt);
                if to_angle(self.v_projectile()) <= consts::FRAC_PI_4 {
                    self.m_proj = 0.01;
                    self.state = TrebuchetState::Stage3;
                }
            }
            TrebuchetState::Stage3 => {
                self.stage_2(dt);
            }
        }
    }

    fn stage_1(&mut self, dt: f32) {
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

        self.arm.angle += self.d_arm * dt;
        self.weight.angle += self.d_weight * dt;
        self.sling.angle += self.d_sling * dt;

        let aq = self.arm.angle;
        let wq = self.weight.angle;
        let sq = self.sling.angle;

        #[rustfmt::skip]
        let m11 = -mp * lal.powi(2)
            * (-1.0 + 2.0 * aq.sin() * sq.cos() / (aq + sq).sin())
            + ia + iw + ma * cga.powi(2) + mp
            * lal.powi(2) * aq.sin().powi(2) / (aq + sq).sin().powi(2)
            + mw * (las.powi(2) + lw.powi(2)+ 2.0 * las * lw * wq.cos());
        let m12 = iw + lw * mw * (lw + las * wq.cos());
        let m21 = iw + lw * mw * (lw + las * wq.cos());
        let m22 = iw + mw * lw.powi(2);

        #[rustfmt::skip]
        let r1 = GRAVITY * cga * ma * aq.sin() + lal * ls * mp * (sq.sin()
            * (self.d_arm + self.d_sling).powi(2) + sq.cos()
            * ((aq + sq).cos() * self.d_sling * (self.d_sling + 2.0 * self.d_arm)
            / (aq + sq).sin() + ((aq + sq).cos()
            / (aq + sq).sin() + lal * aq.cos()
            / (ls * (aq + sq).sin())) * self.d_arm.powi(2)))
            + lal * mp * aq.sin() * (lal * sq.sin() * self.d_arm.powi(2) - ls
            * ((aq + sq).cos() * self.d_sling * (self.d_sling + 2.0 * self.d_arm)
            / (aq  + sq).sin() + ((aq + sq).cos()
            / (aq + sq).sin() + lal * aq.cos() / (ls * (aq
            + sq).sin())) * self.d_arm.powi(2))) / (aq + sq).sin()
            - GRAVITY * mw * (las * aq.sin() + lw * (aq + wq).sin())
            - las * lw * mw * wq.sin() * (self.d_arm.powi(2) 
            - (self.d_arm + self.d_weight).powi(2));

        #[rustfmt::skip]
        let r2 = -lw * mw * (GRAVITY * (aq + wq)
            + las * wq.sin() * self.d_arm.powi(2));

        let arm_prime = (r1 * m22 - r2 * m12) / (m11 * m22 - m12 * m21);
        let weight_prime = -(r1 * m21 - r2 * m11) / (m11 * m22 - m12 * m21);
        #[rustfmt::skip]
        let sling_prime = - (aq + sq).cos() * self.d_sling
            * (self.d_sling + 2.0 * self.d_arm) / (aq + sq).sin()
            - ((aq + sq).cos() / (aq + sq).sin()
            + lal * aq.cos() / (ls * (aq + sq).sin()))
            * self.d_arm.powi(2) - (lal * aq.sin() + ls * (aq + sq).sin())
            * arm_prime / (ls * (aq + sq).sin());

        self.d_arm += arm_prime * dt;
        self.d_weight += weight_prime * dt;
        self.d_sling += sling_prime * dt;

        #[rustfmt::skip]
        let fy = self.m_proj * (GRAVITY + (ls * ((aq + sq).cos() * self.d_sling 
            * (self.d_sling + 2.0 * self.d_arm) / (aq + sq).sin() + ((aq + sq).cos() 
            / (aq + sq).sin() + lal * aq.cos() / (ls * (aq + sq).sin())) * self.d_arm.powi(2)) 
            - lal * sq.sin() * self.d_arm.powi(2) - lal * (sq.cos() - aq.sin() 
            / (aq + sq).sin()) * arm_prime) / (aq + sq).sin());
        if fy <= 0.0 { self.state = TrebuchetState::Stage2 };
    }

    fn stage_2(&mut self, dt: f32) {
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

        self.arm.angle += self.d_arm * dt;
        self.weight.angle += self.d_weight * dt;
        self.sling.angle += self.d_sling * dt;

        let aq = self.arm.angle;
        let wq = self.weight.angle;
        let sq = self.sling.angle;

        #[rustfmt::skip]
        let m11= ia + iw + ma * cga.powi(2) + mp 
            * (lal.powi(2) + ls.powi(2) + 2.0 * lal * ls * sq.cos()) 
            + mw * (las.powi(2) + lw.powi(2) + 2.0 * las * lw * wq.cos());
        let m12= iw + lw * mw * (lw + las * wq.cos());
        let m13= ls * mp * (ls + lal * sq.cos());
        let m21= iw + lw * mw * (lw + las * wq.cos());
        let m22= iw + mw * lw.powi(2);
        let m31= ls * mp * (ls + lal * sq.cos());
        let m33= mp * ls.powi(2);

        #[rustfmt::skip]
        let r1= GRAVITY * cga * ma * aq.sin() + GRAVITY * mp 
            * (lal * aq.sin() + ls * (aq + sq).sin()) - GRAVITY * mw 
            * (las * aq.sin() + lw * (aq + wq).sin()) - lal * ls * mp * sq.sin() 
            * (self.d_arm.powi(2) - (self.d_arm + self.d_sling).powi(2)) 
            - las * lw * mw * wq.sin() 
            * (self.d_arm.powi(2) - (self.d_arm + self.d_weight).powi(2));
        let r2= -lw * mw * (GRAVITY * (aq + wq).sin() + las * wq.sin() * self.d_arm.powi(2));
        let r3= ls * mp * (GRAVITY * (aq + sq).sin() - lal * sq.sin() * self.d_arm.powi(2));

        let arm_prime = -(r1 * m22 * m33 - r2 * m12 * m33 - r3 * m13 * m22) 
            / (m13 * m22 * m31 - m33 * (m11 * m22 - m12 * m21));
        let weight_prime = (r1 * m21 * m33 - r2 * (m11 * m33 - m13 * m31) - r3 * m13 * m21) 
            / (m13 * m22 * m31 - m33 * (m11 * m22 - m12 * m21));
        let sling_prime = (r1 * m22 * m31 - r2 * m12 * m31 - r3 * (m11 * m22 - m12 * m21)) 
            / (m13 * m22 * m31 - m33 * (m11 * m22 - m12 * m21));

        self.d_arm += arm_prime * dt;
        self.d_weight += weight_prime * dt;
        self.d_sling += sling_prime * dt;
    }
}

/*
fn prime(d: f32) -> impl Fn(f32, f32) -> f32 {
    |t, y| d * t
}
*/
pub fn rk4<T>(t_i: f32, y_i: f32, dt: f32, f: T) -> f32
where
    T: Fn(f32, f32) -> f32,
{
    let k1 = f(t_i, y_i);
    let k2 = f(t_i + 0.5 * dt, y_i + k1 * 0.5 * dt);
    let k3 = f(t_i + 0.5 * dt, y_i + k2 * 0.5 * dt);
    let k4 = f(t_i + dt, y_i + k3 * dt);
    y_i + dt / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4)
}
