pub fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    let mut cnt = 0;
    for i in 0..arr_a.len() {
        if arr_a[i] == arr_b[i] {
            cnt += 4;
        }
        if arr_a[i] != "" && arr_b[i] != "" && arr_a[i] != arr_b[i] {
            cnt -= 1;
        }
    }
   match cnt {
       n if n >= 0 => n,
       _ => 0,
    }
}

// Second Solution
#[allow(dead_code)]
fn check_exam2(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    arr_a.iter().zip(arr_b.iter()).fold(0, |pts, ans| {
        match ans {
            (a, b) if a == b => pts + 4, 
            (_, b) if b == &"" => pts, 
            _ => pts - 1
        }
    }).max(0)
}