use crate::ast::ASTNode;
use crate::parser::ParseResult;
use crate::token::Token;
use crate::tree_view;
use eframe::egui;

// UI Constants
pub const TOKENS_SCROLL_HEIGHT: f32 = 150.0;
pub const AST_SCROLL_HEIGHT: f32 = 400.0;
pub const TOKEN_BG_COLOR: egui::Color32 = egui::Color32::from_rgb(220, 230, 255);
pub const TOKEN_TEXT_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 60, 150);
pub const IDENTIFIER_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 100, 200);
pub const SUCCESS_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 150, 0);

// AST Colors
const OPERATOR_COLOR: egui::Color32 = egui::Color32::from_rgb(220, 50, 50);
const NUMBER_COLOR: egui::Color32 = egui::Color32::from_rgb(50, 150, 220);
const VARIABLE_COLOR: egui::Color32 = egui::Color32::from_rgb(150, 100, 200);
const FUNCTION_COLOR: egui::Color32 = egui::Color32::from_rgb(220, 140, 50);

const DEFAULT_EXPRESSION: &str = "A = B + C";

pub struct ExpressionParserApp {
    input: String,
    parse_result: Option<ParseResult>,
    error: Option<String>,
}

impl Default for ExpressionParserApp {
    fn default() -> Self {
        Self {
            input: DEFAULT_EXPRESSION.to_string(),
            parse_result: None,
            error: None,
        }
    }
}

impl ExpressionParserApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn process_expression(&mut self) {
        self.error = None;
        self.parse_result = None;

        match ParseResult::from_input(&self.input) {
            Ok(result) => {
                self.parse_result = Some(result);
            }
            Err(err) => {
                self.error = Some(err.to_string());
            }
        }
    }

    fn render_header(&self, ui: &mut egui::Ui) {
        ui.heading("🔬 Mathematical Expression Parser");
        ui.label("Compiler Design - Lexical & Syntactic Analysis");
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);
    }

    fn render_input_section(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Expression:");
            let response = ui.text_edit_singleline(&mut self.input);

            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.process_expression();
            }

            if ui.button("⚡ Parse").clicked() {
                self.process_expression();
            }
        });

        ui.add_space(5.0);
    }

    fn render_examples(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.label("Examples:");

            let examples = [
                "A = B + C",
                "sqrt(13-(6-0)^2) - 10",
                "a + b * c",
                "x^2 + 2*x + 1",
                "result = alpha * beta",
            ];

            for example in examples {
                if ui.small_button(example).clicked() {
                    self.input = example.to_string();
                }
            }
        });

        ui.add_space(10.0);
    }

    fn render_error(&self, ui: &mut egui::Ui) {
        if let Some(error) = &self.error {
            ui.colored_label(egui::Color32::RED, format!("❌ Error: {}", error));
            ui.add_space(10.0);
        }
    }

    fn render_identifier_table(&self, ui: &mut egui::Ui, table: &[(String, usize)]) {
        ui.group(|ui| {
            ui.heading("🔤 Identifier Table");
            ui.add_space(5.0);

            egui::Grid::new("id_table").striped(true).show(ui, |ui| {
                ui.label(egui::RichText::new("Identifier").strong());
                ui.label(egui::RichText::new("Index").strong());
                ui.end_row();

                for (name, idx) in table {
                    ui.label(egui::RichText::new(name).monospace());
                    ui.label(
                        egui::RichText::new(format!("id{}", idx))
                            .monospace()
                            .color(IDENTIFIER_COLOR),
                    );
                    ui.end_row();
                }
            });
        });

        ui.add_space(10.0);
    }

    fn render_tokens(&self, ui: &mut egui::Ui, tokens: &[Token]) {
        ui.group(|ui| {
            ui.heading("📋 Lexical Analysis (Tokens)");
            ui.add_space(5.0);

            egui::ScrollArea::vertical()
                .id_salt("tokens_scroll")
                .max_height(TOKENS_SCROLL_HEIGHT)
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for token in tokens {
                            ui.label(
                                egui::RichText::new(format!("{}", token))
                                    .background_color(TOKEN_BG_COLOR)
                                    .color(TOKEN_TEXT_COLOR)
                                    .monospace(),
                            );
                        }
                    });
                });
        });

        ui.add_space(10.0);
    }

    fn render_ast(&self, ui: &mut egui::Ui, ast: &ASTNode) {
        ui.group(|ui| {
            ui.heading("🌳 Abstract Syntax Tree");
            ui.add_space(5.0);

            tree_view::render_tree(ui, ast, AST_SCROLL_HEIGHT);
        });

        ui.add_space(10.0);
    }

    fn render_result(&self, ui: &mut egui::Ui, ast: &ASTNode) {
        ui.group(|ui| {
            ui.heading("📊 Evaluation Result");
            ui.add_space(5.0);

            if ast.has_variables() {
                ui.label(
                    egui::RichText::new("Expression contains variables - no numeric evaluation")
                        .italics()
                        .color(egui::Color32::GRAY),
                );
            } else {
                let result = ast.evaluate();
                ui.label(
                    egui::RichText::new(format!("Result: {}", result))
                        .size(24.0)
                        .color(SUCCESS_COLOR)
                        .strong(),
                );
            }
        });
    }

    fn render_legend(&self, ui: &mut egui::Ui) {
        ui.collapsing("ℹ️ Supported Operators", |ui| {
            ui.label("= : Assignment");
            ui.label("+ : Addition");
            ui.label("- : Subtraction");
            ui.label("* : Multiplication");
            ui.label("/ : Division");
            ui.label("^ : Power");
            ui.label("sqrt() : Square Root");
            ui.label("a-z, A-Z : Identifiers");
            ui.label("( ) : Parentheses");
        });
    }

    fn render_results(&self, ui: &mut egui::Ui, result: &ParseResult) {
        ui.separator();
        ui.add_space(10.0);

        if !result.identifier_table.is_empty() {
            self.render_identifier_table(ui, &result.identifier_table);
        }

        self.render_tokens(ui, &result.tokens);
        self.render_ast(ui, &result.ast);
        self.render_result(ui, &result.ast);
    }
}

impl eframe::App for ExpressionParserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_header(ui);
            self.render_input_section(ui);
            self.render_examples(ui);
            self.render_error(ui);

            if let Some(result) = &self.parse_result {
                self.render_results(ui, result);
            }

            ui.add_space(10.0);
            ui.separator();
            self.render_legend(ui);
        });
    }
}
