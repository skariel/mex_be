use std::u64;
use std::cmp::Ordering;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct SessionID(u64, u64);

impl SessionID {
    pub fn from_string(s: &str) -> SessionID {
        let splitted: Vec<&str> = s.split(' ').collect();
        SessionID(u64::from_str_radix(splitted[0], 16).unwrap(), u64::from_str_radix(splitted[1], 16).unwrap())
    }
    pub fn to_string(&self) -> String {
        format!("{}{}", self.0.to_string(), self.1.to_string())
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
