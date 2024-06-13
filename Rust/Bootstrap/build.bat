@echo off
cargo ndk -t arm64-v8a -o ./jniLibs build --release
xcopy "%~dp0jniLibs\arm64-v8a\*.so" "C:/Users/trevo/Desktop/android_2d_bw/lib/arm64-v8a" /Y
IF "%1" NEQ "auto" (
    pause
)