struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let path_iter = path.split('/').collect::<Vec<&str>>();

        println!("path_iter: {:?}", path_iter);

        let mut ans = vec!["/"];

        for &stri in path_iter.iter() {
            if stri == "" {
                continue;
            }
            if stri == ".." && !stri.is_empty() {
                ans.pop();
                ans.pop();
                ans.pop();
                continue;
            } else if stri != "." && stri != ".." {
                ans.push(stri);
            }

            ans.push("/");
            println!("ans: {:?}", ans);
        }

        ans.into_iter().collect()
    }
}
