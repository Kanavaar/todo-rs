use directories;
use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use std::io::Write;

/// The Files and Directories used for storing the applications data
pub struct Files {
    data_dir: std::path::PathBuf,
    data_file: std::ffi::OsString,
}

impl Files {
    fn new(qualifier: &str, org: &str, name: &str) -> Files {
        let mut data_dir = (match directories::ProjectDirs::from(qualifier, org, name) {
            Some(project) => project,
            None => panic!("Could not get ProjectDirs"),
        })
        .data_dir()
        .to_owned();

        let mut data_file = data_dir.as_mut_os_str().to_owned();
        data_file.push("/todos.json");

        Files {
            data_dir,
            data_file,
        }
    }

    /// The directory used
    pub fn data_dir(&self) -> &std::path::PathBuf {
        &self.data_dir
    }

    /// The File used
    pub fn data_file(&self) -> &std::ffi::OsString {
        &self.data_file
    }
}

pub const PROJECT: Lazy<Files> = Lazy::new(|| Files::new("", "", "rodos"));

/// Creates file and directories used for application
pub fn init() {
    if !std::fs::metadata(PROJECT.data_dir()).is_ok() {
        std::fs::create_dir(PROJECT.data_dir()).unwrap();
        println!(
            "{} {}",
            "Created Directory:".green().bold(),
            PROJECT.data_dir().to_str().unwrap().black().italic()
        );
    };

    if !std::fs::metadata(PROJECT.data_file()).is_ok() {
        let mut file = std::fs::File::create(PROJECT.data_file()).unwrap();
        file.write_all(b"{\"data\":[]}").unwrap();
        println!(
            "{} {}",
            "Created Data File:".green().bold(),
            PROJECT.data_file().to_str().unwrap().black().italic()
        );
    }
}
