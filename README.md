# Aura Browser — Android / Rust-first prototype

Aura is a Rust + Tao + Wry Android browser-shell prototype with a dark graphite/violet home screen. The actual page renderer is Android System WebView (Chromium-based on Xiaomi 14T), not a separately bundled Chromium build.

## Build an APK from your phone with GitHub Actions

This repository includes `.github/workflows/android-apk.yml`. GitHub-hosted Linux runners install Java, Android SDK/NDK and Rust, generate the Wry Android scaffold, then attempt an ARM64 release build and publish the APK as a workflow artifact.

1. Download and extract the ZIP on your phone.
2. Open https://github.com/ and sign in or create an account.
3. Create a **new public or private repository** named `Aura-Browser`. Do not add a README, license or `.gitignore` on GitHub because this archive already contains them.
4. On the repository page choose **Add file → Upload files**. Upload the *contents* of the extracted `aura-browser` folder, including the hidden `.github` folder. If the mobile web UI hides hidden folders, use GitHub's browser-based file creator to create `.github/workflows/android-apk.yml` and paste its contents from this archive.
5. Commit the files to the `main` branch.
6. Open the repository's **Actions** tab, choose **Build Aura Browser APK**, press **Run workflow**, then confirm.
7. When the run finishes with a green check, open it and download the `Aura-Browser-Android-APK` artifact. Extract the downloaded artifact ZIP to get the APK.
8. Install the APK on the Xiaomi 14T. Android may ask permission to install apps from the browser or file manager you used.

A push to `main` also starts a build automatically. GitHub Actions artifacts are temporary (14 days in this workflow).

## Important status

This is an early prototype, not a finished daily-use browser. The home/search screen is present, but full navigation, independent tabs, persistent history/bookmarks, downloads, private mode and content blocking are not implemented yet. The current home page uses an embedded frame for page previews; sites that prohibit iframe embedding will not load in that view.

The GitHub Actions workflow is supplied but has **not been run in this environment**. The first cloud build may expose compatibility issues between the current `cargo-mobile2`, Tao and Wry versions. If it fails, open the failed workflow and share the first red error section; do not repeatedly rerun it unchanged.

## Xiaomi 14T

- Target ABI: `arm64-v8a` (`aarch64-linux-android`).
- Uses the system WebView provider; keep Android System WebView / Chrome updated.
- HyperOS manages refresh rate dynamically. The interface avoids continuous animation to reduce unnecessary work.
- Web content memory is mainly managed by Chromium WebView. Proper tab lifecycle and memory policy are still future work.

## Architecture

- Rust: app entry point and browser-shell logic.
- Tao: window/event-loop integration.
- Wry: system WebView integration.
- HTML/CSS/JS: bundled UI.

Wry Android requires generated Kotlin/JNI glue and an Android Activity extending its WryActivity integration. The workflow generates this scaffold with `cargo-mobile2` on a supported Linux host; `cargo-mobile2` itself is not supported when run directly inside Termux on Android.
