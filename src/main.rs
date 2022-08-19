mod cli;

use std::path::{Path, PathBuf};
fn main() {
    // Get config path
    let config_path = std::env::var("XDG_CONFIG_HOME");

    let app_config_path_buf: PathBuf = match config_path {
        Ok(config_home) => Path::new(&config_home).join(Path::new("iforgor")),
        Err(_) => {
            let home_path = std::env::var("HOME").unwrap();
            // If env var doesnt exist fall back to home directory
            Path::new(&home_path).join(Path::new(".config/iforgor"))
        }
    };

    let mut app = cli::Cli::new(app_config_path_buf);

    app.run();
}
