fn main() {
    let raw_inputs = ["10", "20", "bad_input", "40"];

    // Цепочка ленива: пока нет терминального метода, замыкания не запускаются
    let lazy_pipeline = raw_inputs
        .iter()
        .inspect(|s| println!("Читаю: {s}"))
        .map(|s| s.parse::<i32>());

    println!("Пайплайн создан, но ещё ничего не вычислено!");

    // collect::<Result<Vec<_>, _>>() остановится на первой ошибке и не дойдёт до "40"
    let parsed: Result<Vec<i32>, _> = lazy_pipeline.collect();
    println!("Результат разбора: {:?}", parsed);

    // .copied() убирает лишний уровень ссылки (&i32 -> i32)
    let nums = [1, -2, 3, 4];
    let positive_sum: i32 = nums.iter().copied().filter(|&x| x > 0).sum();
    println!("Сумма положительных: {positive_sum}");
}
