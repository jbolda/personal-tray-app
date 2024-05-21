// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::menu::MenuBuilder;
use tauri::tray::{ClickType, TrayIconBuilder};
use tauri::{Manager, WindowEvent};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            /* system tray setup */

            // these variables are an example of how to add MenuItems to the system tray
            //   which then need to be added to the MenuBuilder.items()
            //   as well as listening for events in the TrayIconBuilder.on_menu_event()
            // let quit = MenuItemBuilder::new("Quit").id("quit").build(app).unwrap();
            // let hide = MenuItemBuilder::new("Hide").id("hide").build(app).unwrap();
            // let show = MenuItemBuilder::new("Show").id("show").build(app).unwrap();
            //  it will be imported from use tauri::menu::MenuItemBuilder;
            let menu = MenuBuilder::new(app)
                // .items(&[&quit, &hide, &show])
                .build()
                .unwrap();

            let window = app.get_webview_window("main").unwrap();
            let window_hider = window.clone();
            // window.on_window_event(move |event| match event {
            //     WindowEvent::Focused(false) => {
            //         window_hider.hide().unwrap();
            //     }
            //     _ => {}
            // });

            let _ = TrayIconBuilder::new()
                .tooltip("personal tray app")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                // if you use the MenuItems, handle events here
                // .on_menu_event(|app, event| match event.id().as_ref() {
                //     "quit" => app.exit(0),
                //     "hide" => {
                //         dbg!("menu item hide clicked");
                //         let window = app.get_webview_window("main").unwrap();
                //         window.hide().unwrap();
                //     }
                //     "show" => {
                //         dbg!("menu item show clicked");
                //         let window = app.get_webview_window("main").unwrap();
                //         window.show().unwrap();
                //     }
                //     _ => {}
                // })
                .on_tray_icon_event(|tray_icon, event| match event.click_type {
                    ClickType::Left => {
                        dbg!("system tray received a left click");

                        let window = tray_icon.app_handle().get_webview_window("main").unwrap();
                        let _ = window.show().unwrap();
                        let logical_size = tauri::LogicalSize::<f64> {
                            width: 300.00,
                            height: 400.00,
                        };
                        let logical_s = tauri::Size::Logical(logical_size);
                        let _ = window.set_size(logical_s);
                        let logical_position = tauri::LogicalPosition::<f64> {
                            x: event.position.x - logical_size.width,
                            y: event.position.y - logical_size.height - 30.,
                        };
                        let logical_pos: tauri::Position =
                            tauri::Position::Logical(logical_position);
                        let _ = window.set_position(logical_pos);
                        let _ = window.set_focus();
                    }
                    ClickType::Right => {
                        dbg!("system tray received a right click");
                        let window = tray_icon.app_handle().get_webview_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    ClickType::Double => {
                        dbg!("system tray received a double click");
                    }
                })
                .build(app);

            /* global shortcut setup */

            // let ctrl_alt_space_shortcut = Shortcut::new(Some(Modifiers::CAPS_LOCK), Code::Space);
            // let _ = app.global_shortcut().on_all_shortcuts(
            //     [ctrl_alt_space_shortcut].into_iter(),
            //     move |_app, shortcut, _shortcut_event| {
            //         if shortcut == &ctrl_alt_space_shortcut {
            //             println!("Ctrl-N Detected!");
            //         }
            //     },
            // );

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
