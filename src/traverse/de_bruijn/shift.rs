use crate::expression::variable::DeBruijnIndex;
use crate::expression::Expression;

pub struct DeBruijnShift {
    place: i64,
}

impl DeBruijnShift {
    pub fn shift(place: i64, term: &mut Expression) {
        let mut shifter = Self { place };
        shifter.traverse(1, term)
    }

    fn traverse(&mut self, cutoff: DeBruijnIndex, expression: &mut Expression) {
        match expression {
            Expression::Variable(variable) => {
                if variable.index >= cutoff {
                    variable.index = variable.index.saturating_add_signed(self.place);
                }
            }
            Expression::Abstraction(abstraction) => {
                self.traverse(cutoff + 1, &mut abstraction.body);
            }
            Expression::Application(application) => {
                self.traverse(cutoff, &mut application.applicator);
                self.traverse(cutoff, &mut application.argument);
            }
        }
    }
}
