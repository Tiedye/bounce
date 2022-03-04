#[derive(PartialEq, PartialOrd)]
pub struct Orderedf32(pub f32);

impl Ord for Orderedf32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Eq for Orderedf32 {}
