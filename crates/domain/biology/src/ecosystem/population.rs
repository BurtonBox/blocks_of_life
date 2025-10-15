use crate::anatomy::body::Body;
use crate::ecosystem::environment::Environment;
use crate::ecosystem::interactions::Interaction;

pub struct Population<'a> {
    pub members: Vec<Body<'a>>,
    pub environment: Environment,
    pub interactions: Vec<Interaction>,
}

impl<'a> Population<'a> {
    pub fn new(environment: Environment) -> Self {
        Self {
            members: Vec::new(),
            environment,
            interactions: Vec::new(),
        }
    }

    pub fn add_member(&mut self, member: Body<'a>) {
        self.members.push(member);
    }

    pub fn add_interaction(&mut self, interaction: Interaction) {
        self.interactions.push(interaction);
    }

    pub fn size(&self) -> usize {
        self.members.len()
    }
}