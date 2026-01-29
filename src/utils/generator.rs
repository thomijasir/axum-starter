use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const WORKER_ID: u64 = 1; // Unique ID for this instance (0-1023 generally)
const EPOCH: u64 = 1609459200; // January 1, 2021 (Unix timestamp in seconds)
static SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Snowflake ID using default configurations - Like Twitter.
pub fn id() -> u64 {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed) & 4095; // 4095 = 2^12 - 1 (12 bits)

    (timestamp - EPOCH) << 22 |  // 42 bits for the timestamp
    WORKER_ID << 12 |     // 10 bits for worker ID
    sequence // 12 bits for the sequence number
}

// Time-based ID generator
pub fn time_id(len: usize) -> String {
    // Get current timestamp in milliseconds
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    // Perform the same modulo operation as JS version
    let mixed_number = now % 60466176;
    // Convert to base 36 string (equivalent to toString(36) in JS)
    let id = format!("{:x}", mixed_number);
    // Pad with 'a' characters if needed
    let padding = if len > id.len() {
        "a".repeat(len - id.len())
    } else {
        String::new()
    };
    format!("{}{}", padding, id)
}

pub struct Random {
    state: u64,
}

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}

impl Random {
    pub fn new() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Random { state: seed }
    }
    fn next(&mut self) -> u64 {
        // LCG parameters
        const A: u64 = 6364136223846793005;
        const C: u64 = 1442695040888963407;

        self.state = self.state.wrapping_mul(A).wrapping_add(C);
        self.state
    }
    pub fn gen_range(
        &mut self,
        min: u32,
        max: u32,
    ) -> u32 {
        let range = (max - min + 1) as u64;
        (self.next() % range) as u32 + min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let id = id();
        println!("Generated Snowflake ID: {}", id);
        assert!(id > 0); // Very basic check
    }

    #[test]
    fn test_unique_ids() {
        use std::collections::HashSet;
        let num_ids = 1000;
        let mut ids = HashSet::new();

        for _ in 0..num_ids {
            let id = id();
            assert!(ids.insert(id), "Duplicate ID generated: {}", id);
        }
        assert_eq!(ids.len(), num_ids, "Not all IDs were unique");
    }

    #[test]
    fn test_generate_multiple_ids() {
        // Generate 5 ids to check they are sequential
        let mut ids: Vec<u64> = Vec::new();
        for _ in 0..5 {
            ids.push(id());
        }

        // Check that they increase
        for i in 1..ids.len() {
            assert!(ids[i] > ids[i - 1]);
        }
    }
}
