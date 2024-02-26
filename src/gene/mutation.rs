use rand::prelude::*;

pub enum MutationType {
    DELETE,
    REVERSAL,
    DUPLICATION,
    SHIFT,
    NEW,
}

impl MutationType {
    pub fn get_mutation_type() -> MutationType {
        match thread_rng().gen_range(0, 5) {
            0 => MutationType::DELETE,
            1 => MutationType::REVERSAL,
            2 => MutationType::DUPLICATION,
            3 => MutationType::SHIFT,
            4 => MutationType::NEW,
            _ => panic!("Not suppose to reach here"),
        }
    }
}
