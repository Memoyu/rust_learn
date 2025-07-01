use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};

fn main() {
    // thread spawn创建一个新线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 使用JoinHandle join实现等待线程执行完成
    handle.join().unwrap();

    // 新线程使用move闭包
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector {:?}", v);
    });
    // 此处，v的是有权已经转移到了spawn中，则此处会报错
    // drop(v);
    handle.join().unwrap();

    // 使用消息传递，解决共享内存问题
    // 创建通道
    let (tx, rx) = mpsc::channel();
    // 使用新线程，在线程中发送消息
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // 此处val所有权已经被转移到通道中，所以此处会报所有权错误
        // println!("val is {}", val);
    });

    // 接收消息
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // mpsc “multiple producer, single consumer”（多个生产者，单个消费者）的缩写
    // 多生产者，单消费者
    let (tx, rx) = mpsc::channel();
    // 克隆生产者
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // 使用克隆的生产者发送消息
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            // 使用生产者发送消息
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recv in rx {
        println!("Got: {}", recv);
    }

    // 使用Mutex，互斥锁
    // 使用Rc构建多引用的Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // 克隆counter，传入线程
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
