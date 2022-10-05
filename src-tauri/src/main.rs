#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rand::thread_rng;
use rand::seq::SliceRandom;

mod words;

/// The default hash cost to use for generating
/// bcrypt password hashes.
const HASH_COST: u32 = 8;

/// GeneratedPassword stores a randomly generated password
/// and the bcrypt hash of the password.
#[derive(serde::Serialize)]
struct GeneratedPassword {
    password: String,
    passhash: String,
}

/// Generates a random password from the word list, made up of `nwords`
/// words, joined together by `separator`.
#[tauri::command]
fn generate_password(nwords: u8, separator: &str) -> Result<GeneratedPassword, String> {
    // Setup a random number generator
    // TODO - Use a single instance of this, managed by Tauri
    let mut rng = thread_rng();
    let mut words: Vec<String> = Vec::new();

    // Generate `nwords` random words
    for _ in 0..nwords {
        // Choose a random word from the list
        let word = if let Some(w) = words::WORD_LIST.choose(&mut rng) {
            w
        } else {
            return Err("No words found".to_string());
        };

        // Add the word to the list
        words.push(word.to_string());
    }

    // Join the words together with the separator
    let pass = words.join(separator);

    // Hash the password
    let hash = bcrypt::hash(pass.as_bytes(), HASH_COST)
        .or_else(|_| 
            Err("Failed to hash password".to_string()
        ))?;

    // Return the password and hash
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
