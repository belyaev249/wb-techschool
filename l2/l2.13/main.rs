struct Example(i32);

impl Drop for Example {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

struct ExampleWrap(Example);

impl Drop for ExampleWrap {
    fn drop(&mut self) {
        let e = std::mem::replace(&mut self.0, Example(0)); println!("wrap {}", e.0);
    }
} 

// Размеры структур Example и ExampleWrap известны на компиляции, равны i32
// Память для них выделяется на стеке
// Реализация Drop будет вызываться по освобождению памяти, по выходу объекта из его области видимости
// В примере, максимальная область видимости - функция main
fn main() {

    // Example(1) создается и сразу высвобождается
    // Печатается 1
    Example(1);

    // Example(2) создается
    let _e2 = Example(2);

    // Example(3) создается
    let _e3 = Example(3);

    // Example(4) создается и сразу высвобождается как безымянная переменная
    // Печатается 4
    let _ = Example(4);

    // Отложенная инициализация
    let mut _e5;
    // Example(5) создается
    _e5 = Some(Example(5));
    // Example(5) высвобождается
    // Печатается 5
    _e5 = None;

    // Example(6) создается
    let e6 = Example(6);
    // Example(6) принудительно высвобождается
    // Печатается 6
    drop(e6);

    // Example(7) создается
    let e7 = Example(7);
    // std::mem::forget забирает Example(7) и никогда не вызывает деструктор
    std::mem::forget(e7);
    
    // Создается ExampleWrap(Example(8))
    // Example(8) попадает на стек
    // Внутри деструктора на верх стека попадает Example(0)
    // А затем std::mem::replace меняет их указаетли местами, теперь Example(8) наверху стека
    // Печатается wrap(8)

    // Стек освобождает память с верхнего объекта
    // поэтому сначала освобождается Example(8), затем Example(0)
    // Печатается 8
    // Печатается 0
    ExampleWrap(Example(8));

    // На стеке остались Example(2) и Example(3)
    // Так же освобождаются с верхнего объекта
    // Печатается 3
    // Печатается 2
}