#[derive(Debug)]
pub struct Context {
    pub url: String,
    pub method: String,
    pub content: String,
}

pub fn connect(context: &Context) {
    println!("network connect!!: {:?}", context);
}
