use tauri::{SystemTray, CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

pub fn init_sys_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    return SystemTray::new().with_menu(tray_menu);
}
