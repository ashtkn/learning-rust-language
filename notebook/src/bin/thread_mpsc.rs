use std::sync::mpsc;
use std::thread;

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     let handle = thread::spawn(move || {
//         let data = rx.recv().unwrap();
//         println!("{}", data);
//     });
//     let _ = tx.send("Hello, world!");
//     let _ = handle.join();
// }

fn main() {    
    let mut handles = Vec::new();
    let mut main_to_sub_senders = Vec::new();
    let mut sub_to_main_receivers = Vec::new();

    for _ in 0..10 {
        // メインスレッドからサブスレッドへ送信するチャンネル
        let (main_to_sub_sender, main_to_sub_receiver) = mpsc::channel();
        // サブスレッドからメインスレッドへ送信するチャンネル
        let (sub_to_main_sender, sub_to_main_receiver) = mpsc::channel();

        main_to_sub_senders.push(main_to_sub_sender);
        sub_to_main_receivers.push(sub_to_main_receiver);

        handles.push(thread::spawn(move || {
            let mut data = main_to_sub_receiver.recv().unwrap();
            data += 1;
            let _ = sub_to_main_sender.send(data);
        }));
    }
    
    let mut data = vec![1; 10];
    
    for x in 0..10 {
        let _ = main_to_sub_senders[x].send(data[x]);
    }

    for x in 0..10 {
        data[x] = sub_to_main_receivers[x].recv().unwrap();
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}
