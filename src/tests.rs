use crate::solver::Builder;
use crate::solver::Solver;

#[test]
fn solve_square() {
    let solver = Solver::default();
    let expected = 0.0;
    let result = solver.solve(1.0, |x: f64| x.powi(2)).unwrap();

    println!("output: {:#?}", result);

    assert!((result.x - expected).abs() < 1.0_e-6);
}

#[test]
fn solve_square_lower_tol() {
    let solver = Builder::default().tol(1.0_e-9).build();
    let expected = 0.0;
    let result = solver.solve(1.0, |x: f64| x.powi(2)).unwrap();

    println!("output: {:#?}", result);

    assert!((result.x - expected).abs() < 1.0_e-8);
}

#[test]
fn solve_cos_x_minus_x() {
    let solver = Solver::default();
    let expected = 0.739085;
    let result = solver.solve(1.0, |x: f64| x.cos() - x).unwrap();

    println!("output: {:#?}", result);

    assert!((result.x - expected).abs() < 1.0_e-6);
}
