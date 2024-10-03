use std::thread;

fn main() {
    /*
    多线程
     */

    let thread1 = thread::spawn(|| {
        for i in 1..=10 {
            println!("current thread i value is {}", i);
        }
    });

    let thread2 = thread::spawn(|| {
        for j in 20..=25 {
            println!("current thread j value is {}", j);
        }
    });

    thread2.join().unwrap();
    thread1.join().unwrap();

    println!("yes! all child thread is done!")
}
