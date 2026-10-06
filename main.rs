#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{AboutMetadata, Menu, MenuItem, PredefinedMenuItem, Submenu},
    webview::NewWindowResponse,
    Manager, Url, WebviewUrl, WebviewWindowBuilder,
};

const NETFLIX: &str = "https://www.netflix.com/";
const TITLE: &str = "Lume Netflix — experimental";

// Only top-level navigation is restricted. Netflix's CDN requests are untouched.
fn is_netflix_navigation(url: &Url) -> bool {
    url.scheme() == "https"
        && url.username().is_empty()
        && url.password().is_none()
        && url.port_or_known_default() == Some(443)
        && url
            .host_str()
            .is_some_and(|host| host == "netflix.com" || host.ends_with(".netflix.com"))
}

fn run() -> tauri::Result<()> {
    tauri::Builder::default()
        // Register first: repeated launches focus the existing window.
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .setup(|app| {
            let profile = app.path().app_local_data_dir()?.join("WebView2");
            std::fs::create_dir_all(&profile)?;

            let home = MenuItem::with_id(app, "home", "Início", true, Some("Ctrl+Shift+H"))?;
            let back = MenuItem::with_id(app, "back", "Voltar", true, Some("Alt+ArrowLeft"))?;
            let reload = MenuItem::with_id(app, "reload", "Recarregar", true, Some("F5"))?;
            let fullscreen = MenuItem::with_id(app, "fullscreen", "Tela cheia", true, Some("F11"))?;
            let quit = MenuItem::with_id(app, "quit", "Sair", true, Some("Alt+F4"))?;
            let separator = PredefinedMenuItem::separator(app)?;
            let about = PredefinedMenuItem::about(
                app,
                Some("Sobre / limitação de DRM"),
                Some(AboutMetadata {
                    name: Some("Lume Netflix".into()),
                    version: Some(env!("CARGO_PKG_VERSION").into()),
                    comments: Some(
                        "Cliente experimental e não oficial. Usa WebView2 Runtime, separado do navegador Edge. DRM, reprodução, 1080p, 4K e HDR não são garantidos. Netflix é marca de seus respectivos titulares."
                            .into(),
                    ),
                    ..Default::default()
                }),
            )?;
            let submenu = Submenu::with_items(
                app,
                "Lume",
                true,
                &[&home, &back, &reload, &fullscreen, &separator, &about, &quit],
            )?;
            let menu = Menu::with_items(app, &[&submenu])?;

            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(NETFLIX.parse()?))
                .title(TITLE)
                .inner_size(1100.0, 720.0)
                .min_inner_size(640.0, 420.0)
                .prevent_overflow()
                .center()
                .resizable(true)
                .decorations(true)
                .skip_taskbar(false)
                .data_directory(profile)
                .incognito(false)
                .devtools(false)
                // Leave WebView2's GPU selection and sandbox defaults intact.
                // No force-GPU, single-process, DRM, or user-agent override flags.
                .on_navigation(is_netflix_navigation)
                // Extra webviews would add memory and processes. Popups are blocked.
                .on_new_window(|_, _| NewWindowResponse::Deny)
                .on_download(|_, _| false)
                .menu(menu)
                .build()?;
            Ok(())
        })
        .on_menu_event(|app, event| {
            let Some(window) = app.get_webview_window("main") else {
                return;
            };
            // Fixed expressions only; no credentials or page content are read.
            match event.id().as_ref() {
                "home" => {
                    if let Ok(url) = NETFLIX.parse() {
                        let _ = window.navigate(url);
                    }
                }
                "back" => {
                    let _ = window.eval("window.history.back()");
                }
                "reload" => {
                    let _ = window.eval("window.location.reload()");
                }
                "fullscreen" => {
                    if let Ok(active) = window.is_fullscreen() {
                        let _ = window.set_fullscreen(!active);
                    }
                }
                "quit" => {
                    let _ = window.close();
                }
                _ => {}
            }
        })
        // The last window's close uses Tauri's normal exit behavior.
        .run(tauri::generate_context!())
}

#[cfg(target_os = "windows")]
fn show_startup_error(error: &str) {
    // A release GUI executable has no console. Show initialization errors natively.
    #[link(name = "user32")]
    extern "system" {
        fn MessageBoxW(hwnd: *mut std::ffi::c_void, text: *const u16, title: *const u16, flags: u32) -> i32;
    }
    let message: Vec<u16> = format!(
        "Não foi possível abrir o Lume Netflix.\n\nVerifique o Microsoft Edge WebView2 Runtime (Evergreen). O navegador Edge não é necessário.\n\nDetalhes: {error}"
    )
    .encode_utf16()
    .chain(std::iter::once(0))
    .collect();
    let title: Vec<u16> = "Lume Netflix\0".encode_utf16().collect();
    // Both UTF-16 buffers remain alive throughout the synchronous Windows call.
    unsafe {
        MessageBoxW(std::ptr::null_mut(), message.as_ptr(), title.as_ptr(), 0x10);
    }
}

#[cfg(not(target_os = "windows"))]
fn show_startup_error(error: &str) {
    eprintln!("Lume Netflix: {error}");
}

fn main() {
    if let Err(error) = run() {
        show_startup_error(&error.to_string());
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_accepts_only_https_netflix_origins() {
        for url in [NETFLIX, "https://netflix.com/login", "https://help.netflix.com/pt/node/23931"] {
            assert!(is_netflix_navigation(&Url::parse(url).unwrap()), "{url}");
        }
        for url in [
            "http://www.netflix.com/", "https://netflix.com.evil.example/",
            "https://evilnetflix.com/", "https://netflix.com@evil.example/",
            "https://evil.example@netflix.com/", "https://netflix.com:8443/",
            "file:///C:/Windows/", "javascript:alert(1)",
        ] {
            assert!(!is_netflix_navigation(&Url::parse(url).unwrap()), "{url}");
        }
    }
}

