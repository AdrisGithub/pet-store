use wbsl::ser_servlet::SerializeServlet;

use pet_store::Test;

fn main() {
    SerializeServlet::builder()
        .with_func(test)
        .bind("0.0.0.0:6969").unwrap().start();
}

pub fn test(req: Test) -> Test {
    println!("{:?}", req);
    req
}