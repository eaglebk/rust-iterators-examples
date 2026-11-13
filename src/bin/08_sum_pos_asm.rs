// Проверить ассемблер на Godbolt: https://godbolt.org/z/Kjh19eMv8
#[no_mangle]
pub fn sum_pos(s: &[i32]) -> i32 {
    s.iter()
        .copied()
        .filter(|&x| x > 0)
        .map(|x| x * 2)
        .sum()
}

fn main() {
    let data = [-3, 1, 4, -2, 5, 0, 7, -1, 2, 8];
    let res = sum_pos(&data);
    println!("sum_pos(&data) = {res}");
}
