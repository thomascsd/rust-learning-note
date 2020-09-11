extern crate communicator;

fn main() {
    let context = network::Context {
        url: "https://www.sample.com",
        method: "post",
    };
    network::connect(&context);
}
