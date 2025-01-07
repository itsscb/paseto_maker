use super::Purpose;

pub struct Public;
impl Public {
    pub const NAME: &'static str = "public";
}

impl Purpose for Public {}
