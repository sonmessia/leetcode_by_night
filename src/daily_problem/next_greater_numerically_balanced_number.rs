struct Solution;

impl Solution {
    pub fn next_beautiful_number(mut n: i32) -> i32 {
        // fn binary_search(arr: &Vec<i32>, target: i32) -> i32 {
        //     let mut left = 0;
        //     let mut right = arr.len() - 1;
        //     while left < right {
        //         let mid = left + (right - left) / 2;
        //         if arr[mid] <= target {
        //             left = mid + 1;
        //         } else {
        //             right = mid;
        //         }
        //     }
        //     arr[left]
        // }
        //
        // let mut arr = [
        //     1, 22, 122, 212, 221, 333, 1333, 3133, 3313, 3331, 4444, 14444, 22333, 23233, 23323,
        //     23332, 32233, 32323, 32332, 33223, 33232, 33322, 41444, 44144, 44414, 44441, 55555,
        //     122333, 123233, 123323, 123332, 132233, 132323, 132332, 133223, 133232, 133322, 155555,
        //     212333, 213233, 213323, 213332, 221333, 223133, 223313, 223331, 224444, 231233, 231323,
        //     231332, 232133, 232313, 232331, 233123, 233132, 233213, 233231, 233312, 233321, 242444,
        //     244244, 244424, 244442, 312233, 312323, 312332, 313223, 313232, 313322, 321233, 321323,
        //     321332, 322133, 322313, 322331, 323123, 323132, 323213, 323231, 323312, 323321, 331223,
        //     331232, 331322, 332123, 332132, 332213, 332231, 332312, 332321, 333122, 333212, 333221,
        //     422444, 424244, 424424, 424442, 442244, 442424, 442442, 444224, 444242, 444422, 515555,
        //     551555, 555155, 555515, 555551, 666666, 1224444,
        // ];
        //
        // arr.sort_unstable();
        // binary_search(&arr.to_vec(), n)

        fn is_numerically_balanced(mut num: i32) -> bool {
            let mut count = [0; 10];
            let mut digits = Vec::new();
            while num > 0 {
                let digit = (num % 10) as usize;
                count[digit] += 1;
                digits.push(digit);
                num /= 10;
            }
            for &d in &digits {
                if count[d] != d as i32 {
                    return false;
                }
            }
            true
        }

        n += 1;
        loop {
            if is_numerically_balanced(n) {
                return n;
            }
            n += 1;
        }
    }
}

fn main() {
    let result = Solution::next_beautiful_number(1);
    println!("Next beautiful number: {}", result);
}
