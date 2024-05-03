#![feature(test)]

extern crate test;

use test::Bencher;

use compl::{
    approximating_solution, naive_solution, solve_with_dyn_by_costs, solve_with_dyn_by_weights,
};

fn bench_count_input_data(count: usize) -> (Vec<(usize, usize)>, usize) {
    (vec![(10, 10); count], 100)
}

#[cfg(test)]
mod by_count {
    use super::*;
    #[bench]
    fn bench_naive_solution_1(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(5);
        b.iter(|| naive_solution(objects.clone(), cap));
    }

    #[bench]
    fn bench_naive_solution_2(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(10);
        b.iter(|| naive_solution(objects.clone(), cap));
    }

    #[bench]
    fn bench_naive_solution_3(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(20);
        b.iter(|| naive_solution(objects.clone(), cap));
    }

    #[bench]
    fn bench_solve_with_dyn_by_costs_1(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(5);
        b.iter(|| solve_with_dyn_by_costs(objects.clone(), cap));
    }

    #[bench]
    fn bench_solve_with_dyn_by_costs_2(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(10);
        b.iter(|| solve_with_dyn_by_costs(objects.clone(), cap));
    }

    #[bench]
    fn bench_solve_with_dyn_by_costs_3(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(20);
        b.iter(|| solve_with_dyn_by_costs(objects.clone(), cap));
    }

    #[bench]
    fn bench_solve_with_dyn_by_costs_4(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(40);
        b.iter(|| solve_with_dyn_by_costs(objects.clone(), cap));
    }

    #[bench]
    fn bench_solve_with_dyn_by_weights_1(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(5);
        b.iter(|| solve_with_dyn_by_weights(objects.clone(), cap));
    }

    #[bench]
    fn bench_solve_with_dyn_by_weights_2(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(10);
        b.iter(|| solve_with_dyn_by_weights(objects.clone(), cap));
    }

    #[bench]
    fn bench_solve_with_dyn_by_weights_3(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(20);
        b.iter(|| solve_with_dyn_by_weights(objects.clone(), cap));
    }

    #[bench]
    fn bench_solve_with_dyn_by_weights_4(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(40);
        b.iter(|| solve_with_dyn_by_weights(objects.clone(), cap));
    }

    #[bench]
    fn bench_approximating_solution_1(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(5);
        b.iter(|| approximating_solution(0.1, objects.clone(), cap));
    }

    #[bench]
    fn bench_approximating_solution_2(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(10);
        b.iter(|| approximating_solution(0.1, objects.clone(), cap));
    }

    #[bench]
    fn bench_approximating_solution_3(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(20);
        b.iter(|| approximating_solution(0.1, objects.clone(), cap));
    }

    #[bench]
    fn bench_approximating_solution_4(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(40);
        b.iter(|| approximating_solution(0.1, objects.clone(), cap));
    }
}

#[cfg(test)]
mod by_epsilon {
    use super::*;
    #[bench]
    fn bench_epsilon_1(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(20);
        b.iter(|| approximating_solution(0.001, objects.clone(), cap));
    }

    #[bench]
    fn bench_epsilon_2(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(20);
        b.iter(|| approximating_solution(0.005, objects.clone(), cap));
    }

    #[bench]
    fn bench_epsilon_3(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(20);
        b.iter(|| approximating_solution(0.01, objects.clone(), cap));
    }

    #[bench]
    fn bench_epsilon_4(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(20);
        b.iter(|| approximating_solution(0.05, objects.clone(), cap));
    }

    #[bench]
    fn bench_epsilon_5(b: &mut Bencher) {
        let (objects, cap) = bench_count_input_data(20);
        b.iter(|| approximating_solution(0.1, objects.clone(), cap));
    }
}
