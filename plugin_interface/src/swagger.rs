pub struct CocoSwagger {}

pub trait SwaggerService {
    fn run(&self) -> CocoSwagger;
}
