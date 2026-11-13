fn main() {
    let mut numbers = vec![10, 20, 30];

    // 1. Заимствование по &T (эквивалентно for num in &numbers)
    for num in numbers.iter() {
        println!("По ссылке (&T): {num}");
    }

    // 2. Изменение на месте по &mut T (эквивалентно for num in &mut numbers)
    for num in numbers.iter_mut() {
        *num += 5;
    }
    println!("После iter_mut: {:?}", numbers);

    // 3. Поглощение коллекции: for num in numbers делает то же самое
    for num in numbers.into_iter() {
        println!("Во владение (T): {num}");
    }
    // Здесь numbers уже перемещён, обратиться к нему нельзя
}
