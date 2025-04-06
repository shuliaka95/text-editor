mod buffer;
mod commands;

use anyhow::Result;

pub use buffer::{Buffer, Position};
pub use commands::{Command, CommandExecutor};

pub struct Editor {
    buffer: Buffer,
    command_history: Vec<Command>,
    undo_stack: Vec<Command>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            command_history: Vec::new(),
            undo_stack: Vec::new(),
        }
    }

    pub fn execute_command(&mut self, command: Command) -> Result<()> {
        // Реализуем выполнение команд
        // 1. Добавить команду в историю
        // 2. Очистить стек отмены
        // 3. Выполнить команду
        CommandExecutor::execute(self, command)
    }

    pub fn undo(&mut self) -> Result<()> {
        // Реализуем отмену последней команды
        // 1. Взять последнюю команду из истории
        // 2. Добавить её в стек отмены
        // 3. Выполнить обратную команду
        CommandExecutor::undo(self)
    }

    pub fn redo(&mut self) -> Result<()> {
        // Реализуем возврат отмененной команды
        // 1. Взять последнюю команду из стека отмены
        // 2. Добавить её обратно в историю
        // 3. Выполнить команду
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
}



