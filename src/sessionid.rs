use std::cmp::Ordering;

use rand;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct SessionID(u64, u64);

impl SessionID {
    pub fn new() -> SessionID {
        SessionID(rand::random::<u64>(), rand::random::<u64>())
    }
}

impl Ord for SessionID {
    fn cmp(&self, other: &SessionID) -> Ordering {
        let o1 = self.0.cmp(&other.0);
        if o1 != Ordering::Equal {
            return o1;
        }
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for SessionID {
    fn partial_cmp(&self, other: &SessionID) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}