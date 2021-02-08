use libloading::Library;
use plugin_interface::swagger::SwaggerService;

#[cfg(target_os = "linux")]
fn load_library(lib_name: &str) -> Library {
    let path = format!("target/debug/lib{}.so", lib_name);
    unsafe {
        return libloading::Library::new(path)
            .expect("load library");
    }
}

#[cfg(target_os = "macos")]
fn load_library(lib_name: &str) -> Library {
    let path = format!("target/debug/lib{}.dylib", lib_name);
    unsafe {
        return libloading::Library::new(path)
            .expect("load library");
    }
}

#[cfg(target_os = "windows")]
fn load_library(lib_name: &str) -> Library {
    let path = format!("target\\debug\\{}.dylib", lib_name);
    unsafe {
        return libloading::Library::new(path)
            .expect("load library");
    }
}

fn main() {
    let lib = load_library("coco_swagger");

    let new_swagger: libloading::Symbol<extern "Rust" fn() -> Box<dyn SwaggerService>> = unsafe { lib.get(b"swagger") }
        .expect("load symbol");

    let service = new_swagger();
    service.run();
}
