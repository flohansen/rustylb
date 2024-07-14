use crate::network::{BalancingStrategy, Target};

/// Implements the `BalancingStrategy` trait for load balancing among targets. It iterates over the
/// targets in a cyclical manner.
pub struct RoundRobin {
    /// The index of the currently active target.
    current: usize,
    /// The targets, which are used for load balancing
    targets: Vec<Target>,
}

impl RoundRobin {
    /// Creates a new `RoundRobin` instance with the provided targets.
    ///
    /// # Arguments
    ///
    /// * `targets` - A vector of `Target` instances to balance load across.
    pub fn new(targets: Vec<Target>) -> Self {
        RoundRobin { current: 0, targets }
    }
}

impl BalancingStrategy for RoundRobin {
    /// Returns the next target in the round-robin cycle.
    ///
    /// If no targets are available, returns `None`.
    fn next(&mut self) -> Option<&Target> {
        let target = self.targets.get(self.current);

        if target.is_some() {
            self.current = (self.current + 1) % self.targets.len();
        }

        target
    }
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use super::*;

    #[test]
    fn should_rotate_targets_in_finite_order() {
        // given
        let targets = vec![
            Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3000),
            Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3001),
            Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3002),
        ];

        let mut strategy = RoundRobin::new(targets);

        // when
        // then
        let expected_targets = vec![
            Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3000),
            Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3001),
            Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3002),
            Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3000),
            Target::new(IpAddr::V4([127, 0, 0, 1].into()), 3001),
        ];

        for i in 0..5 {
            let target = strategy.next().unwrap();
            assert_eq!(expected_targets[i].ip(), target.ip());
            assert_eq!(expected_targets[i].port(), target.port());
        }
    }

}
