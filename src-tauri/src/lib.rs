mod command;

use std::env;
use std::fs;
//use tauri::Manager;
//use tauri_plugin_log::{Target, TargetKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    //#[cfg(debug_assertions)]
    //let devtools = tauri_plugin_devtools::init();

    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .setup(|_app| {
            // setup default directories
            let res = env::current_exe();
            match res {
                Ok(exe_path) => {
                    let root_path = exe_path.parent().unwrap().to_path_buf().clone();
                    println!("default root_path={:?}", root_path);
                    if root_path.is_dir() {
                        println!("root_path={:?} is a dir", root_path);
                    } else {
                        match root_path.try_exists() {
                            Ok(try_res) => {
                                if try_res == true {
                                    println!(
                                        "root_path={:?} exists but not a directory, remove it.",
                                        root_path
                                    );
                                    let rm_res = fs::remove_file(root_path.as_os_str());
                                    match rm_res {
                                        Ok(_) => {
                                            println!("remove {:?} OK.", root_path);
                                        }
                                        Err(e) => {
                                            println!("remove error {}", e);
                                        }
                                    }
                                }
                                let cr_res = fs::create_dir(root_path.as_os_str());
                                match cr_res {
                                    Ok(_) => {
                                        println!("create_dir {:?} OK.", root_path);
                                    }
                                    Err(e) => {
                                        println!("create_dir error {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("try_exists() error={}", e);
                            }
                        }
                    }
                    let demo_path = root_path.join("demo");
                    println!("default demo_path={:?}", demo_path);
                    if demo_path.is_dir() {
                        println!("demo_path={:?} is a dir", demo_path);
                    } else {
                        match demo_path.try_exists() {
                            Ok(try_res) => {
                                if try_res == true {
                                    println!(
                                        "demo_path={:?} exists but not a directory, remove it.",
                                        demo_path
                                    );
                                    let rm_res = fs::remove_file(demo_path.as_os_str());
                                    match rm_res {
                                        Ok(_) => {
                                            println!("remove {:?} OK.", demo_path);
                                        }
                                        Err(e) => {
                                            println!("remove error {}", e);
                                        }
                                    }
                                }
                                let cr_res = fs::create_dir(demo_path.as_os_str());
                                match cr_res {
                                    Ok(_) => {
                                        println!("create_dir {:?} OK.", demo_path);
                                    }
                                    Err(e) => {
                                        println!("create_dir error {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("try_exists() error={}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("current_exe() error={}", e);
                }
            }
            Ok(())
        });

    //#[cfg(debug_assertions)]
    //{
    //    builder = builder.plugin(devtools);
    //}

    builder = builder.plugin(tauri_plugin_os::init());

    #[cfg(all(not(mobile)))]
    {
        builder = builder.plugin(tauri_plugin_window_state::Builder::default().build());
        builder = builder.plugin(tauri_plugin_system_info::init());
    }

    builder
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            command::server::get_app_root_dir,
            command::server::start_server,
            command::server::stop_server
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
