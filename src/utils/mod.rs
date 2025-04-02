mod file;

pub use file::FileManager;

pub struct Utils {
    file_manager: FileManager,
}

impl Utils {
    pub fn new() -> Self {
        Self {
            file_manager: FileManager::new(),
        }
    }
}



