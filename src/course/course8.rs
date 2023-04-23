/// 并发
#[cfg(test)]
mod test_concurrent {
    use std::{thread, time::Duration, sync::mpsc};

    #[test]
    fn test_simple_thread() {
        // 使用 move 强制所有权迁移
        let s = "hello";
        let handle = thread::spawn(move || {
            for i in 0..5 {
                println!("spawned thread print {}:{}", s, i);
                thread::sleep(Duration::from_millis(100));
            }
        });

        for i in 0..3 {
            println!("main thread print {}", i);
            thread::sleep(Duration::from_millis(100));
        }

        handle.join().unwrap();
    }

    #[test]
    fn test_channel() {
        // Multiple Producer Single Consumer
        // 多生产者（可以克隆给多个线程使用） 单消费者
        // 相对的 golang 则是 mpmc
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            println!("start to send");
            sender.send(val).unwrap();
        });

        println!("wait for receive");
        let received = receiver.recv().unwrap();
        println!("Got: {}", received);
    }

    // todo async and tokio
}
