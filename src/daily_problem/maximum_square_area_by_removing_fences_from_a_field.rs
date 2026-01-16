use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn maximum_square_area_by_removing_fences_from_a_field(
        m: i32,
        n: i32,
        h_fences: Vec<i32>,
        v_fences: Vec<i32>,
    ) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let get_edges = |fences: &Vec<i32>, limit: i32| -> HashSet<i32> {
            let mut sorted_fences = vec![1];
            sorted_fences.extend_from_slice(fences);
            sorted_fences.push(limit);
            sorted_fences.sort_unstable();

            (0..sorted_fences.len())
                .flat_map(|i| {
                    let pi = sorted_fences[i];
                    sorted_fences[i + 1..].iter().map(move |&pj| pj - pi)
                })
                .collect()
        };

        let h_edges = get_edges(&h_fences, m);
        let v_edges = get_edges(&v_fences, n);

        println!("{:?}", h_edges);
        println!("{:?}", v_edges);

        println!("{:?}", h_edges.union(&v_edges));

        h_edges
            .intersection(&v_edges)
            .max()
            .map_or(-1, |&e| ((e as i64).pow(2) % MOD) as i32)
    }
}

fn main() {
    let m = 5;
    let n = 4;
    let h_fences = vec![2, 3];
    let v_fences = vec![2];

    let result =
        Solution::maximum_square_area_by_removing_fences_from_a_field(m, n, h_fences, v_fences);
    println!("Maximum square area: {}", result);
}
