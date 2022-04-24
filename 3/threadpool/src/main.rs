use threadpool::threadpool::ThreadPool;
fn main() {
    let pool = ThreadPool::new(4);
    for i in 0..4 {
        pool.spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(250 * i)); //para ver el desorden
        });
    }
    std::thread::sleep(std::time::Duration::from_secs(2));
}
