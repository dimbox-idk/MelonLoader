use jni::{
    objects::{JClass, JString},
    sys::{jboolean, JavaVM},
    JNIEnv,
};
use std::path::PathBuf;
use std::ffi::CStr;

use crate::libloading;
use crate::libloading::load_lib;

#[no_mangle]
fn load(env: JNIEnv, _: JClass, _: JString) -> jboolean {
    load_bootstrap(&env);
    load_lib_unity(&env);
    return 1;
}

#[no_mangle]
fn unload(_: JNIEnv, _: JClass) {
    info!("unload");
}

fn load_bootstrap(env: &JNIEnv) {
    let bootstrap_lib = load_lib(&PathBuf::from("libBootstrap.so"))
        .unwrap_or_else(|e| {
            error!("Failed to load libBootstrap.so: {}", e.to_string());

            let dl_error = unsafe { libc::dlerror() };
            let error_message = unsafe {
                CStr::from_ptr(dl_error)
            };
            let formatted_string = error_message.to_string_lossy();
            error!("dlerror: {}", formatted_string);
            panic!();
        });

    let on_load: libloading::NativeMethod<fn(*mut JavaVM, *mut libc::c_void)> = bootstrap_lib
        .sym("JNI_OnLoad")
        .unwrap_or_else(|e| {
            error!("Failed to find JNI_OnLoad: {}", e.to_string());
            panic!();
        });

    (on_load)(env.get_java_vm().expect("msg").get_java_vm_pointer(), std::ptr::null_mut());

    let initialize: libloading::NativeMethod<fn()> = bootstrap_lib
        .sym("startup")
        .unwrap_or_else(|e| {
            info!("Failed to find Initialize: {}", e.to_string());
            panic!();
        });
        
    (initialize)();
}

fn load_lib_unity(env: &JNIEnv) {
    let unity_lib = load_lib(&PathBuf::from("libunity.so"))
        .expect("Couldn't load libunity!");

    let on_load: libloading::NativeMethod<fn(*mut JavaVM, *mut libc::c_void)> = unity_lib
        .sym("JNI_OnLoad")
        .expect("Couldn't find JNI_OnLoad!");

    (on_load)(
        env.get_java_vm().expect("Failed to get JavaVM from JNIEnv").get_java_vm_pointer(),
        std::ptr::null_mut()
    );
}