use crate::editor::buffer::Position;
use anyhow::{Result, anyhow};

// Простое перечисление всех возможных команд
#[derive(Clone)]
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

// Структура для хранения состояния буфера перед выполнением команды
#[derive(Clone)]
struct BufferState {
    lines: Vec<String>,
    cursor: Position,
    selection: Option<(Position, Position)>,
}

// Интерфейс для выполнения команд
pub trait CommandExecutor {
    fn execute(&mut self, command: Command) -> Result<()>;
    fn undo(&mut self) -> Result<()>;
    fn redo(&mut self) -> Result<()>;
}

// Реализация CommandExecutor для Editor
impl CommandExecutor for crate::editor::Editor {
    fn execute(&mut self, command: Command) -> Result<()> {
        // Сохраняем текущее состояние буфера
        let current_state = self.save_buffer_state();
        
        // Выполняем команду
        let result = match command.clone() {
            Command::InsertChar(c) => {
                self.buffer.insert_char(c).map_err(|e| anyhow!("Ошибка вставки символа: {}", e))
            },
            Command::DeleteChar => {
                self.buffer.delete_char().map_err(|e| anyhow!("Ошибка удаления символа: {}", e))
            },
            Command::InsertNewline => {
                self.buffer.insert_newline().map_err(|e| anyhow!("Ошибка вставки новой строки: {}", e))
            },
            Command::MoveCursor(pos) => {
                self.buffer.set_cursor_position(pos).map_err(|e| anyhow!("Ошибка перемещения курсора: {}", e))
            },
            Command::Select(start, end) => {
                self.buffer.set_selection(start, end).map_err(|e| anyhow!("Ошибка выделения: {}", e))
            },
            Command::DeleteSelection => {
                self.buffer.delete_selection().map_err(|e| anyhow!("Ошибка удаления выделения: {}", e))
            },
            Command::Copy => {
                // Копирование выделенного текста в буфер обмена
                if let Some((start, end)) = self.buffer.get_selection() {
                    let text = self.buffer.get_text_in_range(start, end)
                        .map_err(|e| anyhow!("Ошибка получения текста: {}", e))?;
                    self.set_clipboard(text);
                    Ok(())
                } else {
                    Ok(()) // Нет выделения, ничего не делаем
                }
            },
            Command::Cut => {
                // Вырезание выделенного текста в буфер обмена
                if let Some((start, end)) = self.buffer.get_selection() {
                    let text = self.buffer.get_text_in_range(start, end)
                        .map_err(|e| anyhow!("Ошибка получения текста: {}", e))?;
                    self.set_clipboard(text);
                    self.buffer.delete_selection().map_err(|e| anyhow!("Ошибка удаления выделения: {}", e))
                } else {
                    Ok(()) // Нет выделения, ничего не делаем
                }
            },
            Command::Paste => {
                // Вставка текста из буфера обмена
                if let Some(text) = self.get_clipboard() {
                    self.buffer.insert_text(text).map_err(|e| anyhow!("Ошибка вставки текста: {}", e))
                } else {
                    Ok(()) // Нет текста в буфере обмена, ничего не делаем
                }
            },
            Command::Undo => {
                // Отмена последней команды
                return self.undo();
            },
            Command::Redo => {
                // Возврат отмененной команды
                return self.redo();
            },
        };
        
        // Если команда выполнилась с ошибкой, восстанавливаем состояние буфера
        if let Err(e) = result {
            self.restore_buffer_state(current_state)?;
            return Err(e);
        }
        
        // Добавляем команду в историю
        self.command_history.push(command);
        
        // Очищаем стек отмены
        self.undo_stack.clear();
        
        Ok(())
    }
    
