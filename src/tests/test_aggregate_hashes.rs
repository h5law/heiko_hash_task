use crate::aggregator;
use crate::helpers;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_aggregate_hashes_returns_correct_hash_sum() {
        let hashes = helpers::generate_random_hashes();
        let res_1 = aggregator::aggregate_hashes(&hashes);
        let res_2 = aggregator::aggregate_hashes_parts(&hashes);
        let res_3 = aggregator::aggregate_hashes_async_parts(&hashes).await;

        let mut expected_1 = [0u64; aggregator::HASH_LENGTH_U64];
        let mut expected_2 = [0u64; aggregator::HASH_LENGTH_U64];
        let mut expected_3 = [0u64; aggregator::HASH_LENGTH_U64];
        for i in 0..hashes.len() {
            expected_1 = helpers::add_hashes(&expected_1, &hashes[i]);
            expected_2 = helpers::add_hashes_parts::<
                { aggregator::HASH_LENGTH_U64 },
                { aggregator::HASH_SPLIT_SIZE_U64 },
            >(&expected_2, &hashes[i]);
            expected_3 = helpers::add_hashes_async_parts::<
                { aggregator::HASH_LENGTH_U64 },
                { aggregator::HASH_SPLIT_SIZE_U64 },
            >(&expected_3, &hashes[i])
            .await;
        }

        assert_eq!(
            res_1, expected_1,
            "Aggregating multiple hashes should return a correct hash sum (1 = 1)"
        );
        assert_eq!(
            res_1, expected_2,
            "Aggregating multiple hashes should return a correct hash sum (1 = 2)"
        );
        assert_eq!(
            res_1, expected_3,
            "Aggregating multiple hashes should return a correct hash sum (1 = 2)"
        );
        assert_eq!(
            res_2, expected_1,
            "Aggregating multiple hashes should return a correct hash sum (2 = 1)"
        );
        assert_eq!(
            res_2, expected_2,
            "Aggregating multiple hashes should return a correct hash sum (2 = 2)"
        );
        assert_eq!(
            res_2, expected_3,
            "Aggregating multiple hashes should return a correct hash sum (1 = 2)"
        );
        assert_eq!(
            res_3, expected_1,
            "Aggregating multiple hashes should return a correct hash sum (1 = 2)"
        );
        assert_eq!(
            res_3, expected_2,
            "Aggregating multiple hashes should return a correct hash sum (1 = 2)"
        );
        assert_eq!(
            res_3, expected_3,
            "Aggregating multiple hashes should return a correct hash sum (1 = 2)"
        );
    }
}
