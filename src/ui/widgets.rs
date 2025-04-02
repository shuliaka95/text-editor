use egui::{Color32, RichText, Ui, Button, Label};

use crate::editor::Position;

pub struct TextLine {
    text: String,
    line_number: usize,
    cursor_pos: Option<Position>,
}

impl TextLine {
    pub fn new(text: String, line_number: usize, cursor_pos: Option<Position>) -> Self {
        Self {
            text,
            line_number,
            cursor_pos,
        }
    }

    pub fn draw(&self, ui: &mut Ui) {
        // TODO: Реализовать отрисовку строки
        // 1. Отрисовать номер строки
        // 2. Отрисовать текст
        // 3. Отрисовать курсор если есть
        // 4. Отрисовать выделение если есть
        todo!()
    }
}

pub struct LineNumber {
    number: usize,
}

impl LineNumber {
    pub fn new(number: usize) -> Self {
        Self { number }
    }

    pub fn draw(&self, ui: &mut Ui) {
        // TODO: Реализовать отрисовку номера строки
        // 1. Отрисовать номер в сером цвете
        // 2. Добавить отступ
        // 3. Выровнять по правому краю
        todo!()
    }
}



