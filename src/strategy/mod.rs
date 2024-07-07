use crate::network::{BalancingStrategy, Target};

pub struct RoundRobin {
    current: usize,
    targets: Vec<Target>,
}

impl RoundRobin {
    pub fn new(targets: Vec<Target>) -> Self {
        RoundRobin { current: 0, targets }
    }
}

impl BalancingStrategy for RoundRobin {
    fn next(&mut self) -> Option<&Target> {
        let target = self.targets.get(self.current);

        if let Some(_) = target {
            self.current = (self.current + 1) % self.targets.len();
        }

        target
    }
}

