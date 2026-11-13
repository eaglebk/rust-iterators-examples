use std::collections::VecDeque;
use std::iter::FusedIterator;

// Имитация удалённого API: отдаёт по 2 элемента на страницу
fn fetch_page(page: usize) -> Vec<String> {
    match page {
        p if p == 0 => vec!["user_1".into(), "user_2".into()],
        p if p == 1 => vec!["user_3".into(), "user_4".into()],
        p if p == 2 => vec!["user_5".into()],
        _ => vec![],
    }
}

struct PageIter {
    next_page: usize,
    buffer: VecDeque<String>,
    done: bool,
}

impl PageIter {
    fn new() -> Self {
        Self {
            next_page: 0,
            buffer: VecDeque::new(),
            done: false,
        }
    }
}

impl Iterator for PageIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while self.buffer.is_empty() && !self.done {
            println!("-> Запрашиваю страницу {}", self.next_page);
            let items = fetch_page(self.next_page);
            if items.is_empty() {
                self.done = true;
            } else {
                self.next_page += 1;
                self.buffer.extend(items);
            }
        }
        self.buffer.pop_front()
    }
}

impl FusedIterator for PageIter {}

fn main() {
    // Загружаем только первые 3 записи: страница 2 даже не будет запрошена!
    let first_three: Vec<String> = PageIter::new().take(3).collect();
    println!("Получено записей: {:?}", first_three);
}
