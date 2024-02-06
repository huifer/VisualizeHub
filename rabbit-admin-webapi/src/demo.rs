pub struct Demo {
    pub name: String,

}

// 在 Demo 结构体上实现一个关联函数 hello
impl Demo {
    pub fn hello(&self) {
        println!("Hello from Demo! {}", self.name);
    }
}