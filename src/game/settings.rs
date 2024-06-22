pub struct Settings {
    pub autosave: bool,
    pub scale: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { autosave: true, scale: 1.0 }
    }
}
