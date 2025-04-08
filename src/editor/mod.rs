mod buffer;
mod commands;

use anyhow::Result;
use std::sync::Arc;
use std::sync::Mutex;

pub use buffer::{Buffer, Position};
pub use commands::{Command, CommandExecutor};

// Структура для хранения буфера обмена
struct ClipboardData {
    content: Option<String>,
}

pub struct Editor {
    buffer: Buffer,
    command_history: Vec<Command>,
    undo_stack: Vec<Command>,
    clipboard: Arc<Mutex<ClipboardData>>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            command_history: Vec::new(),
            undo_stack: Vec::new(),
            clipboard: Arc::new(Mutex::new(ClipboardData { content: None })),
        }
    }

    pub fn execute_command(&mut self, command: Command) -> Result<()> {
        CommandExecutor::execute(self, command)
    }

    pub fn undo(&mut self) -> Result<()> {
        CommandExecutor::undo(self)
    }

    pub fn redo(&mut self) -> Result<()> {
        CommandExecutor::redo(self)
    }
    
    // Получает буфер
    pub fn get_buffer(&self) -> &Buffer {
        &self.buffer
    }
    
    // Получает мутабельную ссылку на буфер
    pub fn get_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    // Сохраняет текст в буфер обмена
    fn set_clipboard(&mut self, text: String) {
        if let Ok(mut clipboard) = self.clipboard.lock() {
            clipboard.content = Some(text);
        }
    }

    // Получает текст из буфера обмена
    fn get_clipboard(&self) -> Option<String> {
        if let Ok(clipboard) = self.clipboard.lock() {
            clipboard.content.clone()
        } else {
            None
        }
    }

    // Сохраняет текущее состояние буфера
    fn save_buffer_state(&self) -> commands::BufferState {
        commands::BufferState {
            lines: self.buffer.get_all_lines().iter().cloned().collect(),
            cursor: self.buffer.get_cursor_position(),
            selection: self.buffer.get_selection(),
        }
    }

    // Восстанавливает состояние буфера
    fn restore_buffer_state(&mut self, state: commands::BufferState) -> Result<()> {
        // Очищаем текущий буфер
        self.buffer.clear()?;
        
        // Восстанавливаем строки
        for line in state.lines {
            self.buffer.append_line(line)?;
        }
        
        // Восстанавливаем позицию курсора
        self.buffer.set_cursor_position(state.cursor)?;
        
        // Восстанавливаем выделение
        if let Some((start, end)) = state.selection {
            self.buffer.set_selection(start, end)?;
        }
        
        Ok(())
    }

    // Получает текущий текст из буфера
    pub fn get_text(&self) -> Result<String> {
        self.buffer.get_text()
    }

    // Устанавливает текст в буфер
    pub fn set_text(&mut self, text: String) -> Result<()> {
        self.buffer.set_text(text)
    }

    // Получает текущую позицию курсора
    pub fn get_cursor_position(&self) -> Position {
        self.buffer.get_cursor_position()
    }

    // Устанавливает позицию курсора
    pub fn set_cursor_position(&mut self, position: Position) -> Result<()> {
        self.buffer.set_cursor_position(position)
    }

    // Получает текущее выделение
    pub fn get_selection(&self) -> Option<(Position, Position)> {
        self.buffer.get_selection()
    }

    // Устанавливает выделение
    pub fn set_selection(&mut self, start: Position, end: Position) -> Result<()> {
        self.buffer.set_selection(start, end)
    }

    // Очищает выделение
    pub fn clear_selection(&mut self) -> Result<()> {
        self.buffer.clear_selection()
    }
}