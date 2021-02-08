extern crate dlopen;
#[macro_use]
extern crate dlopen_derive;

use dlopen::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
struct SwaggerApi {
    demo: fn() -> i32,
}

fn main() {
    let cont: Container<SwaggerApi> = unsafe {
        Container::load("target/debug/libcoco_swagger.dylib")
    }.expect("Could not open library or load symbols");

    let plugin = cont.demo();
    println!("{:?}", plugin);
}
