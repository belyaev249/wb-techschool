fn as_chan(vs: &[i32]) -> std::sync::mpsc::Receiver<i32> {
    // Создается mpsc канал
    let (tx, rx) = std::sync::mpsc::channel();

    let handle = std::thread::spawn({
        let vs = vs.to_owned();
        move || {
            // Создается очередь,
            // внутри которой с периодичностью в 1 секунду отправляются элементы вектора vs через mpsc канал
            for v in vs {
                tx.send(v).unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            // Вызываем деструктор, закрываем канал
            // чтобы впоследствии receiver не ждал данные бесконечно
            drop(tx);
        }
    });
    
    // Ждем завершения работы созданной очереди
    handle.join().unwrap();

    // Возвращаем receiver
    rx
} 
    
fn merge(a: std::sync::mpsc::Receiver<i32>, b: std::sync::mpsc::Receiver<i32>) -> std::sync::mpsc::Receiver<i32> {
    // Создается mpsc канал
    let (tx, rx) = std::sync::mpsc::channel();
    
    // Заводим флаги, информирующие о закрытии каналов "a" и "b"
    let mut a_done = false;    
    let mut b_done = false;
    
    // В бесконечном цикле, на каждой итерации пытаемся последовательно получать данные из ранее созданных receiver-ов
    // Отправлеям полученные данные в mpsc канал
    // Если оба receiver-а считали все данные - выходим из цикла 
    loop {
        match a.try_recv() {
            Ok(i) => {
                tx.send(i).unwrap();
            },
            Err(_) => {
                a_done = true;
            }
        }
    
        match b.try_recv() {
            Ok(i) => {
                tx.send(i).unwrap();
            },
            Err(_) => {
                b_done = true;
            }
        }

        if a_done && b_done {
            break;
        } 
    }

    // Возвращаем созданный receiver
    rx    
} 
    
fn main() {
    // "a" receiver
    let a = as_chan(&vec![1, 3, 5, 7]);

    // "b" receiver
    let b = as_chan(&vec![2, 4, 6, 8]);

    // "c" receiver, который будет последовательно (по очереди) получать данные из "a" и "b"
    // вне зависимости от времени работы каждого из них
    let c = merge(a, b);
    
    // Числа будут распечатаны все сразу, а не по мере их получения
    // так как loop в merge крутится на главном потоке и держит его, пока не считает все данные
    // Печатается 1 2 3 4 5 6 7 8
    for v in c.iter() {
        println!("{v:?}");
    }    
}