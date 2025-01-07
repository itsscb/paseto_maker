use super::Version;

pub struct V4;
impl V4 {
    pub const NAME: &'static str = "v4";
}
impl Version for V4 {}
