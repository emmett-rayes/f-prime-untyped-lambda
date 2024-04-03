use crate::expression::variable::Variable;
use crate::untyped_lambda::term::UntypedTerm;

pub fn replace_term(dst: &mut UntypedTerm, f: impl FnOnce(UntypedTerm) -> UntypedTerm) {
    let dummy_term = UntypedTerm::from(Variable::new(""));
    let term = std::mem::replace(dst, dummy_term);
    let _ = std::mem::replace(dst, f(term));
}

pub fn try_replace_term(
    dst: &mut UntypedTerm,
    f: impl FnOnce(UntypedTerm) -> Result<UntypedTerm, UntypedTerm>,
) -> Result<(), UntypedTerm> {
    let dummy_term = UntypedTerm::from(Variable::new(""));
    let term = std::mem::replace(dst, dummy_term);
    let _ = std::mem::replace(dst, f(term)?);
    Ok(())
}
