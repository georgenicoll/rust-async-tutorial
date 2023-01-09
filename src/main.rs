mod chapter1;
mod chapter2;
mod chapter4;
mod server;

// fn main() {
//     server::serve();
// }

#[async_std::main]
async fn main() {
    server::serve().await;
}

pub fn stop_dead_code_warnings() {
    chapter1::_1_3a();
    chapter1::_1_3b();
    chapter2::_2_3();
    chapter4::_4_1();
    chapter4::_4_2();
}
