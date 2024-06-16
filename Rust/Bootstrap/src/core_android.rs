use crate::{log, melonenv::paths};
use jni::{
    sys::{jint, JNI_VERSION_1_6},
    JNIEnv, JavaVM,
};
use lazy_static::lazy_static;
use std::{os::raw::c_void, panic::catch_unwind, sync::Mutex};

const INVALID_JNI_VERSION: jint = 0;

lazy_static! {
    pub static ref JAVA_VM: Mutex<Option<JavaVM>> = Mutex::new(None);
}

#[no_mangle]
pub extern "system" fn JNI_OnLoad(vm: JavaVM, _: *mut c_void) -> jint {
    let mut vm_mutex = JAVA_VM.lock().unwrap();
    *vm_mutex = Some(vm);

    let mut env: JNIEnv = vm_mutex.as_ref().unwrap().get_env().expect("Cannot get reference to the JNIEnv");
    vm_mutex.as_ref().unwrap().attach_current_thread()
        .expect("Unable to attach current thread to the JVM");

    paths::cache_data_dir(&mut env);
    crate::logging::logger::init().expect("Failed to initialize logger!");

    #[cfg(debug_assertions)]
    std::thread::spawn(|| unsafe {
        crate::dotnet_trace::redirect_stderr();
    });
    std::thread::spawn(|| unsafe {
        crate::dotnet_trace::redirect_stdout();
    });

    // TODO: copy dotnet runtime into /data/data

    log!("JNI initialized!");

    catch_unwind(|| JNI_VERSION_1_6).unwrap_or(INVALID_JNI_VERSION)
}

pub unsafe fn get_raw_java_vm() -> *mut *const c_void {
    let mutex = JAVA_VM.lock().unwrap();
    let vm = mutex.as_ref().expect("JavaVM not initialized");
    vm.get_java_vm_pointer() as *mut *const c_void
}