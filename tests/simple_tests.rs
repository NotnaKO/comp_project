use rstest::rstest;

use compl::{approximating_with_substitute, solve_with_dyn_by_costs, solve_with_dyn_by_weights};

#[rstest]
#[case(solve_with_dyn_by_weights)]
#[case(solve_with_dyn_by_costs)]
#[case(approximating_with_substitute(0.001))]
#[case(approximating_with_substitute(0.01))]
#[case(approximating_with_substitute(0.1))]
fn it_works_1(#[case] func: impl Fn(Vec<(usize, usize)>, usize) -> usize) {
    let data = vec![(1, 1), (2, 2), (3, 4), (4, 3)];
    assert_eq!(func(data.clone(), 0), 0);
    assert_eq!(func(data.clone(), 1), 1);
    assert_eq!(func(data.clone(), 2), 2);
    assert_eq!(func(data.clone(), 3), 4);
    assert_eq!(func(data.clone(), 4), 5);
    assert_eq!(func(data.clone(), 5), 6);
}

#[rstest]
#[case(solve_with_dyn_by_weights)]
#[case(solve_with_dyn_by_costs)]
#[case(approximating_with_substitute(0.001))]
#[case(approximating_with_substitute(0.01))]
#[case(approximating_with_substitute(0.1))]
fn is_works_2(#[case] func: impl Fn(Vec<(usize, usize)>, usize) -> usize) {
    let data = vec![(4, 1), (2, 3), (2, 3), (3, 4)];
    assert_eq!(func(data, 4), 6);
}

#[rstest]
#[case(solve_with_dyn_by_weights)]
#[case(solve_with_dyn_by_costs)]
#[case(approximating_with_substitute(0.001))]
#[case(approximating_with_substitute(0.01))]
#[case(approximating_with_substitute(0.1))]
fn is_works_3(#[case] func: impl Fn(Vec<(usize, usize)>, usize) -> usize) {
    let data = vec![(1, 1)];
    assert_eq!(func(data, 1000), 1);
}

#[rstest]
#[case(solve_with_dyn_by_weights)]
#[case(solve_with_dyn_by_costs)]
#[case(approximating_with_substitute(0.001))]
#[case(approximating_with_substitute(0.01))]
#[case(approximating_with_substitute(0.1))]
#[case(approximating_with_substitute(1000.0))]
fn empty_data(#[case] func: impl Fn(Vec<(usize, usize)>, usize) -> usize) {
    let data = vec![];
    assert_eq!(func(data, 1000), 0);
}

#[rstest]
#[case(solve_with_dyn_by_weights)]
#[case(solve_with_dyn_by_costs)]
#[case(approximating_with_substitute(0.001))]
#[case(approximating_with_substitute(0.01))]
#[case(approximating_with_substitute(0.1))]
#[case(approximating_with_substitute(1000.0))]
fn zero_capacity(#[case] func: impl Fn(Vec<(usize, usize)>, usize) -> usize) {
    let data = vec![(1, 1000)];
    assert_eq!(func(data, 0), 0);
}
