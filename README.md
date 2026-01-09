# Android-Imgui-Mod-Menu-rs

## About

Rust-based ImGui mod menu for Android games. Minimal, low-level overlay implementation inspired by existing templates. The codebase contains extensive **unsafe code** and interacts directly with the Android runtime and native graphics APIs.

## Quick notes

* Contains a lot of **unsafe code** — use at your own risk.
* Overlay is rendered by hooking graphics calls (commonly `eglSwapBuffers`). This is a known, older technique — if you prefer a newer approach, apply it yourself.
* Target audience: Rust and Android mod developers.

## Injection & loading

**Runtime injection (ptrace-based)**

* [AndKittyInjector](https://github.com/MJx0/AndKittyInjector)
* [Android-Ptrace-Injector](https://github.com/reveny/Android-Ptrace-Injector)

**APK / startup load**
Place the compiled `.so` inside the APK and add the following smali into the target `Activity.onCreate` to load the library at startup:

```smali
const-string v0, "menu"
invoke-static {v0}, Ljava/lang/System;->loadLibrary(Ljava/lang/String;)V
```

To locate the `Activity.onCreate` for a given package you can use the *Current Activity* helper (optional):
[Current Activity](https://apkcombo.com/current-activity/com.yumo.current.activity/)

## Build

Minimal example (ARM64):

```bash
cargo ndk build --release -t arm64-v8a
```

## Examples / Inspiration

* [https://github.com/reveny/Android-ImGui-Mod-Menu](https://github.com/reveny/Android-ImGui-Mod-Menu)
* [https://github.com/fedes1to/Zygisk-ImGui-Menu](https://github.com/fedes1to/Zygisk-ImGui-Menu)
* [https://github.com/FrenchModding/ImGui-Android-Mod-Menu](https://github.com/FrenchModding/ImGui-Android-Mod-Menu)

## License

This project is released under the **GNU GPL v3** (GPL-3.0).
