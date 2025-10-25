use crate::ast::ASTNode;
use crate::parser::ParseResult;
use crate::tree_view;
use eframe::egui;

// UI Constants
pub const TOKENS_SCROLL_HEIGHT: f32 = 150.0;
pub const AST_SCROLL_HEIGHT: f32 = 350.0;
pub const CODE_SCROLL_HEIGHT: f32 = 200.0;
pub const TOKEN_BG_COLOR: egui::Color32 = egui::Color32::from_rgb(220, 230, 255);
pub const TOKEN_TEXT_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 60, 150);
pub const IDENTIFIER_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 100, 200);
pub const SUCCESS_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 150, 0);
pub const WARNING_COLOR: egui::Color32 = egui::Color32::from_rgb(200, 120, 0);
pub const PHASE_HEADER_COLOR: egui::Color32 = egui::Color32::from_rgb(70, 130, 180);

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
        ui.heading("Compiler Design - Complete Pipeline");
        ui.label("Five Phases: Lexical → Syntax → Semantic → Intermediate Code → Optimization");
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

            if ui.button("⚡ Compile").clicked() {
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
                "sqrt(16) + 2 * 3",
                "a + b * c",
                "x^2 + 2*x + 1",
                "5 + 3 * 0",
                "(10 - 4) / 2",
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

    fn render_phase_header(&self, ui: &mut egui::Ui, phase_num: usize, title: &str) {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(format!("Phase {}", phase_num))
                    .size(18.0)
                    .color(PHASE_HEADER_COLOR)
                    .strong(),
            );
            ui.label(egui::RichText::new(title).size(18.0).strong());
        });
        ui.add_space(5.0);
    }

    fn render_identifier_table(&self, ui: &mut egui::Ui, table: &[(String, usize)]) {
        if table.is_empty() {
            return;
        }

        ui.group(|ui| {
            ui.label(egui::RichText::new("Symbol Table").strong());
            ui.add_space(3.0);

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

        ui.add_space(8.0);
    }

    fn render_phase1_lexical(&self, ui: &mut egui::Ui, result: &ParseResult) {
        ui.group(|ui| {
            self.render_phase_header(ui, 1, "Lexical Analysis");
            ui.label("Breaking down the input into tokens (lexemes)");
            ui.add_space(8.0);

            self.render_identifier_table(ui, &result.identifier_table);

            ui.label(egui::RichText::new("Tokens:").strong());
            ui.add_space(3.0);

            egui::ScrollArea::vertical()
                .id_salt("tokens_scroll")
                .max_height(TOKENS_SCROLL_HEIGHT)
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for token in &result.tokens {
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

        ui.add_space(15.0);
    }

    fn render_phase2_syntax(&self, ui: &mut egui::Ui, ast: &ASTNode) {
        ui.group(|ui| {
            self.render_phase_header(ui, 2, "Syntax Analysis (Parsing)");
            ui.label("Building Abstract Syntax Tree (AST) from tokens");
            ui.add_space(8.0);

            tree_view::render_tree(ui, ast, AST_SCROLL_HEIGHT);
        });

        ui.add_space(15.0);
    }

    fn render_phase3_semantic(&self, ui: &mut egui::Ui, warnings: &[String]) {
        ui.group(|ui| {
            self.render_phase_header(ui, 3, "Semantic Analysis");
            ui.label("Checking for semantic errors and type consistency");
            ui.add_space(8.0);

            if warnings.is_empty() {
                ui.label(
                    egui::RichText::new("✓ No semantic warnings detected")
                        .color(SUCCESS_COLOR)
                        .strong(),
                );
            } else {
                ui.label(
                    egui::RichText::new("Warnings:")
                        .strong()
                        .color(WARNING_COLOR),
                );
                ui.add_space(3.0);
                for warning in warnings {
                    ui.horizontal(|ui| {
                        ui.label("⚠");
                        ui.label(egui::RichText::new(warning).color(WARNING_COLOR));
                    });
                }
            }
        });

        ui.add_space(15.0);
    }

    fn render_phase4_intermediate(&self, ui: &mut egui::Ui, code: &[String]) {
        ui.group(|ui| {
            self.render_phase_header(ui, 4, "Intermediate Code Generation");
            ui.label("Generating Three-Address Code (TAC)");
            ui.add_space(8.0);

            ui.label(egui::RichText::new("Three-Address Code:").strong());
            ui.add_space(3.0);

            egui::ScrollArea::vertical()
                .id_salt("tac_scroll")
                .max_height(CODE_SCROLL_HEIGHT)
                .show(ui, |ui| {
                    egui::Frame::NONE
                        .fill(egui::Color32::from_rgb(40, 40, 45))
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            for (i, line) in code.iter().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(format!("{:2}:", i + 1))
                                            .color(egui::Color32::GRAY)
                                            .monospace(),
                                    );
                                    ui.label(
                                        egui::RichText::new(line)
                                            .color(egui::Color32::WHITE)
                                            .monospace(),
                                    );
                                });
                            }
                        });
                });
        });

        ui.add_space(15.0);
    }

    fn render_phase5_optimization(&self, ui: &mut egui::Ui, result: &ParseResult) {
        ui.group(|ui| {
            self.render_phase_header(ui, 5, "Code Optimization");
            ui.label("Constant folding, algebraic simplification, and dead code elimination");
            ui.add_space(8.0);

            // Show optimization comparison
            let original_code_len = result.three_address_code.len();
            let optimized_code_len = result.optimized_three_address_code.len();

            if result.ast.to_string() != result.optimized_ast.to_string()
                || original_code_len != optimized_code_len
            {
                ui.label(
                    egui::RichText::new(format!(
                        "✓ Optimizations applied: {} → {} instructions",
                        original_code_len, optimized_code_len
                    ))
                    .color(SUCCESS_COLOR)
                    .strong(),
                );
                ui.add_space(5.0);
            } else {
                ui.label(
                    egui::RichText::new("No further optimizations possible")
                        .color(egui::Color32::GRAY)
                        .italics(),
                );
                ui.add_space(5.0);
            }

            ui.columns(2, |columns| {
                // Original
                columns[0].group(|ui| {
                    ui.label(egui::RichText::new("Before Optimization:").strong());
                    ui.add_space(3.0);
                    ui.label(
                        egui::RichText::new(format!("AST: {}", result.ast))
                            .monospace()
                            .small(),
                    );
                });

                // Optimized
                columns[1].group(|ui| {
                    ui.label(egui::RichText::new("After Optimization:").strong());
                    ui.add_space(3.0);
                    ui.label(
                        egui::RichText::new(format!("AST: {}", result.optimized_ast))
                            .monospace()
                            .small()
                            .color(SUCCESS_COLOR),
                    );
                });
            });

            ui.add_space(8.0);
            ui.label(egui::RichText::new("Optimized Three-Address Code:").strong());
            ui.add_space(3.0);

            egui::ScrollArea::vertical()
                .id_salt("optimized_tac_scroll")
                .max_height(CODE_SCROLL_HEIGHT)
                .show(ui, |ui| {
                    egui::Frame::NONE
                        .fill(egui::Color32::from_rgb(30, 50, 35))
                        .inner_margin(10.0)
                        .show(ui, |ui| {
                            for (i, line) in result.optimized_three_address_code.iter().enumerate()
                            {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(format!("{:2}:", i + 1))
                                            .color(egui::Color32::GRAY)
                                            .monospace(),
                                    );
                                    ui.label(
                                        egui::RichText::new(line)
                                            .color(egui::Color32::from_rgb(150, 255, 150))
                                            .monospace(),
                                    );
                                });
                            }
                        });
                });
        });

        ui.add_space(15.0);
    }

    fn render_final_result(&self, ui: &mut egui::Ui, ast: &ASTNode) {
        ui.group(|ui| {
            ui.heading("Final Evaluation");
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
        ui.collapsing("ℹ️ Supported Operators & Features", |ui| {
            ui.label("= : Assignment");
            ui.label("+ : Addition");
            ui.label("- : Subtraction");
            ui.label("* : Multiplication");
            ui.label("/ : Division");
            ui.label("^ : Power");
            ui.label("sqrt() : Square Root");
            ui.label("a-z, A-Z : Identifiers");
            ui.label("( ) : Parentheses");
            ui.separator();
            ui.label(egui::RichText::new("Optimization Techniques:").strong());
            ui.label("• Constant folding (e.g., 2+3 → 5)");
            ui.label("• Algebraic simplification (e.g., x*1 → x, x+0 → x)");
            ui.label("• Dead code elimination (e.g., x*0 → 0)");
        });
    }

    fn render_results(&self, ui: &mut egui::Ui, result: &ParseResult) {
        ui.separator();
        ui.add_space(15.0);

        // Phase 1: Lexical Analysis
        self.render_phase1_lexical(ui, result);

        // Phase 2: Syntax Analysis
        self.render_phase2_syntax(ui, &result.ast);

        // Phase 3: Semantic Analysis
        self.render_phase3_semantic(ui, &result.semantic_warnings);

        // Phase 4: Intermediate Code Generation
        self.render_phase4_intermediate(ui, &result.three_address_code);

        // Phase 5: Code Optimization
        self.render_phase5_optimization(ui, result);

        // Final Result
        self.render_final_result(ui, &result.optimized_ast);
    }
}

impl eframe::App for ExpressionParserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("main_scroll")
                .show(ui, |ui| {
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
        });
    }
}
