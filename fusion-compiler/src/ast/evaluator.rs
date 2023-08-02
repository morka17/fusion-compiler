// Import the ASTVisitor trait to define ASTEvaluator as a visitor for the AST nodes.
use super::ASTVisitor;

// Define the ASTEvaluator struct to evaluate the AST nodes.
pub struct ASTEvaluator {
    pub(crate) last_value: Option<i64>,
}

impl ASTEvaluator {
    // Create a new ASTEvaluator instance with the last_value set to None.
    pub fn new() -> Self {
        Self { last_value: None }
    }
}

// Implement the ASTVisitor trait for the ASTEvaluator struct.
impl ASTVisitor for ASTEvaluator {
    // Implement the visit_number method to handle visiting a number node in the AST.
    fn visit_number(&mut self, number: &super::ASTNumberExpression) {
        self.last_value = Some(number.number) // Set last_value to the value of the number node.
    }

    // Implement the visit_binary_expression method to handle visiting a binary expression node in the AST.
    fn visit_binary_expression(&mut self, expr: &super::ASTBinaryExpression) {
        self.visit_expression(&expr.left); // Recursively visit the left-hand side of the binary expression.
        let left = self.last_value.unwrap(); // Get the value of the left-hand side expression.

        self.visit_expression(&expr.right); // Recursively visit the right-hand side of the binary expression.
        let right = self.last_value.unwrap(); // Get the value of the right-hand side expression.

        // Evaluate the binary expression based on the operator and update last_value with the result.
        self.last_value = Some(match expr.operator.kind {
            super::ASTBinaryOperatorKind::Plus => left + right,
            super::ASTBinaryOperatorKind::Minus => left - right,
            super::ASTBinaryOperatorKind::Multiply => left * right,
            super::ASTBinaryOperatorKind::Divide => left / right,
        });
    }
}
