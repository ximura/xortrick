// xor(1..n) ^ xor(input) = m
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

pub fn find_missing_opt(input: &[i32], n: i32) -> i32 {
    let mut result = 0;
    // iterate up to n-1 because input has length n-1
    for i in 0..n - 1 {
        result ^= i + 1; // XOR from sequence 1..=n
        result ^= input[i as usize]; // XOR from input
    }

    // one number from the sequence remains: n
    result ^= n;

    return result;
}

pub fn find_missing_iter(input: &[i32], n: i32) -> i32 {
    use std::iter;

    let mut result = 0;

    // input iter, pad with None after exhaustion
    let input_iter = input.iter().map(|&x| Some(x)).chain(iter::repeat(None));

    // zip numbers 1..=n with input elements (or None)
    for (num, opt_x) in (1..=n).zip(input_iter) {
        result ^= num;
        if let Some(x) = opt_x {
            result ^= x;
        }
    }

    result
}

// xor(1..n) ^ xor(input) = u ^ v
// Partition 0:
// * The set of all values from 1 to n where i-th bit is 0
// * The set of input where i-th bit is 0
// Partition 1:
// * The set of all values from 1 to n where i-th bit is 1
// * The set of input where i-th bit is 1
pub fn find_missing_2(input: &[i32], n: i32) -> (i32, i32) {
    let uv = find_missing(input, n);
    let lsb = uv & !(uv - 1);
    let partion_of_input = input.iter().filter(|&x| x & lsb == 0);
    let partion_of_total = (1..n + 1).filter(|&x| x & lsb == 0);
    let mut u = 0;
    for num in partion_of_input {
        u ^= num
    }
    for num in partion_of_total {
        u ^= num
    }
    let v = uv ^ u;

    return (u, v);
}
