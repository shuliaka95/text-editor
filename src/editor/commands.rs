use crate::editor::buffer::Position;

// Простое перечисление всех возможных команд
pub enum Command {
    InsertChar(char),           // Вставка символа
    DeleteChar,                 // Удаление символа
    InsertNewline,              // Вставка новой строки
    MoveCursor(Position),       // Перемещение курсора
    Select(Position, Position), // Выделение текста
    DeleteSelection,            // Удаление выделения
    Copy,                       // Копирование
    Cut,                        // Вырезание
    Paste,                      // Вставка
    Undo,                       // Отмена
    Redo,                       // Возврат
}

// Интерфейс для выполнения команд
pub trait CommandExecutor {
    fn execute(&mut self, command: Command) -> anyhow::Result<()>;
    fn undo(&mut self) -> anyhow::Result<()>;
    fn redo(&mut self) -> anyhow::Result<()>;
}
