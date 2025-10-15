use crate::Summarizable;

pub struct Body<'a> {
    pub entity: &'a dyn Summarizable,
}

impl<'a> Body<'a> {
    pub fn new(entity: &'a impl Summarizable) -> Self {
        Self { entity: { entity } }
    }

    pub fn display_summary(&self) {
        println!("{}", self.entity.summary());
    }
}