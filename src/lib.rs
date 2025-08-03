pub fn find_missing(input: &[i32], n: i32) -> i32 {
    let mut result = 0;
    for x in input {
        result ^= x
    }

    for x in 1..=n {
        result ^= x
    }

    return result;
}
