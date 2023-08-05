use directories;
use once_cell::sync::Lazy;

struct Files {
    data_dir: std::path::PathBuf,
    data_file: std::ffi::OsString,
}

impl Files {
    fn new(qualifier: &str, org: &str, name: &str) -> Files {
        let mut data_dir = (match directories::ProjectDirs::from(qualifier, org, name) {
            Some(project) => project,
            None => panic!("Could not get ProjectDirs"),
        }).data_dir().to_owned();

        let mut data_file = data_dir.as_mut_os_str().to_owned();
        data_file.push("/todos.json");

        Files {data_dir, data_file}
    }

    fn data_dir(&self) -> &std::path::PathBuf {
        &self.data_dir
    }

    fn data_file(&self) -> &std::ffi::OsString {
        &self.data_file
    }
}

const PROJECT: Lazy<Files> = Lazy::new(|| {
    Files::new("", "", "rodos")
});

pub fn init() {
    dbg!(PROJECT.data_dir());
    dbg!(PROJECT.data_file());
}