// xor(1..n) ^ xor(input) = m
pub fn find_missing(input: &[i32], total: &[i32]) -> i32 {
    let mut result = 0;
    for x in input {
        result ^= x
    }

    for x in total {
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
pub fn find_missing_2(input: &[i32], total: &[i32]) -> (i32, i32) {
    let uv = find_missing(input, total);
    let lsb = uv & !(uv - 1);
    let partion_of_input = input.iter().filter(|&x| x & lsb == 0);
    let partion_of_total = total.iter().filter(|&x| x & lsb == 0);
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

pub fn find_missing_k(input: &[i32], total: &[i32], k: usize) -> Vec<i32> {
    let mut results = Vec::with_capacity(k);

    // Work stack: (input_slice, total_slice, k_missing)
    let mut stack = vec![(input.to_vec(), total.to_vec(), k)];

    while let Some((input, total, k)) = stack.pop() {
        // Compute XOR of total and input
        let xor_total = total.iter().fold(0, |acc, &x| acc ^ x);
        let xor_input = input.iter().fold(0, |acc, &x| acc ^ x);
        let xor_all = xor_total ^ xor_input;

        if k == 1 {
            results.push(xor_all);
            continue;
        }

        // Lowest set bit
        let lsb = xor_all & -xor_all;

        // Partition totals
        let mut total0 = Vec::new();
        let mut total1 = Vec::new();
        for &x in &total {
            if x & lsb == 0 {
                total0.push(x);
            } else {
                total1.push(x);
            }
        }

        // Partition inputs
        let mut input0 = Vec::new();
        let mut input1 = Vec::new();
        for &x in &input {
            if x & lsb == 0 {
                input0.push(x);
            } else {
                input1.push(x);
            }
        }

        // Number of missing in partition 0
        let k0 = total0.len() - input0.len();
        let k1 = k - k0;

        if k0 > 0 {
            stack.push((input0, total0, k0));
        }
        if k1 > 0 {
            stack.push((input1, total1, k1));
        }
    }

    return results;
}

pub fn find_missing_k_rec(input: &[i32], total: &[i32], k: usize) -> Vec<i32> {
    let xor_all = find_missing(input, total);

    if k == 1 {
        return vec![xor_all];
    }

    let lsb = xor_all & !(xor_all - 1);

    let total0: Vec<i32> = total.iter().copied().filter(|x| x & lsb == 0).collect();
    let total1: Vec<i32> = total.iter().copied().filter(|x| x & lsb != 0).collect();

    let input0: Vec<i32> = input.iter().copied().filter(|x| x & lsb == 0).collect();
    let input1: Vec<i32> = input.iter().copied().filter(|x| x & lsb != 0).collect();

    let k0 = total0.len() - input0.len();
    let k1 = k - k0;
    let mut result = Vec::new();
    if k0 > 0 {
        result.extend(find_missing_k_rec(&input0, &total0, k0));
    }
    if k1 > 0 {
        result.extend(find_missing_k_rec(&input1, &total1, k1));
    }

    return result;
}
