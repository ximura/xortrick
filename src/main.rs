use xortrick::find_missing;

fn main() {
    let input = vec![1, 2, 3, 4, 6, 7, 8];
    let m = find_missing(&input, 8);
    println!("{input:?}: {m}");
}
