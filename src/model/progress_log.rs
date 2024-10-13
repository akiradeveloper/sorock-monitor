use super::*;

pub struct ProgressLog {
    log: BTreeMap<Instant, u64>,
}
impl ProgressLog {
    pub fn new() -> Self {
        Self {
            log: BTreeMap::new(),
        }
    }
    pub fn get_range(&self, start: Instant, end: Instant) -> BTreeMap<Instant, u64> {
        self.log.range(start..=end).map(|(&k, &v)| (k, v)).collect()
    }
    pub fn test() -> Self {
        let mut log = BTreeMap::new();
        let now = Instant::now();
        for i in 0..100000 {
            log.insert(now + Duration::from_secs(i), i * i);
        }
        Self { log }
    }
}
