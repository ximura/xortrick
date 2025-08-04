use xortrick::{find_missing, find_missing_2, find_missing_k_rec};

fn main() {
    let total: Vec<i32> = (1..=8).collect();

    let input = vec![1, 2, 4, 5, 7, 8];
    let m = find_missing(&input, &total);
    println!("{input:?}: {m}");
    println!("-----------------------------");

    let input2 = vec![1, 2, 4, 6, 7, 8];
    let (u, v) = find_missing_2(&input2, &total);
    println!("{input2:?}: {u}, {v}");
    println!("-----------------------------");

    let input3 = vec![1, 4, 6, 7, 8];
    let r: Vec<i32> = find_missing_k_rec(&input3, &total, 3);
    println!("missing 2 3 5");
    println!("{input3:?}: {r:?}");
}
