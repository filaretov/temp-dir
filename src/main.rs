use std::fs::create_dir;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn now() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Is your system time before the Unix epoch?")
        .as_millis()
        .to_string()
}

fn main() {
    let sys_temp = std::env::temp_dir();
    let new_temp_dir = loop {
        let now = now();
        let subdir = format!("temp-dir-{}", now);
        let candidate = sys_temp.join(Path::new(&subdir));
        if !candidate.exists() {
            break candidate;
        }
    };
    create_dir(&new_temp_dir).expect("Failed to create directory");
    print!("{}", new_temp_dir.display());
}
