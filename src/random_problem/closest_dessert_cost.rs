struct Solution;

impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        fn choose_toppings(
            topping_costs: &Vec<i32>,
            index: usize,
            current_cost: i32,
            possible_costs: &mut Vec<bool>,
        ) {
            possible_costs[current_cost as usize] = true;

            if index == topping_costs.len() {
                return;
            }

            // Choose 0 of this topping
            choose_toppings(topping_costs, index + 1, current_cost, possible_costs);
            // Choose 1 of this topping
            choose_toppings(
                topping_costs,
                index + 1,
                current_cost + topping_costs[index],
                possible_costs,
            );
            // Choose 2 of this topping
            choose_toppings(
                topping_costs,
                index + 1,
                current_cost + 2 * topping_costs[index],
                possible_costs,
            );
        }

        let mut possible_costs = vec![false; 200005];
        for &base in &base_costs {
            possible_costs[base as usize] = true;
        }

        if possible_costs[target as usize] {
            return target;
        }

        for base in base_costs.iter() {
            choose_toppings(&topping_costs, 0, *base, &mut possible_costs);
        }

        println!("{:?}", possible_costs);

        let (mut left, mut right) = (target as usize, target as usize);
        let (mut left_found, mut right_found) = (false, false);
        while left > 0 && !possible_costs[left] {
            left -= 1;
        }

        while right < 200004 && !possible_costs[right] {
            right += 1;
        }
        let left_found = possible_costs[left];
        let right_found = possible_costs[right];
        println!("left: {}, right: {}", left, right);

        if left_found && right_found {
            if target - left as i32 <= right as i32 - target {
                left as i32
            } else {
                right as i32
            }
        } else if left_found {
            left as i32
        } else {
            right as i32
        }
    }
}

fn main() {
    let base_costs = vec![3, 10];
    let topping_costs = vec![2, 5];
    let target = 9;
    let result = Solution::closest_cost(base_costs, topping_costs, target);
    println!("Result: {}", result);
}
