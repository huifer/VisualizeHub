use rabbit_admin_webapi::demo::Demo;

fn main() {
    let x = Demo { name: "alksf".to_string() };
    x.hello();
}