use std::path::PathBuf;
use anyhow::Result;

pub struct FileManager {
    current_file: Option<PathBuf>,
    last_directory: Option<PathBuf>,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            current_file: None,
            last_directory: None,
        }
    }

    pub fn open_file(&mut self, path: &PathBuf) -> Result<String> {
        // TODO: Реализовать открытие файла
        // 1. Проверить существование файла
        // 2. Прочитать содержимое файла
        // 3. Определить кодировку
        // 4. Обновить текущий файл
        todo!()
    }

    pub fn save_file(&self, path: &PathBuf, content: &str) -> Result<()> {
        // TODO: Реализовать сохранение файла
        // 1. Проверить права доступа
        // 2. Создать резервную копию если файл существует
        // 3. Записать содержимое
        // 4. Обновить текущий файл
        todo!()
    }
}



