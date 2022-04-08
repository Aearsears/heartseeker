mod proxy;
mod threadpool;
mod webserver;

fn main() {
    let pool = threadpool::ThreadPool::new(2);
    println!("Backend started...");
    pool.execute(|| {
        proxy::start_proxy(String::from("127.0.0.1:4000"));
    });
    pool.execute(|| webserver::start_admin_page(String::from("127.0.0.1:4001")));
}
