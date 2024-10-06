use crate::helpers::{add_hashes_async_parts, add_hashes_final, add_hashes_parts};

pub const HASH_LENGTH_U64: usize = 64;
pub const HASH_SPLIT_SIZE_U64: usize = 32;

pub fn aggregate_hashes<T>(hashes: T) -> [u64; HASH_LENGTH_U64]
where
    T: AsRef<[[u64; HASH_LENGTH_U64]]>,
{
    let hashes = hashes.as_ref();
    let mut res = [0u64; HASH_LENGTH_U64];
    for hash in hashes {
        append_hash(&mut res, hash);
    }
    res
}

pub fn aggregate_hashes_parts<T>(hashes: T) -> [u64; HASH_LENGTH_U64]
where
    T: AsRef<[[u64; HASH_LENGTH_U64]]>,
{
    let hashes = hashes.as_ref();
    let mut res = [0u64; HASH_LENGTH_U64];

    for hash in hashes {
        res = add_hashes_parts::<HASH_LENGTH_U64, HASH_SPLIT_SIZE_U64>(&mut res, hash);
    }

    return res;
}

pub fn aggregate_hashes_halves<T>(hashes: T) -> [u64; HASH_LENGTH_U64]
where
    T: AsRef<[[u64; HASH_LENGTH_U64]]>,
{
    let hashes = hashes.as_ref();
    let mut res = [0u64; HASH_LENGTH_U64];

    for hash in hashes.as_ref() {
        res = add_hashes_final::<HASH_LENGTH_U64, HASH_SPLIT_SIZE_U64>(&mut res, hash);
    }

    return res;
}

pub async fn aggregate_hashes_async_parts<T>(hashes: T) -> [u64; HASH_LENGTH_U64]
where
    T: AsRef<[[u64; HASH_LENGTH_U64]]>,
{
    let hashes = hashes.as_ref();
    let mut res = [0u64; HASH_LENGTH_U64];

    for hash in hashes {
        res = add_hashes_async_parts::<HASH_LENGTH_U64, HASH_SPLIT_SIZE_U64>(&mut res, hash).await;
    }

    return res;
}

fn append_hash(base_hash: &mut [u64; HASH_LENGTH_U64], hash: &[u64; HASH_LENGTH_U64]) {
    let mut carry = 0u64;

    for (base, &h) in base_hash.iter_mut().zip(hash.iter()) {
        let (sum, overflow) = base.overflowing_add(h);
        let (sum_with_carry, carry_overflow) = sum.overflowing_add(carry);
        *base = sum_with_carry;
        carry = (overflow | carry_overflow) as u64;
    }
}
