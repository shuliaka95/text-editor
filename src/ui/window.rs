use anyhow::Result;
use egui::{CentralPanel, Context, TopBottomPanel, Window};

use crate::editor::Editor;

pub struct App {
    editor: Editor,
    show_menu: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            editor: Editor::new(),
            show_menu: true,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        // TODO: Реализовать главный цикл приложения
        // 1. Создать окно
        // 2. Отрисовать меню
        // 3. Отрисовать область редактирования
        // 4. Обработать события
        todo!()
    }

    fn draw_menu(&mut self, ctx: &Context) {
        // TODO: Реализовать отрисовку меню
        // 1. Создать меню File (New, Open, Save)
        // 2. Создать меню Edit (Copy, Paste, Cut)
        // 3. Создать меню View (Settings)
        todo!()
    }

    fn draw_editor(&mut self, ctx: &Context) {
        // TODO: Реализовать отрисовку редактора
        // 1. Отрисовать номера строк
        // 2. Отрисовать текст
        // 3. Отрисовать курсор
        // 4. Отрисовать выделение
        todo!()
    }
}

