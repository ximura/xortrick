use xortrick::{find_missing, find_missing_2};

fn main() {
    let input = vec![1, 2, 4, 5, 7, 8];
    let m = find_missing(&input, 8);
    println!("{input:?}: {m}");

    println!("-----------------------------");
    let input2 = vec![1, 2, 4, 6, 7, 8];
    let (u, v) = find_missing_2(&input2, 8);
    println!("{input2:?}: {u}, {v}");
}
