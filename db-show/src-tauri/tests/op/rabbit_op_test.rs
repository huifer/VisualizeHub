use rabbit_admin_webapi::demo::Demo;

#[tokio::test]
async fn main() {
    let demo = Demo { name: "omg".to_string() };
    demo.hello();
}