use rand::Rng;
use rstest::rstest;

use compl::{
    approximating_solution, naive_solution, solve_with_dyn_by_costs, solve_with_dyn_by_weights,
};

pub fn parametrization_of_input_data(
    count_bound: usize,
    weight_bound: usize,
    cost_bound: usize,
    cap_bound: usize,
) -> (Vec<(usize, usize)>, usize) {
    let mut gen = rand::thread_rng();
    let count = gen.gen_range(0..=count_bound);

    let mut objects = Vec::with_capacity(count);
    for _ in 0..count {
        objects.push((
            gen.gen_range(1..=weight_bound),
            gen.gen_range(0..=cost_bound),
        ));
    }

    (objects, gen.gen_range(0..=cap_bound))
}

fn small_input_data() -> (Vec<(usize, usize)>, usize) {
    parametrization_of_input_data(20, 50, 50, 30)
}

fn normal_input_data() -> (Vec<(usize, usize)>, usize) {
    parametrization_of_input_data(1000, 500, 500, 1000)
}

#[rstest]
#[case(naive_solution, solve_with_dyn_by_costs, small_input_data, 10)]
#[case(naive_solution, solve_with_dyn_by_weights, small_input_data, 10)]
#[case(
    solve_with_dyn_by_costs,
    solve_with_dyn_by_weights,
    small_input_data,
    1000
)]
#[case(
    solve_with_dyn_by_costs,
    solve_with_dyn_by_weights,
    normal_input_data,
    10
)]
fn compare_two(
    #[case] f_1: impl Fn(Vec<(usize, usize)>, usize) -> usize,
    #[case] f_2: impl Fn(Vec<(usize, usize)>, usize) -> usize,
    #[case] input_fn: impl Fn() -> (Vec<(usize, usize)>, usize),
    #[case] count: usize,
) {
    for _ in 0..count {
        let (obj, cap) = input_fn();
        assert_eq!(f_1(obj.clone(), cap), f_2(obj, cap))
    }
}

#[rstest]
#[case(solve_with_dyn_by_costs, 0.1, small_input_data, 100)]
#[case(solve_with_dyn_by_costs, 0.01, small_input_data, 10)]
#[case(solve_with_dyn_by_costs, 0.001, small_input_data, 10)]
#[case(solve_with_dyn_by_weights, 0.1, small_input_data, 100)]
#[case(solve_with_dyn_by_weights, 0.01, small_input_data, 10)]
#[case(solve_with_dyn_by_weights, 0.001, small_input_data, 10)]
fn compare_approximation(
    #[case] verify: impl Fn(Vec<(usize, usize)>, usize) -> usize,
    #[case] epsilon: f64,
    #[case] input_fn: impl Fn() -> (Vec<(usize, usize)>, usize),
    #[case] count: usize,
) {
    for _ in 0..count {
        let (obj, cap) = input_fn();
        let answer = verify(obj.clone(), cap);

        let approx = approximating_solution(epsilon, obj, cap);

        // Check (1 - eps) approximation
        let diff = answer.checked_sub(approx).unwrap(); // approx < answer, so it is correct
        assert!(diff as f64 <= epsilon * answer as f64)
    }
}

fn big_capacity_input_data() -> (Vec<(usize, usize)>, usize) {
    parametrization_of_input_data(100, 5000, 5000, 10000)
}

#[rstest]
#[case(solve_with_dyn_by_costs, 0.1, big_capacity_input_data, 10)]
#[case(solve_with_dyn_by_costs, 0.01, big_capacity_input_data, 5)]
#[case(solve_with_dyn_by_costs, 0.001, big_capacity_input_data, 1)]
#[case(solve_with_dyn_by_weights, 0.1, big_capacity_input_data, 10)]
#[case(solve_with_dyn_by_weights, 0.01, big_capacity_input_data, 5)]
#[case(solve_with_dyn_by_weights, 0.001, big_capacity_input_data, 1)]
fn big_capacity_test(
    #[case] verify: impl Fn(Vec<(usize, usize)>, usize) -> usize,
    #[case] epsilon: f64,
    #[case] input_fn: impl Fn() -> (Vec<(usize, usize)>, usize),
    #[case] count: usize,
) {
    for _ in 0..count {
        let (obj, cap) = input_fn();
        let answer = verify(obj.clone(), cap);

        let approx = approximating_solution(epsilon, obj, cap);

        // Check (1 - eps) approximation
        let diff = answer.checked_sub(approx).unwrap(); // approx < answer, so it is correct
        assert!(diff as f64 <= epsilon * answer as f64)
    }
}
