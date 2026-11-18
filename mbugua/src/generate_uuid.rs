use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn random_u64() -> u64 {
    let mut hasher = RandomState::new().build_hasher();
    // If the clock is before the epoch, fall back to 0: the random state
    // and counter below still keep the ids unique.
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    hasher.write_u128(nanos);
    hasher.write_u64(COUNTER.fetch_add(1, Ordering::Relaxed));
    hasher.finish()
}

pub fn generate_uuid() -> String {
    let n = ((random_u64() as u128) << 64) | random_u64() as u128;

    // Set the version (4) and variant (RFC 4122) bits
    let n = (n & !(0xf_u128 << 76)) | (0x4_u128 << 76);
    let n = (n & !(0x3_u128 << 62)) | (0x2_u128 << 62);

    format!(
        "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        (n >> 96) as u32,
        (n >> 80) as u16,
        (n >> 64) as u16,
        (n >> 48) as u16,
        n & 0xffff_ffff_ffff,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn has_uuid_v4_shape() {
        let id = generate_uuid();
        assert_eq!(id.len(), 36);

        let parts: Vec<&str> = id.split('-').collect();
        let lengths: Vec<usize> = parts.iter().map(|p| p.len()).collect();
        assert_eq!(lengths, [8, 4, 4, 4, 12]);

        assert!(parts[2].starts_with('4'), "version nibble must be 4");
        assert!(
            "89ab".contains(&parts[3][..1]),
            "variant must be 8, 9, a or b"
        );
    }

    #[test]
    fn ids_are_unique() {
        let ids: HashSet<String> = (0..1000).map(|_| generate_uuid()).collect();
        assert_eq!(ids.len(), 1000);
    }
}
