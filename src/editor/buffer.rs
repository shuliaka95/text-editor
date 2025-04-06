// Импортируем нужные типы из стандартной библиотеки
use std::collections::VecDeque;

// Структура для хранения позиции курсора в тексте
// line - номер строки (начиная с 0)
// column - номер столбца в строке (начиная с 0)
#[derive(Copy, Clone)]
pub struct Position {
    pub line: usize,    // Номер строки
    pub column: usize,  // Номер столбца
}

// Основная структура для работы с текстом
// lines - массив строк текста
// cursor - текущая позиция курсора
// selection - выделение текста (начало и конец)
pub struct Buffer {
    lines: VecDeque<String>,  // Строки текста
    cursor: Position,         // Позиция курсора
    selection: Option<(Position, Position)>,  // Выделение текста (начало и конец)
}

// Реализация методов для работы с буфером
impl Buffer {
    // Создаем новый пустой буфер
    // Возвращает буфер с одной пустой строкой
    pub fn new() -> Self {
        Self {
            lines: VecDeque::from(vec![String::new()]),  // Создаем одну пустую строку
            cursor: Position { line: 0, column: 0 },      // Курсор в начале
            selection: None,                              // Нет выделения
        }
    }

    // Удаляем выделенный текст
    // Возвращает Ok(()) если удаление успешно
    // Возвращает ошибку если нет выделения
    pub fn delete_selection(&mut self) -> Result<(), String> {
        // Проверяем есть ли выделение
        if let Some((start, end)) = &self.selection {
            // Сохраняем координаты начала и конца выделения
            let start_line = start.line;
            let start_col = start.column;
            let end_line = end.line;
            let end_col = end.column;

            // Если выделение в одной строке
            if start_line == end_line {
                // Получаем строку для редактирования
                let line = &mut self.lines[start_line];
                // Удаляем выделенный текст
                line.replace_range(start_col..end_col, "");
            } else {
                // Если выделение в нескольких строках
                // Берем начало первой строки
                let first_part = &self.lines[start_line][..start_col];
                // Берем конец последней строки
                let last_part = &self.lines[end_line][end_col..];
                // Соединяем их
                let new_line = format!("{}{}", first_part, last_part);
                
                // Удаляем строки между началом и концом выделения
                for _ in start_line + 1..=end_line {
                    self.lines.remove(start_line + 1);
                }
                
                // Обновляем первую строку
                self.lines[start_line] = new_line;
            }

            // Очищаем выделение
            self.selection = None;
            // Ставим курсор в начало выделения
            self.cursor = Position {
                line: start_line,
                column: start_col,
            };
            Ok(())
        } else {
            Err("Нет выделения".to_string())
        }
    }

    // Вставляем символ в текущую позицию курсора
    // Возвращает Ok(()) если вставка успешна
    // Возвращает ошибку если что-то пошло не так
    pub fn insert_char(&mut self, c: char) -> Result<(), String> {
        // Если есть выделение, удаляем его
        if let Some((start, _)) = self.selection.take() {
            self.delete_selection()?;
            // Ставим курсор в начало выделения
            self.cursor = Position {
                line: start.line,
                column: start.column,
            };
        }

        // Проверяем что строка существует
        if self.cursor.line >= self.lines.len() {
            self.lines.push_back(String::new());
        }

        // Получаем текущую строку
        let line = &mut self.lines[self.cursor.line];
        
        // Проверяем позицию курсора
        if self.cursor.column > line.len() {
            self.cursor.column = line.len();
        }

        // Обрабатываем специальные символы
        match c {
            '\n' => {
                // Разбиваем строку на две части
                let left = line[..self.cursor.column].to_string();
                let right = line[self.cursor.column..].to_string();
                
                // Обновляем текущую строку
                *line = left;
                
                // Добавляем новую строку
                self.lines.insert(self.cursor.line + 1, right);
                
                // Переходим на новую строку
                self.cursor.line += 1;
                self.cursor.column = 0;
                return Ok(());
            }
            '\t' => {
                // Вставляем 4 пробела вместо табуляции
                for _ in 0..4 {
                    line.insert(self.cursor.column, ' ');
                    self.cursor.column += 1;
                }
                return Ok(());
            }
            _ => {
                // Просто вставляем символ
                line.insert(self.cursor.column, c);
                self.cursor.column += 1;
            }
        }

        Ok(())
    }

