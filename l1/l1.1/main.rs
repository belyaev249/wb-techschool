use std::ops::Fn;

// Person
struct Person {
    name: &'static str
}

// Default impl
trait Action {
    fn say(&self);
}

impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", &self.name);
    }
}

// Lazy impl
// По предположению должен быть быстрее. Собираем строку "Hello, NAME" один раз - вместо каждого вызова
trait LazyAction {
    fn say_lazy(&self) -> impl Fn() -> ();
}

impl LazyAction for Person {
    fn say_lazy(&self) -> impl Fn() -> () {
        let p: String = format!("Hello, {}", &self.name);
        {
             move || {
                println!("{p}");
            }
        }
    }
}

fn main() {
    let john = Person { name: "John" };
    john.say();

    let say_lazy = john.say_lazy();
    say_lazy();
}

// Из интереса пробовал пооптимизировать
// По времени, writeln без lock() на каждом вызове, работает немного быстрее
#[cfg(test)]
mod tests {
    use std::{time::Instant, io::Write};
    use crate::{Action, LazyAction, Person};

    const LONG_TEXT: &'static str = "Lorem ipsum odor amet, consectetuer adipiscing elit. Lorem taciti semper sem etiam nunc parturient. Vitae cras mus libero feugiat sapien ex etiam dictum amet. Taciti torquent tellus mus mi condimentum dui donec metus quisque. Primis rutrum tempus libero proin auctor parturient sollicitudin ultricies hendrerit. Dignissim curae fusce finib";

    impl Person {
        fn with_long_name() -> Person {
            Person { name: LONG_TEXT }
        }
    }

    #[test]
    #[inline(never)]
    fn default_action_bench() {
        let john = Person::with_long_name();
        let now = Instant::now();
        for _ in 1..=10000 {
            john.say();
        }
        let t = now.elapsed();
        println!("Default Action | 10_000 calls | {:?}", t);
    }

    #[test]
    #[inline(never)]
    fn lazy_action_bench() {
        let john = Person::with_long_name();
        let say_lazy = john.say_lazy();
        let now = Instant::now();
        for _ in 1..=10000 {
            say_lazy();
        }
        let t = now.elapsed();
        println!("Lazy Action | 10_000 calls | {:?}", t);
    }

    #[test]
    #[inline(never)]
    fn println_without_lock_bench() {
        let mut stdout = std::io::stdout().lock();
        let john = Person::with_long_name();
        let text = format!("Hello, {}", &john.name);
        let now = Instant::now();
        for _ in 1..=10000 {
            writeln!(stdout, "{text}").unwrap();
        }
        let t = now.elapsed();
        println!("Println without lock | 10_000 calls | {:?}", t);
    }
}