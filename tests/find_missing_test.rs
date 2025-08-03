mod find_missing_tests {
    use xortrick::{find_missing, find_missing_iter, find_missing_opt};

    #[test]
    fn test_missing_middle() {
        let input = vec![1, 2, 3, 5];
        assert_eq!(find_missing(&input, 5), 4);
    }

    #[test]
    fn test_missing_first() {
        let input = vec![2, 3, 4, 5];
        assert_eq!(find_missing(&input, 5), 1);
        assert_eq!(find_missing_opt(&input, 5), 1);
        assert_eq!(find_missing_iter(&input, 5), 1);
    }

    #[test]
    fn test_missing_last() {
        let input = vec![1, 2, 3, 4];
        assert_eq!(find_missing(&input, 5), 5);
        assert_eq!(find_missing_opt(&input, 5), 5);
        assert_eq!(find_missing_iter(&input, 5), 5);
    }

    #[test]
    fn test_large_case() {
        let input: Vec<i32> = (1..=1000).filter(|&x| x != 789).collect();
        assert_eq!(find_missing(&input, 1000), 789);
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
        let input = vec![1, 2, 4];
        let (u, v) = find_missing_2(&input, 5);
        let mut missing = [u, v];
        missing.sort();
        assert_eq!(missing, [3, 5]);
    }

    #[test]
    fn test_missing_first_and_last() {
        // total = [1,2,3,4,5,6], input = [2,3,4,5]
        // missing = {1,6}
        let input = vec![2, 3, 4, 5];
        let (u, v) = find_missing_2(&input, 6);
        let mut missing = [u, v];
        missing.sort();
        assert_eq!(missing, [1, 6]);
    }

    #[test]
    fn test_missing_middle_numbers() {
        // total = [1..=8], input = [1,2,4,5,6,8]
        // missing = {3,7}
        let input = vec![1, 2, 4, 5, 6, 8];
        let (u, v) = find_missing_2(&input, 8);
        let mut missing = [u, v];
        missing.sort();
        assert_eq!(missing, [3, 7]);
    }

    #[test]
    fn test_large_case() {
        let n = 1000;
        let missing = [123, 987];
        let input: Vec<i32> = (1..=n).filter(|x| !missing.contains(x)).collect();
        let (u, v) = find_missing_2(&input, n);
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

            let (u, v) = find_missing_2(&input, n);
            let mut result = [u, v];
            result.sort();

            prop_assert_eq!(result, missing);
        }
    }
}
