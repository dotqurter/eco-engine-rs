
pub struct Application {
    pub name: String,
    pub build: String,
    pub fullscreen: bool,
    pub windowsize: (u32, u32),
}

impl Default for Application {
    fn default() -> Self {
        Application {
            name: String::from("Default Game"),
            build: String::from("0.0.1"),
            fullscreen: false,
            windowsize: (800,600),
        }
    }
}

impl Application {
    
}