    // Удаляем символ перед курсором
    // Возвращает Ok(()) если удаление успешно
    // Возвращает ошибку если что-то пошло не так
    pub fn delete_char(&mut self) -> Result<(), String> {
        // Проверяем что курсор в пределах буфера
        if self.cursor.line >= self.lines.len() {
            return Err("Курсор за пределами буфера".to_string());
        }

        // Получаем текущую строку
        let line = &mut self.lines[self.cursor.line];
        
        // Проверяем что курсор в пределах строки
        if self.cursor.column >= line.len() {
            return Err("Курсор за пределами строки".to_string());
        }

        // Проверяем что строка не пустая
        if line.is_empty() {
            return Err("Строка пуста".to_string());
        }

        // Проверяем что курсор не в начале строки
        if self.cursor.column == 0 {
            return Err("Курсор в начале строки".to_string());
        }
        
        // Удаляем символ
        line.remove(self.cursor.column - 1);
        
        // Обновляем позицию курсора
        self.cursor.column -= 1;
        
        Ok(())
    }

    // Вставляем новую строку
    // Возвращает Ok(()) если вставка успешна
    // Возвращает ошибку если что-то пошло не так
    pub fn insert_newline(&mut self) -> Result<(), String> {
        // Проверяем что курсор в пределах буфера
        if self.cursor.line >= self.lines.len() {
            return Err("Курсор за пределами буфера".to_string());
        }

        // Получаем текущую строку
        let line = self.get_current_line();
        
        // Проверяем что курсор в пределах строки
        if self.cursor.column > line.len() {
            return Err("Курсор за пределами строки".to_string());
        }

        // Проверяем что буфер не переполнен
        if self.lines.len() >= usize::MAX {
            return Err("Буфер переполнен".to_string());
        }

        // Вставляем символ новой строки
        self.insert_char('\n')
    }

    // Получаем весь текст из буфера
    // Возвращает Ok(String) если получение успешно
    // Возвращает ошибку если что-то пошло не так
    pub fn get_text(&self) -> Result<String, String> {
        // Проверяем что буфер не пустой
        if self.lines.is_empty() {
            return Err("Буфер пуст".to_string());
        }

        // Проверяем что есть хотя бы одна строка
        if self.lines.len() == 0 {
            return Err("Нет строк в буфере".to_string());
        }

        // Проверяем что все строки валидны
        for (i, line) in self.lines.iter().enumerate() {
            if line.is_empty() && i != self.lines.len() - 1 {
                return Err("Обнаружена пустая строка в середине буфера".to_string());
            }
        }

        // Объединяем все строки с разделителем новой строки
        let text = self.lines
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join("\n");

        Ok(text)
    }

    // Устанавливаем новый текст в буфер
    // Возвращает Ok(()) если установка успешна
    // Возвращает ошибку если что-то пошло не так
    pub fn set_text(&mut self, text: String) -> Result<(), String> {
        // Проверяем что текст не пустой
        if text.is_empty() {
            return Err("Текст не может быть пустым".to_string());
        }

        // Проверяем что текст не слишком большой
        if text.len() > usize::MAX {
            return Err("Текст слишком большой".to_string());
        }
        // Проверяем что текст содержит только валидные символы
        if text.chars().any(|c| c.is_control() && c != '\n' && c != '\t' && c != '\r') {
            return Err("Текст содержит недопустимые управляющие символы".to_string());
        }

        //устанавливаем новый текст в буфер
        //разбиваем текст на строки
        let lines: Vec<String> = text
            .split('\n')
            .map(|s| s.to_string())
            .collect();

        //замена табуляции на пробелы
        let lines: Vec<String> = lines
            .iter()
            .map(|s| s.replace('\t', "    "))
            .collect();

        //проверка на количество строк
        if lines.len() > usize::MAX {
            return Err("Слишком много строк".to_string());
        }

        //если нет строк, добавляем одну пустую
        if lines.is_empty() {
            self.lines.clear();
            self.lines.push_back(String::new());
        }
        else{
            //очищаем текущий буфер и добавляем новые строки
            self.lines.clear();
            for line in lines {
                self.lines.push_back(line);
            }
        }

        //сбрасываем позицию курсора в начало
        self.cursor = Position { line: 0, column: 0 };

        //очищаем выделение
        self.selection = None;

        Ok(())
    }
    
    pub fn get_cursor_position(&self) -> Position {
        self.cursor
    }

    pub fn get_all_lines(&self) -> &VecDeque<String> {
        &self.lines
    }

    pub fn get_current_line(&self) -> &str {
        if self.cursor.line < self.lines.len() {
            &self.lines[self.cursor.line]
        } else {
            ""
        }
    }
    
    // Новые методы для поддержки команд
    
