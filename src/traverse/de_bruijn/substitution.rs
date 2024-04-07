use crate::expression::variable::DeBruijnIndex;
use crate::expression::Expression;
use crate::traverse::de_bruijn::shift::DeBruijnShift;

pub struct DeBruijnSubstitution {
    replacement: Expression,
}

impl DeBruijnSubstitution {
    pub fn substitute(target: DeBruijnIndex, replacement: Expression, expression: &mut Expression) {
        let mut substitution = DeBruijnSubstitution { replacement };
        substitution.traverse(target, expression);
    }

    fn traverse(&mut self, target: DeBruijnIndex, expression: &mut Expression) {
        match expression {
            Expression::Variable(variable) => {
                if variable.index == target {
                    *expression = self.replacement.clone()
                }
            }
            Expression::Abstraction(abstraction) => {
                let replacement = self.replacement.clone();
                DeBruijnShift::shift(1, &mut self.replacement);
                self.traverse(target + 1, &mut abstraction.body);
                self.replacement = replacement;
            }
            Expression::Application(application) => {
                self.traverse(target, &mut application.applicator);
                self.traverse(target, &mut application.argument);
            }
        }
    }
}
