pub fn balanced_num(n: u64) -> String {
    let len = n.to_string().len();
    let num_str = n.to_string();
    let (left, right);

    if len % 2 == 0 {
        left = &num_str[..len / 2 - 1];
        right = &num_str[len / 2 + 1..];
    } else {
        left = &num_str[..len / 2];
        right = &num_str[len / 2 + 1..];
    }

    let sum_left: u32 = left.chars().map(|d| d.to_digit(10).unwrap()).sum();
    let sum_right: u32 = right.chars().map(|d| d.to_digit(10).unwrap()).sum();

    if sum_left == sum_right {
        "Balanced".to_string()
    } else {
        "Not Balanced".to_string()
    }
}
