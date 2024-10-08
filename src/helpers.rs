use crate::aggregator;
use rand::Rng;
use tokio::try_join;

pub const MIN_HASHES: usize = 0;
pub const MAX_HASHES: usize = 100;

pub fn generate_random_hashes() -> Vec<[u64; aggregator::HASH_LENGTH_U64]> {
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(MIN_HASHES..=MAX_HASHES);
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push(generate_random_hash());
    }
    vec
}

fn generate_random_hash() -> [u64; aggregator::HASH_LENGTH_U64] {
    generate_random_u64_array::<{ aggregator::HASH_LENGTH_U64 }>()
}

fn generate_random_u64_array<const N: usize>() -> [u64; N] {
    let mut rng = rand::thread_rng();
    let mut arr = [0u64; N];
    for elem in &mut arr {
        *elem = rng.gen();
    }
    arr
}

pub fn add_hashes<const N: usize>(hash_a: &[u64; N], hash_b: &[u64; N]) -> [u64; N] {
    let mut res = [0u64; N];
    let mut carry = false;

    for i in 0..N {
        let (sum, overflow_1) = hash_a[i].overflowing_add(hash_b[i]);
        let (sum_with_overflow, overflow_2) = sum.overflowing_add(carry as u64);
        res[i] = sum_with_overflow;
        carry = overflow_1 | overflow_2;
    }

    res
}

pub fn add_hashes_parts<const N: usize, const SPLIT: usize>(
    hash_a: &[u64; N],
    hash_b: &[u64; N],
) -> [u64; N] {
    let mut res = [0u64; N];
    let mut carry = false;
    let mut lower = [0u64; SPLIT];
    let mut higher = [0u64; SPLIT];
    for i in 0..SPLIT {
        let (sum, flow_1) = hash_a[i].overflowing_add(hash_b[i]);
        let (sum_with_flow, flow_2) = sum.overflowing_add(carry as u64);
        lower[i] = sum_with_flow;
        carry = flow_1 | flow_2;
    }
    for i in SPLIT..N {
        let (sum, flow_1) = hash_a[i].overflowing_add(hash_b[i]);
        let (sum_with_flow, flow_2) = sum.overflowing_add(carry as u64);
        higher[i - SPLIT] = sum_with_flow;
        carry = flow_1 | flow_2;
    }
    for (idx, val) in lower.into_iter().chain(higher.into_iter()).enumerate() {
        res[idx] = val;
    }
    return res;
}

pub fn add_hashes_final<const N: usize, const SPLIT: usize>(
    hash_a: &[u64; N],
    hash_b: &[u64; N],
) -> [u64; N] {
    let mut res = [0u64; N];
    let mut hashes_a = hash_a.clone();
    let mut hashes_b = hash_b.clone();
    let (lower_a, higher_a) = hashes_a.split_at_mut(SPLIT);
    let (lower_b, higher_b) = hashes_b.split_at_mut(SPLIT);
    let mut carry_l1: bool = false;
    let mut carry_h1: bool = false;

    // Initialise a carries array to store the carry digits at their indexes.
    for i in 0..SPLIT {
        // Sum the first halves of each hash being summed
        let (sum_l1, lo_flow_1) = lower_a[i].overflowing_add(lower_b[i]);
        let (sum_with_flow_l1, lo_flow_2) = sum_l1.overflowing_add(carry_l1 as u64);

        // These will take the carries from the previous steps later.
        let (sum_h1, hi_flow_1) = higher_a[i].overflowing_add(higher_b[i]);
        let (sum_with_flow_h1, hi_flow_2) = sum_h1.overflowing_add(carry_h1 as u64);

        // While here set the relevant result buts before finalising with carries.
        res[i] = sum_with_flow_l1;
        res[i + SPLIT] = sum_with_flow_h1;

        // Sort out the carries into the vector.
        carry_l1 = lo_flow_1 | lo_flow_2;
        carry_h1 = hi_flow_1 | hi_flow_2;
    }

    for i in SPLIT..N {
        if carry_l1 {
            let (sum, flow_1) = res[i].overflowing_add(1);
            let (sum_with_flow, flow_1) = sum.overflowing_add(flow_1 as u64);
            res[i] = sum_with_flow;
            carry_l1 = flow_1;
            continue;
        }
        break;
    }

    return res;
}

pub async fn add_hashes_async_parts<const N: usize, const SPLIT: usize>(
    hash_a: &[u64; N],
    hash_b: &[u64; N],
) -> [u64; N] {
    let mut res = [0u64; N];
    let mut carry_1 = false;
    let mut carry_2 = false;
    let mut lower = [0u64; SPLIT];
    let mut higher = [0u64; SPLIT];
    try_join!(
        async {
            for i in 0..SPLIT {
                let (sum, flow_1) = hash_a[i].overflowing_add(hash_b[i]);
                let (sum_with_flow, flow_2) = sum.overflowing_add(carry_1 as u64);
                lower[i] = sum_with_flow;
                carry_1 = flow_1 | flow_2;
            }
            Ok::<[u64; SPLIT], usize>(lower)
        },
        async {
            for i in SPLIT..N {
                let (sum, flow_1) = hash_a[i].overflowing_add(hash_b[i]);
                let (sum_with_flow, flow_2) = sum.overflowing_add(carry_2 as u64);
                higher[i - SPLIT] = sum_with_flow;
                carry_2 = flow_1 | flow_2;
            }
            Ok::<[u64; SPLIT], usize>(higher)
        }
    )
    .expect("failed to sum splits");
    for i in 0..SPLIT {
        if !carry_1 {
            break;
        }
        let (sum, flow_1) = higher[i].overflowing_add(1);
        higher[i] = sum;
        carry_1 = flow_1;
    }
    for (idx, val) in lower.into_iter().chain(higher.into_iter()).enumerate() {
        res[idx] = val;
    }
    return res;
}
