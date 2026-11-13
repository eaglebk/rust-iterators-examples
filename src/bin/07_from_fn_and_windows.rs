// Возвращаем ленивый итератор без создания промежуточного Vec
fn non_empty_lines(text: &str) -> impl Iterator<Item = &str> + '_ {
    text.lines().map(str::trim).filter(|line| !line.is_empty())
}

fn main() {
    // 1. std::iter::from_fn - нарезка ID на батчи без отдельной структуры
    let mut cursor = Some(0u64);
    let batches = std::iter::from_fn(move || {
        let start = cursor?;
        if start >= 100 {
            cursor = None;
            return None;
        }
        let end = (start + 25).min(100);
        cursor = Some(end);
        Some(start..end)
    });
    println!(
        "Размер обёртки from_fn на стеке: {} байт",
        std::mem::size_of_val(&batches)
    );
    println!("Батчи ID: {:?}", batches.collect::<Vec<_>>());

    // 2. Ленивый парсинг строк без аллокаций
    let log = "\n  INFO start \n\n  WARN disk 85% \n";
    let lines: Vec<_> = non_empty_lines(log).collect();
    println!("Строки лога: {:?}", lines);

    // 3. Скользящее окно windows(2) вместо ручного slice[i + 1]
    let temps = [18, 21, 19, 24];
    let diffs: Vec<i32> = temps.windows(2).map(|w| w[1] - w[0]).collect();
    println!("Дельты температур: {:?}", diffs);
}
