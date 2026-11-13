fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Никаких вычислений! В памяти на стеке лежит структура Map<Iter<'_, i32>, ...>
    let doubled = numbers.iter().map(|x| {
        println!("Обрабатываю {x}");
        x * 2
    });

    println!(
        "Размер обёртки на стеке: {} байт",
        std::mem::size_of_val(&doubled)
    );
    println!("Реальный тип: {}", std::any::type_name_of_val(&doubled));
}
