use crate::common::process_untyped;

/// courtesy of https://github.com/ljedrz/lambda_calculus/blob/master/src/combinators.rs

#[test]
fn identity() {
    assert_eq!(process_untyped("λx. x"), "λ 1");
}

#[test]
fn constant() {
    assert_eq!(process_untyped("λx y. x"), "λ λ 2");
}

#[test]
fn substitution() {
    assert_eq!(process_untyped("λx y z. x z (y z)"), "λ λ λ 3 1 (2 1)");
}

#[test]
fn iota() {
    assert_eq!(process_untyped("λx. x S K"), "λ 1 S K");
}

#[test]
fn composition() {
    assert_eq!(process_untyped("λx y z. x (y z)"), "λ λ λ 3 (2 1)");
}

#[test]
fn swapping() {
    assert_eq!(process_untyped("λ x y z. x z y"), "λ λ λ 3 1 2")
}

#[test]
fn duplicating() {
    assert_eq!(process_untyped(" λx y. x y y"), "λ λ 2 1 1")
}

#[test]
fn self_application() {
    assert_eq!(process_untyped("λx. x x"), "λ 1 1")
}

#[test]
fn divergent() {
    assert_eq!(process_untyped("(λx. x x) (λx. x x)"), "(λ 1 1) (λ 1 1)")
}

#[test]
fn lazy_fixed_point() {
    assert_eq!(
        process_untyped("λf. (λx. f (x x)) (λx. f (x x))"),
        "λ (λ 2 (1 1)) (λ 2 (1 1))",
    )
}

#[test]
fn strict_fixed_point() {
    assert_eq!(
        process_untyped("λf. (λx. f (λv. x x v)) (λx. f (λv. x x v))"),
        "λ (λ 2 (λ 2 2 1)) (λ 2 (λ 2 2 1))",
    )
}

#[test]
fn reverse_application() {
    assert_eq!(process_untyped("λx f. f x"), "λ λ 1 2")
}

#[test]
fn turing_fixed_point() {
    assert_eq!(
        process_untyped("(λx y. y (x x y)) (λx y. y (x x y))"),
        "(λ λ 1 (2 2 1)) (λ λ 1 (2 2 1))",
    )
}
