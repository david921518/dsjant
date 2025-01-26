#![deny(warnings)]

use std::env;
use std::fs;
use std::thread;
use std::time::Duration;
use warp::Filter;

#[tokio::main]
async fn server_main(port: u16, root_path: String) {
    let route = warp::get()
        .and(warp::path("demo"))
        .and(warp::fs::dir(root_path));

    warp::serve(route)
        .run(([127, 0, 0, 1], port))
        .await;
}

static mut SERVER_START: bool = false;

fn server_thread(port: &str, root_path: &str) {
    unsafe {
        if SERVER_START == false {
            let my_port = port.to_string().clone();
            let my_root_path = root_path.to_string().clone();
            thread::spawn(move || {
                println!("Server thread 1!");
                let u_port = my_port.parse::<u16>().unwrap();
                server_main(u_port, my_root_path);
                println!("Server thread 2!");
            });
            thread::sleep(Duration::from_millis(1));
            SERVER_START = true;
        } else {
            println!("Server has been started already!");
        }
    }
}

#[tauri::command]
pub fn start_server(port: &str, root_path: &str) -> String {
    println!("Hello, port: {}!", port);
    println!("root_path: {}!", root_path);
    server_thread(port, root_path);
    println!("Hello2, {}!", port);
    format!("Hello, {}!", port)
}

#[tauri::command]
pub fn stop_server(port: &str) -> String {
    println!("Bye, {}!", port);
    format!("Bye, {}!", port)
}

#[tauri::command]
pub fn get_app_root_dir() -> String {
    let res = env::current_exe();
    match res {
        Ok(exe_path) => {
            let root_path = exe_path.parent().unwrap().to_path_buf().clone();
            println!("default root_path={}", root_path.to_string_lossy());
            if root_path.is_dir() {
                println!("root_path={} is a dir", root_path.to_string_lossy());
            } else {
                match root_path.try_exists() {
                    Ok(try_res) => {
                        if try_res == true {
                            println!(
                                "root_path={} exists but not a directory, remove it.",
                                root_path.to_string_lossy()
                            );
                            let rm_res = fs::remove_file(root_path.as_os_str());
                            match rm_res {
                                Ok(_) => {
                                    println!("remove {} OK.", root_path.to_string_lossy());
                                }
                                Err(e) => {
                                    println!("remove error {}", e);
                                }
                            }
                        }
                        let cr_res = fs::create_dir(root_path.as_os_str());
                        match cr_res {
                            Ok(_) => {
                                println!("create_dir {} OK.", root_path.to_string_lossy());
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
            println!("{}", root_path.to_string_lossy());
            return format!("{}", root_path.to_string_lossy());
        }
        Err(e) => {
            println!("current_exe() error={}", e);
        }
    }
    format!("")
}