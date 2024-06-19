use jni::{
    objects::{JObject, JObjectArray, JString, JValueGen},
    JNIEnv,
};
use ndk_sys::AAssetManager;
use std::{ffi::CString, fs, io::Write};

use crate::{errors::DynErr, melonenv::paths};

pub fn copy_melonloader_data(env: &mut JNIEnv) -> Result<bool, DynErr> {
    let base = format!("{}/", paths::BASE_DIR.clone().display());

    unsafe {
        copy_asset_to_path("MelonLoader", &base, env);
        copy_asset_to_path("dotnet", paths::get_internal_data_path()?.to_str().unwrap(), env);
    }
    Ok(true)
}

unsafe fn copy_asset_to_path(path: &str, destination: &str, env: &mut JNIEnv) {
    let unity_class_name = "com/unity3d/player/UnityPlayer";
    let unity_class = &env
        .find_class(unity_class_name)
        .expect("Failed to find class com/unity3d/player/UnityPlayer");

    let current_activity_obj: JObject = env
        .get_static_field(unity_class, "currentActivity", "Landroid/app/Activity;")
        .expect("Failed to get static field currentActivity")
        .l()
        .unwrap();

    let asset_manager_obj = env
        .call_method(
            current_activity_obj,
            "getAssets",
            "()Landroid/content/res/AssetManager;",
            &[],
        )
        .unwrap()
        .l()
        .unwrap();

    let path_string = env.new_string(path).unwrap();
    let base_string = env.new_string(destination).unwrap();

    let assets_array: JObjectArray = env
        .call_method(
            &asset_manager_obj,
            "list",
            "(Ljava/lang/String;)[Ljava/lang/String;",
            &[JValueGen::from(&path_string)],
        )
        .unwrap()
        .l()
        .unwrap()
        .into();

    let assets_length = env.get_array_length(&assets_array).unwrap();
    let assets_size = assets_length as usize;

    if assets_size == 0 {
        copy_file(path, destination, env);
    } else {
        let full_path = format!("{}/{}", destination, path);
        create_directory(&full_path);

        for i in 0..assets_size {
            let asset = env
                .get_object_array_element(&assets_array, i as i32)
                .unwrap();
            let jstr: JString = asset.into();
            let asset_str: String = env.get_string(&jstr).unwrap().into();
            let asset_path = format!("{}/{}", path, asset_str);
            env.delete_local_ref(jstr).unwrap();

            copy_asset_to_path(&asset_path, destination, env);
        }
    }

    env.delete_local_ref(path_string).unwrap();
    env.delete_local_ref(base_string).unwrap();
}

unsafe fn copy_file(filename: &str, base: &str, env: &mut JNIEnv) {
    let asset_manager = get_asset_manager(env);

    let filename_cstr = CString::new(filename).unwrap();
    let asset = ndk_sys::AAssetManager_open(
        asset_manager,
        filename_cstr.as_ptr(),
        ndk_sys::AASSET_MODE_UNKNOWN as i32,
    );

    let full_path = format!("{}/{}", base, filename);
    if std::path::Path::new(&full_path).exists() {
        fs::remove_file(&full_path).unwrap();
    }
    let mut output_stream = fs::File::create(full_path).unwrap();

    const BUFFER_SIZE: usize = 1024;
    let mut buffer = [0u8; BUFFER_SIZE];

    loop {
        let bytes_read = ndk_sys::AAsset_read(
            asset,
            buffer.as_mut_ptr() as *mut std::ffi::c_void,
            BUFFER_SIZE as usize,
        );
        if bytes_read <= 0 {
            break;
        }

        output_stream
            .write_all(&buffer[0..bytes_read as usize])
            .unwrap();
    }

    ndk_sys::AAsset_close(asset);

    output_stream.flush().unwrap();
}

fn create_directory(path: &str) -> bool {
    if !fs::metadata(&path).is_ok() {
        if fs::create_dir(&path).is_err() {
            return false;
        }
    }

    true
}

fn get_asset_manager(env: &mut JNIEnv) -> *mut AAssetManager {
    let unity_class_name = "com/unity3d/player/UnityPlayer";
    let unity_class = &env
        .find_class(unity_class_name)
        .expect("Failed to find class com/unity3d/player/UnityPlayer");

    let current_activity_obj: JObject = env
        .get_static_field(unity_class, "currentActivity", "Landroid/app/Activity;")
        .expect("Failed to get static field currentActivity")
        .l()
        .unwrap();

    let asset_manager = env.call_method(
        current_activity_obj,
        "getAssets",
        "()Landroid/content/res/AssetManager;",
        &[],
    );
    unsafe {
        return ndk_sys::AAssetManager_fromJava(
            env.get_native_interface(),
            asset_manager.unwrap().l().unwrap().as_raw(),
        );
    }
}
