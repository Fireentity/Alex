#[derive(Serialize, Deserialize, Debug)]
struct WindowConfig {
    width: i32,
    height: i32,
    title: str,
}

impl WindowConfig {

    pub fn width(&self) -> &i32 {
        &self.width
    }

    pub fn height(&self) -> &i32 {
        &self.height
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}