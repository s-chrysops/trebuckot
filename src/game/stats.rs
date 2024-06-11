
pub struct Stat {
    pub field: String,
    pub value: f32,
    pub unit:  String,
}

#[derive(Default)]
pub struct Stats {
    pub time:         f32,
    pub distance:     f32,
    pub max_altitude: f32,
    pub max_speed:    f32,
}

impl Stats {
    pub fn as_vec(&self) -> Vec<Stat> {
        vec![
            Stat {field: "Time".to_string(), value: self.time, unit: "s".to_string()},
            Stat {field: "Distance".to_string(), value: self.distance, unit: "m".to_string()},
            Stat {field: "Max Altitude".to_string(), value: self.max_altitude, unit: "m".to_string()},
            Stat {field: "Max Speed".to_string(), value: self.max_speed, unit: "m/s".to_string()},
        ]
    }
    pub fn crunch(&self) -> u32 {
        ((self.distance * 0.1) + (self.max_altitude * 0.3) + (self.max_speed * 0.6)) as u32
    }
}