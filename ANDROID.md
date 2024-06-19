Notes for running the WIP Android build of CoreCLR/new MelonLoader
Uses .NET 8.0.6 as it was the newest available at the time of writing.

Rough steps
1. Compile both `MelonProxy` and `Bootstrap` using the `cargo ndk` command. They should compile at the same time as they are part of the same workspace.
2. Copy the following files into the decompiled APK's arm64 library folder
   - `libmain.so` (compiled; replaces the original)
   - `libBootstrap.so` (compiled)
   - `libdobby.so` (available [here](https://github.com/RinLovesYou/dobby-sys/raw/master/dobby_libraries/android/arm64/libdobby.so))
   - `libhostfxr.so` (available inside the dotnet runtime [here](https://dotnetcli.azureedge.net/dotnet/Runtime/8.0.6/dotnet-runtime-8.0.6-linux-bionic-arm64.tar.gz))
   - `libssl.so` and `libcrypto.so` (both available inside this repo's `BaseLibs/openssl` folder; can also be compiled manually)
3. Download the .NET runtime for Android [here](https://dotnetcli.azureedge.net/dotnet/Runtime/8.0.6/dotnet-runtime-8.0.6-linux-bionic-arm64.tar.gz) and extract it's contents into `assets/dotnet` inside the decompiled APK.
4. Take the files from `BaseLibs/dotnet_fixed_gc` and replace the files in `dotnet/shared/Microsoft.NETCore.App/8.0.6` with them.
   - This fixes a bug where Mono and IL2CPP fight each other's garbage collector, causing freezes and/or crashes. If you want to compile this fix manually (available in LemonLoader/runtime), the Docker command I used is below.
5. Compile the MelonLoader solution and copy the resulting output into your APK's `assets` folder.
6. Add the following permissions to your APK's manifest.
   - `android.permission.ACCESS_NETWORK_STATE`
   - `android.permission.INTERNET`


.NET Runtime fork compilation command (ran on Ubuntu 18.04 x86_64 in a VM)
```
sudo docker run --rm \
  -v ~/runtime:/runtime \
  -v ~/android-ndk-r26d:/ndk \
  -w /runtime \
  -e ANDROID_NDK_ROOT=/ndk \
  -e ROOTFS_DIR=/ndk/toolchains/llvm/prebuilt/linux-x86_64/sysroot/ \
  mcr.microsoft.com/dotnet-buildtools/prereqs:cbl-mariner-2.0-cross-android-amd64 \
  /bin/bash -c \
  "./build.sh --subset Mono.Runtime --os 'linux-bionic' --cross --arch arm64"
```