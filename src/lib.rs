pub fn solve_with_dyn_by_weights(objects: Vec<(usize, usize)>, knapsack_capacity: usize) -> usize {
    let mut old_layer = vec![0; knapsack_capacity + 1];
    for (cur_weight, cur_cost) in objects.into_iter() {
        let mut new_layer = old_layer.clone();
        for j in 0..=knapsack_capacity {
            _ = j
                .checked_sub(cur_weight)
                .map(|k| new_layer[j] = old_layer[j].max(old_layer[k] + cur_cost));
        }
        old_layer = new_layer;
    }
    old_layer.last().copied().unwrap_or_default()
}

pub fn solve_with_dyn_by_costs(objects: Vec<(usize, usize)>, knapsack_capacity: usize) -> usize {
    // filter to optimize all_costs below
    let objects: Vec<_> = objects
        .into_iter()
        .filter(|(w, _)| *w <= knapsack_capacity)
        .collect();

    let all_costs = objects.iter().map(|x| x.1).sum();
    const INF: usize = usize::MAX;
    assert!(knapsack_capacity < INF, "Too big weights");
    let mut old_layer = vec![INF; all_costs + 1];
    old_layer[0] = 0;

    let mut answer = 0;

    for (cur_weight, cur_cost) in objects {
        let mut new_layer = old_layer.clone();
        for j in 0..=all_costs {
            _ = j.checked_sub(cur_cost).map(|k| {
                // To avoid overflow use saturating_add:
                // it is correct because then we will filter this values with
                // predicate w <= knapsack_capacity
                if old_layer[k] < INF {
                    new_layer[j] = old_layer[j].min(old_layer[k].saturating_add(cur_weight))
                }
            });
        }
        _ = new_layer
            .iter()
            .enumerate()
            .rfind(|(_, &w)| w <= knapsack_capacity)
            .map(|(i, _)| answer = i.max(answer));
        old_layer = new_layer;
    }

    answer
}

pub fn approximating_solution(
    epsilon: f64,
    objects: Vec<(usize, usize)>,
    knapsack_capacity: usize,
) -> usize {
    let objects: Vec<_> = objects
        .into_iter()
        .filter(|(w, _)| *w <= knapsack_capacity)
        .collect();
    if objects.is_empty() {
        return 0;
    }

    let delta =
        epsilon * objects.iter().map(|(_, c)| *c).max().unwrap() as f64 / objects.len() as f64;
    let transformed_objects = objects
        .into_iter()
        .map(|(w, c)| (w, (c as f64 / delta).floor() as usize))
        .collect();
    let answer_after_transform = solve_with_dyn_by_costs(transformed_objects, knapsack_capacity);
    (answer_after_transform as f64 * delta).floor() as usize
}

pub fn approximating_with_substitute(epsilon: f64) -> impl Fn(Vec<(usize, usize)>, usize) -> usize {
    move |objects: Vec<(usize, usize)>, cap: usize| approximating_solution(epsilon, objects, cap)
}
