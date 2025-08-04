mod find_missing_tests {
    use xortrick::{find_missing, find_missing_iter, find_missing_opt};

    #[test]
    fn test_missing_middle() {
        let total = vec![1, 2, 3, 4, 5];
        let input = vec![1, 2, 3, 5];
        assert_eq!(find_missing(&input, &total), 4);
    }

    #[test]
    fn test_missing_first() {
        let total = vec![1, 2, 3, 4, 5];
        let input = vec![2, 3, 4, 5];
        assert_eq!(find_missing(&input, &total), 1);
        assert_eq!(find_missing_opt(&input, 5), 1);
        assert_eq!(find_missing_iter(&input, 5), 1);
    }

    #[test]
    fn test_missing_last() {
        let total = vec![1, 2, 3, 4, 5];
        let input = vec![1, 2, 3, 4];
        assert_eq!(find_missing(&input, &total), 5);
        assert_eq!(find_missing_opt(&input, 5), 5);
        assert_eq!(find_missing_iter(&input, 5), 5);
    }

    #[test]
    fn test_large_case() {
        let total: Vec<i32> = (1..=1000).collect();
        let input: Vec<i32> = (1..=1000).filter(|&x| x != 789).collect();
        assert_eq!(find_missing(&input, &total), 789);
        assert_eq!(find_missing_opt(&input, 1000), 789);
        assert_eq!(find_missing_iter(&input, 1000), 789);
    }
}

mod find_missing_2_tests {
    use proptest::prelude::*;
    use xortrick::find_missing_2;

    #[test]
    fn test_missing_two_numbers_small() {
        // total = [1,2,3,4,5], input = [1,2,4]
        // missing = {3,5}
        let total = vec![1, 2, 3, 4, 5];
        let input = vec![1, 2, 4];
        let (u, v) = find_missing_2(&input, &total);
        let mut missing = [u, v];
        missing.sort();
        assert_eq!(missing, [3, 5]);
    }

    #[test]
    fn test_missing_first_and_last() {
        // total = [1,2,3,4,5,6], input = [2,3,4,5]
        // missing = {1,6}
        let total = vec![1, 2, 3, 4, 5, 6];
        let input = vec![2, 3, 4, 5];
        let (u, v) = find_missing_2(&input, &total);
        let mut missing = [u, v];
        missing.sort();
        assert_eq!(missing, [1, 6]);
    }

    #[test]
    fn test_missing_middle_numbers() {
        // total = [1..=8], input = [1,2,4,5,6,8]
        // missing = {3,7}
        let total = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let input = vec![1, 2, 4, 5, 6, 8];
        let (u, v) = find_missing_2(&input, &total);
        let mut missing = [u, v];
        missing.sort();
        assert_eq!(missing, [3, 7]);
    }

    #[test]
    fn test_large_case() {
        let n = 1000;
        let total: Vec<i32> = (1..=n).collect();
        let missing = [123, 987];
        let input: Vec<i32> = (1..=n).filter(|x| !missing.contains(x)).collect();
        let (u, v) = find_missing_2(&input, &total);
        let mut result = [u, v];
        result.sort();
        assert_eq!(result, missing);
    }

    proptest! {
        #[test]
        fn prop_test_find_missing_2(n in 5..500i32, a in 1..500i32, b in 1..500i32) {
            // make sure a and b are distinct and within [1..=n]
            let (a, b) = if a == b || a > n || b > n {
                (1, 2) // fallback to valid missing pair
            } else {
                (a, b)
            };

            let missing = if a < b { [a, b] } else { [b, a] };

            // construct input missing a and b
            let input: Vec<i32> = (1..=n).filter(|&x| x != missing[0] && x != missing[1]).collect();
            let total: Vec<i32> = (1..=n).collect();

            let (u, v) = find_missing_2(&input, &total);
            let mut result = [u, v];
            result.sort();

            prop_assert_eq!(result, missing);
        }
    }
}

mod find_missing_k_tests {
    use xortrick::{find_missing_k, find_missing_k_rec};

    #[test]
    fn test_find_missing_k_one() {
        let total: Vec<i32> = (1..=5).collect();
        let input = vec![1, 2, 3, 5]; // missing = {4}
        let missing = find_missing_k(&input, &total, 1);
        assert_eq!(missing, vec![4]);
        let missing = find_missing_k_rec(&input, &total, 1);
        assert_eq!(missing, vec![4]);
    }

    #[test]
    fn test_find_missing_k_two() {
        let total: Vec<i32> = (1..=6).collect();
        let input = vec![2, 3, 5, 6]; // missing = {1,4}
        let mut missing = find_missing_k(&input, &total, 2);
        missing.sort();
        assert_eq!(missing, vec![1, 4]);

        let mut missing = find_missing_k_rec(&input, &total, 2);
        missing.sort();
        assert_eq!(missing, vec![1, 4]);
    }

    #[test]
    fn test_find_missing_k_three() {
        let total: Vec<i32> = (1..=7).collect();
        let input = vec![1, 3, 4, 7]; // missing = {2,5,6}
        let mut missing = find_missing_k(&input, &total, 3);
        missing.sort();
        assert_eq!(missing, vec![2, 5, 6]);
        let mut missing = find_missing_k_rec(&input, &total, 3);
        missing.sort();
        assert_eq!(missing, vec![2, 5, 6]);
    }

    #[test]
    fn test_find_missing_k_large_case() {
        let n = 1000;
        let total: Vec<i32> = (1..=n).collect();
        let missing = [123, 456, 789];
        let input: Vec<i32> = total
            .iter()
            .copied()
            .filter(|x| !missing.contains(x))
            .collect();
        let mut result = find_missing_k(&input, &total, 3);
        result.sort();
        assert_eq!(result, missing);
        let mut result = find_missing_k_rec(&input, &total, 3);
        result.sort();
        assert_eq!(result, missing);
    }
}
