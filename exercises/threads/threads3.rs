// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


// 引入库，用于多线程同步和通信
use std::sync::mpsc; // 引入多生产者单消费者通道（multiple producer, single consumer）
use std::sync::Arc; // 引入原子引用计数智能指针（Atomic Reference Counting），提供线程安全的数据共享
use std::thread; // 引入线程处理模块
use std::time::Duration; // 引入时间间隔类型，用于在代码中休眠指定的时间

// 定义一个结构体Queue，它包含长度以及两个分别存储一半元素的向量
struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

// 实现Queue的构造函数new()，初始化Queue实例并填充数据
impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

// 定义一个名为send_tx的函数，它接受一个Queue实例和一个mpsc通道发送端，并在两个新线程中分别发送队列的两半数据
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    // 使用Arc将Queue实例包装起来，以便在多个线程间安全地共享所有权
    let qc = Arc::new(q);
    // 创建Arc的两个克隆副本qc1和qc2，它们共享同一份数据
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    // 同样对通道发送端进行克隆，创建tx1和tx2，这样每个线程都有自己的独立发送端
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    // 创建并启动第一个线程，发送Queue.first_half中的元素
    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap(); // 发送数据到通道
            thread::sleep(Duration::from_secs(1)); // 每发送一个元素后休眠1秒
        }
    });

    // 创建并启动第二个线程，发送Queue.second_half中的元素
    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap(); // 发送数据到通道
            thread::sleep(Duration::from_secs(1)); // 每发送一个元素后休眠1秒
        }
    });
}

// 主函数main()
fn main() {
    // 创建一个多生产者单消费者通道
    let (tx, rx) = mpsc::channel();

    // 创建一个Queue实例，并获取其长度
    let queue = Queue::new();
    let queue_length = queue.length;

    // 调用send_tx函数，在新的线程中开始发送队列中的数值
    send_tx(queue, tx);

    // 在主线程中循环接收来自通道rx的数据，并累加接收到的数值
    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    // 输出接收到的总数量，并与原始队列长度进行比较以确保所有数据都被正确接收
    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}
