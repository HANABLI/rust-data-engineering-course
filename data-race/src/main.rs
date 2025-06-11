use std::thread;
use std::sync::Mutex;


fn main() {
    let data = Mutex::new(vec![1, 2, 3]);

    let handles: Vec<_> = (0..3).map(|i| {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", data);
}
