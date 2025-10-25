mod ast;
mod error;
mod lexer;
mod parser;
mod token;
mod tree_view;
mod ui;

use eframe::egui;
use ui::ExpressionParserApp;

const WINDOW_WIDTH: f32 = 900.0;
const WINDOW_HEIGHT: f32 = 700.0;
const MIN_WINDOW_WIDTH: f32 = 600.0;
const MIN_WINDOW_HEIGHT: f32 = 400.0;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_min_inner_size([MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT]),
        ..Default::default()
    };

    eframe::run_native(
        "Expression Parser - Compiler Design",
        options,
        Box::new(|cc| Ok(Box::new(ExpressionParserApp::new(cc)))),
    )
}
