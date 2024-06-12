#[allow(dead_code)]
#[derive(Default)]
pub struct Resources {
    pub research:  u32,
    pub rocks:     u32,
    pub beans:     u32,
    pub antibeans: u32,
}

// impl Default for Resources {
//     fn default() -> Self {
//         Resources { research: 0, rocks: 69, beans: 1337, antibeans: 420 }
//     }
// }

impl Resources {
    pub fn as_vec(&self) -> Vec<u32> {
        vec![self.research, self.rocks, self.beans, self.antibeans]
    }
}