    fn undo(&mut self) -> Result<()> {
        // Проверяем, есть ли команды для отмены
        if let Some(command) = self.command_history.pop() {
            // Сохраняем команду в стек отмены
            self.undo_stack.push(command.clone());
            
            // Сохраняем текущее состояние буфера
            let current_state = self.save_buffer_state();
            
            // Выполняем обратную команду
            let result = match command {
                Command::InsertChar(_) => {
                    self.buffer.delete_char().map_err(|e| anyhow!("Ошибка отмены вставки символа: {}", e))
                },
                Command::DeleteChar => {
                    // Для отмены удаления нужно знать, какой символ был удален
                    // Это требует дополнительной информации, которую мы не храним
                    // В реальном приложении нужно хранить удаленный символ
                    Err(anyhow!("Невозможно отменить удаление символа без дополнительной информации"))
                },
                Command::InsertNewline => {
                    // Для отмены вставки новой строки нужно удалить символ новой строки
                    // и объединить строки
                    self.buffer.delete_char().map_err(|e| anyhow!("Ошибка отмены вставки новой строки: {}", e))
                },
                Command::MoveCursor(_) => {
                    // Возвращаем курсор на предыдущую позицию
                    // Для этого нужно хранить предыдущую позицию
                    // В реальном приложении нужно хранить предыдущую позицию
                    Err(anyhow!("Невозможно отменить перемещение курсора без предыдущей позиции"))
                },
                Command::Select(_, _) => {
                    // Отмена выделения - просто снимаем выделение
                    self.buffer.clear_selection().map_err(|e| anyhow!("Ошибка отмены выделения: {}", e))
                },
                Command::DeleteSelection => {
                    // Для отмены удаления выделения нужно знать, какой текст был удален
                    // Это требует дополнительной информации, которую мы не храним
                    Err(anyhow!("Невозможно отменить удаление выделения без дополнительной информации"))
                },
                Command::Copy => {
                    // Копирование не изменяет буфер, поэтому отмена не требуется
                    Ok(())
                },
                Command::Cut => {
                    // Для отмены вырезания нужно знать, какой текст был вырезан
                    // Это требует дополнительной информации, которую мы не храним
                    Err(anyhow!("Невозможно отменить вырезание без дополнительной информации"))
                },
                Command::Paste => {
                    // Для отмены вставки нужно удалить вставленный текст
                    // Это требует знания длины вставленного текста
                    // В реальном приложении нужно хранить длину вставленного текста
                    Err(anyhow!("Невозможно отменить вставку без дополнительной информации"))
                },
                Command::Undo | Command::Redo => {
                    // Эти команды обрабатываются отдельно
                    Ok(())
                },
            };
            
            // Если команда выполнилась с ошибкой, восстанавливаем состояние буфера
            if let Err(e) = result {
                self.restore_buffer_state(current_state)?;
                return Err(e);
            }
        }
        
        Ok(())
    }
    
