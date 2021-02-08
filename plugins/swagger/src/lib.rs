use plugin_interface::swagger::{CocoSwagger, SwaggerService};

#[no_mangle]
pub fn swagger() -> Box<dyn SwaggerService> {
    Box::new(SwaggerPlugin::default())
}

#[no_mangle]
pub fn demo() -> i32 {
    return 42
}

pub struct SwaggerPlugin {}

impl Default for SwaggerPlugin {
    fn default() -> Self {
        SwaggerPlugin {}
    }
}

impl SwaggerService for SwaggerPlugin {
    fn run(&self) -> CocoSwagger {
        println!("start swagger services");
        CocoSwagger {}
    }
}
