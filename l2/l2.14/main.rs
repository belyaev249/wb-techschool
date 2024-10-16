fn main() {
    // Создается multi-producer, single-consumer канал
    let (tx, rv) = std::sync::mpsc::channel::<i32>();
    
    // Создается очередь и возвращается handle на нее
    // Замыкание забирает tx (sender) во владение, поэтому указывается аттрибут move
    let handle = std::thread::spawn(move || {
        // Итерируемся по диапазону от 0 до 9
        // и отправлем очередное число по mpsc каналу, последовательно
        for i in 0..10 {
            tx.send(i).unwrap();        
        }
    });

    // Главный поток ждет завершения вызванной очереди
    handle.join().unwrap();
    
    // Итерируемся по всем числам, которые получил rv (receiver)
    // Выводим в stdout в том же порядке как и были получены
    // Печатается 0 1 2 3 4 5 6 7 8 9
    for i in rv.iter() {
        println!("{i:?}");
    }    
}