    fn redo(&mut self) -> Result<()> {
        // Проверяем, есть ли команды для возврата
        if let Some(command) = self.undo_stack.pop() {
            // Сохраняем текущее состояние буфера
            let current_state = self.save_buffer_state();
            
            // Выполняем команду напрямую, без вызова execute_command
            let result = match command.clone() {
                Command::InsertChar(c) => {
                    self.buffer.insert_char(c).map_err(|e| anyhow!("Ошибка вставки символа: {}", e))
                },
                Command::DeleteChar => {
                    self.buffer.delete_char().map_err(|e| anyhow!("Ошибка удаления символа: {}", e))
                },
                Command::InsertNewline => {
                    self.buffer.insert_newline().map_err(|e| anyhow!("Ошибка вставки новой строки: {}", e))
                },
                Command::MoveCursor(pos) => {
                    self.buffer.set_cursor_position(pos).map_err(|e| anyhow!("Ошибка перемещения курсора: {}", e))
                },
                Command::Select(start, end) => {
                    self.buffer.set_selection(start, end).map_err(|e| anyhow!("Ошибка выделения: {}", e))
                },
                Command::DeleteSelection => {
                    self.buffer.delete_selection().map_err(|e| anyhow!("Ошибка удаления выделения: {}", e))
                },
                Command::Copy => {
                    // Копирование выделенного текста в буфер обмена
                    if let Some((start, end)) = self.buffer.get_selection() {
                        let text = self.buffer.get_text_in_range(start, end)
                            .map_err(|e| anyhow!("Ошибка получения текста: {}", e))?;
                        self.set_clipboard(text);
                        Ok(())
                    } else {
                        Ok(()) // Нет выделения, ничего не делаем
                    }
                },
                Command::Cut => {
                    // Вырезание выделенного текста в буфер обмена
                    if let Some((start, end)) = self.buffer.get_selection() {
                        let text = self.buffer.get_text_in_range(start, end)
                            .map_err(|e| anyhow!("Ошибка получения текста: {}", e))?;
                        self.set_clipboard(text);
                        self.buffer.delete_selection().map_err(|e| anyhow!("Ошибка удаления выделения: {}", e))
                    } else {
                        Ok(()) // Нет выделения, ничего не делаем
                    }
                },
                Command::Paste => {
                    // Вставка текста из буфера обмена
                    if let Some(text) = self.get_clipboard() {
                        self.buffer.insert_text(text).map_err(|e| anyhow!("Ошибка вставки текста: {}", e))
                    } else {
                        Ok(()) // Нет текста в буфере обмена, ничего не делаем
                    }
                },
                Command::Undo => {
                    // Отмена последней команды
                    return self.undo();
                },
                Command::Redo => {
                    // Возврат отмененной команды
                    return self.redo();
                },
            };
            
            // Если команда выполнилась с ошибкой, восстанавливаем состояние буфера
            if let Err(e) = result {
                self.restore_buffer_state(current_state)?;
                return Err(e);
            }
            
            // Добавляем команду в историю
            self.command_history.push(command);
        }
        
        Ok(())
    }
}

// Расширение для Editor для поддержки команд
impl crate::editor::Editor {
    // Сохраняет текущее состояние буфера
    fn save_buffer_state(&self) -> BufferState {
        BufferState {
            lines: self.buffer.get_all_lines().iter().cloned().collect(),
            cursor: self.buffer.get_cursor_position(),
            selection: self.buffer.get_selection(),
        }
    }
    
    // Восстанавливает состояние буфера
    fn restore_buffer_state(&mut self, state: BufferState) -> Result<()> {
        // Очищаем буфер
        self.buffer.clear().map_err(|e| anyhow!("Ошибка очистки буфера: {}", e))?;
        
        // Восстанавливаем строки
        for line in state.lines {
            self.buffer.append_line(line).map_err(|e| anyhow!("Ошибка добавления строки: {}", e))?;
        }
        
        // Восстанавливаем позицию курсора
        self.buffer.set_cursor_position(state.cursor).map_err(|e| anyhow!("Ошибка установки позиции курсора: {}", e))?;
        
        // Восстанавливаем выделение
        if let Some((start, end)) = state.selection {
            self.buffer.set_selection(start, end).map_err(|e| anyhow!("Ошибка установки выделения: {}", e))?;
        }
        
        Ok(())
    }
    
    // Устанавливает текст в буфер обмена
    fn set_clipboard(&mut self, text: String) {
        // В реальном приложении здесь будет код для работы с буфером обмена
        // Для простоты используем статическую переменную
        use std::sync::Mutex;
        use lazy_static::lazy_static;
        
        lazy_static! {
            static ref CLIPBOARD: Mutex<Option<String>> = Mutex::new(None);
        }
        
        if let Ok(mut clipboard) = CLIPBOARD.lock() {
            *clipboard = Some(text);
        }
    }
    
    // Получает текст из буфера обмена
    fn get_clipboard(&self) -> Option<String> {
        // В реальном приложении здесь будет код для работы с буфером обмена
        // Для простоты используем статическую переменную
        use std::sync::Mutex;
        use lazy_static::lazy_static;
        
        lazy_static! {
            static ref CLIPBOARD: Mutex<Option<String>> = Mutex::new(None);
        }
        
        if let Ok(clipboard) = CLIPBOARD.lock() {
            clipboard.clone()
        } else {
            None
        }
    }
}
