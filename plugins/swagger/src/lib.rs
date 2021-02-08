use plugin_interface::swagger::{CocoSwagger, SwaggerService};

#[no_mangle]
pub fn swagger() -> Box<dyn SwaggerService> {
    Box::new(SwaggerPlugin::default())
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