    // Устанавливает позицию курсора
    pub fn set_cursor_position(&mut self, pos: Position) -> Result<(), String> {
        // Проверяем что позиция в пределах буфера
        if pos.line >= self.lines.len() {
            return Err("Позиция за пределами буфера".to_string());
        }
        
        // Проверяем что позиция в пределах строки
        let line = &self.lines[pos.line];
        if pos.column > line.len() {
            return Err("Позиция за пределами строки".to_string());
        }
        
        // Устанавливаем позицию курсора
        self.cursor = pos;
        
        Ok(())
    }
    
    // Устанавливает выделение
    pub fn set_selection(&mut self, start: Position, end: Position) -> Result<(), String> {
        // Проверяем что начальная позиция в пределах буфера
        if start.line >= self.lines.len() {
            return Err("Начальная позиция за пределами буфера".to_string());
        }
        
        // Проверяем что конечная позиция в пределах буфера
        if end.line >= self.lines.len() {
            return Err("Конечная позиция за пределами буфера".to_string());
        }
        
        // Проверяем что начальная позиция в пределах строки
        let start_line = &self.lines[start.line];
        if start.column > start_line.len() {
            return Err("Начальная позиция за пределами строки".to_string());
        }
        
        // Проверяем что конечная позиция в пределах строки
        let end_line = &self.lines[end.line];
        if end.column > end_line.len() {
            return Err("Конечная позиция за пределами строки".to_string());
        }
        
        // Устанавливаем выделение
        self.selection = Some((start, end));
        
        Ok(())
    }
    
    // Получает выделение
    pub fn get_selection(&self) -> Option<(Position, Position)> {
        self.selection
    }
    
    // Очищает выделение
    pub fn clear_selection(&mut self) -> Result<(), String> {
        self.selection = None;
        Ok(())
    }
    
    // Получает текст в указанном диапазоне
    pub fn get_text_in_range(&self, start: Position, end: Position) -> Result<String, String> {
        // Проверяем что начальная позиция в пределах буфера
        if start.line >= self.lines.len() {
            return Err("Начальная позиция за пределами буфера".to_string());
        }
        
        // Проверяем что конечная позиция в пределах буфера
        if end.line >= self.lines.len() {
            return Err("Конечная позиция за пределами буфера".to_string());
        }
        
        // Проверяем что начальная позиция в пределах строки
        let start_line = &self.lines[start.line];
        if start.column > start_line.len() {
            return Err("Начальная позиция за пределами строки".to_string());
        }
        
        // Проверяем что конечная позиция в пределах строки
        let end_line = &self.lines[end.line];
        if end.column > end_line.len() {
            return Err("Конечная позиция за пределами строки".to_string());
        }
        
        // Если выделение в одной строке
        if start.line == end.line {
            let line = &self.lines[start.line];
            return Ok(line[start.column..end.column].to_string());
        }
        
        // Если выделение в нескольких строках
        let mut result = String::new();
        
        // Добавляем первую строку
        let first_line = &self.lines[start.line];
        result.push_str(&first_line[start.column..]);
        result.push('\n');
        
        // Добавляем промежуточные строки
        for i in start.line + 1..end.line {
            result.push_str(&self.lines[i]);
            result.push('\n');
        }
        
        // Добавляем последнюю строку
        let last_line = &self.lines[end.line];
        result.push_str(&last_line[..end.column]);
        
        Ok(result)
    }
    
    // Вставляет текст в текущую позицию курсора
    pub fn insert_text(&mut self, text: String) -> Result<(), String> {
        // Проверяем что текст не пустой
        if text.is_empty() {
            return Ok(());
        }
        
        // Проверяем что текст не слишком большой
        if text.len() > usize::MAX {
            return Err("Текст слишком большой".to_string());
        }
        
        // Проверяем что текст содержит только валидные символы
        if text.chars().any(|c| c.is_control() && c != '\n' && c != '\t' && c != '\r') {
            return Err("Текст содержит недопустимые управляющие символы".to_string());
        }
        
        // Вставляем каждый символ
        for c in text.chars() {
            self.insert_char(c)?;
        }
        
        Ok(())
    }
    
    // Очищает буфер
    pub fn clear(&mut self) -> Result<(), String> {
        self.lines.clear();
        self.lines.push_back(String::new());
        self.cursor = Position { line: 0, column: 0 };
        self.selection = None;
        Ok(())
    }
    
    // Добавляет строку в конец буфера
    pub fn append_line(&mut self, line: String) -> Result<(), String> {
        self.lines.push_back(line);
        Ok(())
    }
}

