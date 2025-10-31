use crate::common::constants::PS_PER_SEC;

pub struct ExtDuration {
    pub secs: u64,
    pub ns: u32,
    pub ps: u32,
}
impl ExtDuration {
    pub fn from_ps(total_ps: u64) -> Self {
        let secs = total_ps / PS_PER_SEC;
        let rem_ps = total_ps % PS_PER_SEC;
        let ns = (rem_ps / 1_000) as u32;
        let ps = (rem_ps % 1_000) as u32;
        ExtDuration {
            secs,
            ns,
            ps,
        }
    }

    pub fn to_ps(&self) -> u64 {
        self.secs * PS_PER_SEC + (self.ns as u64) * 1_000 + (self.ps as u64)
    }
}
