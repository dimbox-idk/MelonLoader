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
3. Create a folder named `MelonLoader` inside your app's `Android/data` folder.
4. Download the .NET runtime for Android [here](https://dotnetcli.azureedge.net/dotnet/Runtime/8.0.6/dotnet-runtime-8.0.6-linux-bionic-arm64.tar.gz) and extract it's contents into `assets/dotnet` inside the decompiled APK.
5. Compile the MelonLoader solution and copy the resulting output into the `MelonLoader` folder.
6. Copy `Il2CppInterop.Common.dll`, `Il2CppInterop.Generator.dll`, and `Il2CppInterop.Runtime.dll` from `MelonLoader/net8` into `MelonLoader/Dependencies/Il2CppAssemblyGenerator`.
