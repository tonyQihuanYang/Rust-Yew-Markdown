#[derive(PartialEq, Clone, Debug)]
pub struct Setting {
    pub is_show_preview: bool,
}
impl Setting {
    pub fn new() -> Self {
        Self {
            is_show_preview: true,
        }
    }
}
