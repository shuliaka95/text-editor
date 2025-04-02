use std::collections::VecDeque;

// Простая структура для хранения позиции курсора
pub struct Position {
    pub line: usize,
    pub column: usize,
}

pub struct Buffer {
    lines: VecDeque<String>,
    cursor: Position,
    selection: Option<(Position, Position)>,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            lines: VecDeque::from(vec![String::new()]),
            cursor: Position { line: 0, column: 0 },
            selection: None,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        // TODO: Реализовать вставку символа
        // 1. Проверить позицию курсора
        // 2. Вставить символ в нужную строку
        // 3. Обновить позицию курсора
        todo!()
    }

    pub fn delete_char(&mut self) {
        // TODO: Реализовать удаление символа
        // 1. Проверить позицию курсора
        // 2. Удалить символ из строки
        // 3. Обновить позицию курсора
        todo!()
    }

    pub fn insert_newline(&mut self) {
        // TODO: Реализовать вставку новой строки
        // 1. Разделить текущую строку по позиции курсора
        // 2. Вставить новую строку
        // 3. Обновить позицию курсора
        todo!()
    }

    pub fn get_text(&self) -> String {
        // TODO: Реализовать получение текста
        // 1. Объединить все строки
        // 2. Добавить переносы строк
        todo!()
    }

    pub fn set_text(&mut self, text: String) {
        // TODO: Реализовать установку текста
        // 1. Разбить текст на строки
        // 2. Очистить текущий буфер
        // 3. Установить новые строки
        todo!()
    }
}

