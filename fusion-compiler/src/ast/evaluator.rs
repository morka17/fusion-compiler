use super::ASTVisitor;

pub struct ASTEvaluator{
    pub(crate) last_value: Option<i64>,
}

impl ASTEvaluator {
    pub fn new() -> Self {
        Self{last_value: None}
    }
}

impl ASTVisitor for ASTEvaluator {
    fn visit_number(&mut self, number: &super::ASTNumberExpression) {
        self.last_value = Some(number.number)
    }

    fn visit_binary_expression(&mut self, expr: &super::ASTBinaryExpression) {
        self.visit_expression(&expr.left);
        let left = self.last_value.unwrap();
        self.visit_expression(&expr.right);
        let right = self.last_value.unwrap();
        self.last_value = Some(match expr.operator.kind {
            super::ASTBinaryOperatorKind::Plus => left + right,
            super::ASTBinaryOperatorKind::Minus => left - right,
            super::ASTBinaryOperatorKind::Multiply => left * right,
            super::ASTBinaryOperatorKind::Divide => left / right,
        })

    }
}