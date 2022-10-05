#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use rand::seq::SliceRandom;
// use tauri::Manager;

use rand::thread_rng;
use rand::seq::SliceRandom;

mod words;


const HASH_COST: u32 = 8;

#[derive(serde::Serialize)]
struct GeneratedPassword {
    password: String,
    passhash: String,
}


#[tauri::command]
fn generate_password(nwords: u8, separator: &str) -> Result<GeneratedPassword, String> {
    let mut rng = thread_rng();
    let mut words: Vec<String> = Vec::new();

    for _ in 0..nwords {
        let word = if let Some(w) = words::WORD_LIST.choose(&mut rng) {
            w
        } else {
            return Err("No words found".to_string());
        };

        words.push(word.to_string());
    }

    let pass = words.join(separator);

    let hash = bcrypt::hash(pass.as_bytes(), HASH_COST)
        .or_else(|_| Err("Failed to hash password".to_string()))?;

    Ok(GeneratedPassword {
        password: pass,
        passhash: hash,
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_password,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
