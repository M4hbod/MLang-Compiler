use crate::ast::ASTNode;
use eframe::egui;

// Tree layout constants
const NODE_WIDTH: f32 = 120.0;
const NODE_HEIGHT: f32 = 40.0;
const LEVEL_SPACING: f32 = 80.0;
const SIBLING_SPACING: f32 = 20.0;

// Colors
const OPERATOR_COLOR: egui::Color32 = egui::Color32::from_rgb(220, 50, 50);
const NUMBER_COLOR: egui::Color32 = egui::Color32::from_rgb(50, 150, 220);
const VARIABLE_COLOR: egui::Color32 = egui::Color32::from_rgb(150, 100, 200);
const FUNCTION_COLOR: egui::Color32 = egui::Color32::from_rgb(220, 140, 50);
const LINE_COLOR: egui::Color32 = egui::Color32::from_rgb(100, 100, 100);

#[derive(Clone)]
struct TreeNode {
    pos: egui::Pos2,
    size: egui::Vec2,
    label: String,
    color: egui::Color32,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn from_ast(ast: &ASTNode) -> Self {
        match ast {
            ASTNode::Number(n) => TreeNode {
                pos: egui::Pos2::ZERO,
                size: egui::vec2(NODE_WIDTH, NODE_HEIGHT),
                label: format!("{}", n),
                color: NUMBER_COLOR,
                children: vec![],
            },
            ASTNode::Identifier(name, _) => TreeNode {
                pos: egui::Pos2::ZERO,
                size: egui::vec2(NODE_WIDTH, NODE_HEIGHT),
                label: name.clone(),
                color: VARIABLE_COLOR,
                children: vec![],
            },
            ASTNode::BinaryOp { op, left, right } => TreeNode {
                pos: egui::Pos2::ZERO,
                size: egui::vec2(NODE_WIDTH, NODE_HEIGHT),
                label: op.to_string(),
                color: OPERATOR_COLOR,
                children: vec![TreeNode::from_ast(left), TreeNode::from_ast(right)],
            },
            ASTNode::UnaryOp { op, operand } => TreeNode {
                pos: egui::Pos2::ZERO,
                size: egui::vec2(NODE_WIDTH, NODE_HEIGHT),
                label: op.clone(),
                color: FUNCTION_COLOR,
                children: vec![TreeNode::from_ast(operand)],
            },
        }
    }

    fn calculate_width(&self) -> f32 {
        if self.children.is_empty() {
            self.size.x
        } else {
            let children_width: f32 = self
                .children
                .iter()
                .map(|c| c.calculate_width())
                .sum::<f32>()
                + (self.children.len() - 1) as f32 * SIBLING_SPACING;
            children_width.max(self.size.x)
        }
    }

    fn layout(&mut self, x: f32, y: f32) {
        self.pos = egui::pos2(x, y);

        if !self.children.is_empty() {
            let total_width = self.calculate_width();
            let mut current_x = x - total_width / 2.0;

            for child in &mut self.children {
                let child_width = child.calculate_width();
                child.layout(current_x + child_width / 2.0, y + LEVEL_SPACING);
                current_x += child_width + SIBLING_SPACING;
            }
        }
    }

    fn get_bounds(&self) -> egui::Rect {
        let mut min = self.pos - self.size / 2.0;
        let mut max = self.pos + self.size / 2.0;

        for child in &self.children {
            let child_bounds = child.get_bounds();
            min = min.min(child_bounds.min);
            max = max.max(child_bounds.max);
        }

        egui::Rect::from_min_max(min, max)
    }
}

pub fn render_tree(ui: &mut egui::Ui, ast: &ASTNode, max_height: f32) {
    let mut tree = TreeNode::from_ast(ast);

    // Calculate the tree width
    let tree_width = tree.calculate_width();

    // Layout the tree starting from origin
    tree.layout(tree_width / 2.0, NODE_HEIGHT / 2.0);

    // Get bounds
    let bounds = tree.get_bounds();

    // Add generous padding
    let padding = 100.0;
    let total_width = bounds.width() + padding * 2.0;
    let total_height = bounds.height() + padding * 2.0;

    // Calculate offset to center the tree
    let offset_x = padding - bounds.min.x;
    let offset_y = padding - bounds.min.y;

    // Use a Frame to contain the scroll area
    egui::Frame::default()
        .fill(egui::Color32::from_rgb(30, 30, 35))
        .show(ui, |ui| {
            egui::ScrollArea::both()
                .id_salt("ast_tree_scroll")
                .max_height(max_height)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    // Create a child UI with the exact size we need
                    let desired_size = egui::vec2(total_width, total_height);

                    let (rect, response) =
                        ui.allocate_exact_size(desired_size, egui::Sense::hover());

                    if ui.is_rect_visible(rect) {
                        let painter = ui.painter();

                        // Draw background
                        painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(30, 30, 35));

                        // Draw the tree with offset
                        draw_tree_with_offset(
                            &tree,
                            painter,
                            rect.min.x + offset_x,
                            rect.min.y + offset_y,
                        );
                    }

                    response
                });
        });
}

fn draw_tree_with_offset(node: &TreeNode, painter: &egui::Painter, offset_x: f32, offset_y: f32) {
    // Draw lines to children
    let node_center = egui::pos2(
        node.pos.x + offset_x,
        node.pos.y + offset_y + node.size.y / 2.0,
    );

    for child in &node.children {
        let child_top = egui::pos2(
            child.pos.x + offset_x,
            child.pos.y + offset_y - child.size.y / 2.0,
        );
        painter.line_segment([node_center, child_top], egui::Stroke::new(3.0, LINE_COLOR));
    }

    // Draw node box
    let node_rect = egui::Rect::from_center_size(
        egui::pos2(node.pos.x + offset_x, node.pos.y + offset_y),
        node.size,
    );

    painter.rect_filled(node_rect, 5.0, node.color);

    // Draw outline with 4 lines
    let stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);
    painter.line_segment([node_rect.left_top(), node_rect.right_top()], stroke);
    painter.line_segment([node_rect.right_top(), node_rect.right_bottom()], stroke);
    painter.line_segment([node_rect.right_bottom(), node_rect.left_bottom()], stroke);
    painter.line_segment([node_rect.left_bottom(), node_rect.left_top()], stroke);

    // Draw label
    painter.text(
        node_rect.center(),
        egui::Align2::CENTER_CENTER,
        &node.label,
        egui::FontId::proportional(16.0),
        egui::Color32::WHITE,
    );

    // Draw children recursively
    for child in &node.children {
        draw_tree_with_offset(child, painter, offset_x, offset_y);
    }
}
