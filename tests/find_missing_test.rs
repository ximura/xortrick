use xortrick::find_missing;

#[test]
fn test_missing_middle() {
    let input = vec![1, 2, 3, 5];
    assert_eq!(find_missing(&input, 5), 4);
}

#[test]
fn test_missing_first() {
    let input = vec![2, 3, 4, 5];
    assert_eq!(find_missing(&input, 5), 1);
}

#[test]
fn test_missing_last() {
    let input = vec![1, 2, 3, 4];
    assert_eq!(find_missing(&input, 5), 5);
}

#[test]
fn test_large_case() {
    let input: Vec<i32> = (1..=1000).filter(|&x| x != 789).collect();
    assert_eq!(find_missing(&input, 1000), 789);
}
