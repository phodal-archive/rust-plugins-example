use plugin_interface::swagger::{SwaggerService, CocoSwagger};

#[no_mangle]
pub extern "Rust" fn swagger() -> Box<dyn SwaggerService> {
    Box::new(SwaggerPlugin::default())
}

pub struct SwaggerPlugin {

}

impl Default for SwaggerPlugin {
    fn default() -> Self {
        SwaggerPlugin {}
    }
}

impl SwaggerService for SwaggerPlugin {
    fn run(&self) -> CocoSwagger {
        CocoSwagger {

        }
    }
}
