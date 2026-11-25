#[derive(Debug, Clone)]
pub enum Interaction {
    Predation {
        predator_id: usize,
        prey_id: usize,
    },
    Competition {
        competitor1_id: usize,
        competitor2_id: usize,
        resource: String,
    },
    Mutualism {
        participant1_id: usize,
        participant2_id: usize,
        benefit: String,
    },
    Parasitism {
        parasite_id: usize,
        host_id: usize,
    },
}