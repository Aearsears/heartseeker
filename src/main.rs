mod proxy;

mod threadpool;

fn main() {
    proxy::start_proxy(String::from("127.0.0.1:4000"));
}
