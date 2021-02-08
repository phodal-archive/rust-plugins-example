use libloading::Library;
use plugin_interface::swagger::SwaggerService;

#[cfg(target_os = "linux")]
fn load_library() -> Library {
    unsafe {
        return libloading::Library::new("target/debug/libcoco_swagger.so")
            .expect("load library");
    }
}

#[cfg(target_os = "macos")]
fn load_library() -> Library {
    unsafe {
        return libloading::Library::new("target/debug/libcoco_swagger.dylib")
            .expect("load library");
    }
}

#[cfg(target_os = "windows")]
fn load_library() -> Library {
    unsafe {
        return libloading::Library::new("target\\debug\\libcoco_swagger.dll")
            .expect("load library");
    }
}

fn main() {
    let lib = load_library();

    let new_swagger: libloading::Symbol<extern "Rust" fn() -> Box<dyn SwaggerService>> = unsafe { lib.get(b"swagger") }
        .expect("load symbol");

    let service = new_swagger();
    service.run();
}
