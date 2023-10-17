// https://leetcode.com/problems/validate-binary-tree-nodes

impl Solution {
    pub fn validate_binary_tree_nodes(
        n: i32,
        left_child: Vec<i32>,
        right_child: Vec<i32>,
    ) -> bool {
        let mut root_candidate = vec![true; n as usize];

        for &x in left_child.iter().filter(|&&x| x >= 0) {
            root_candidate[x as usize] = false;
        }

        for &x in right_child.iter().filter(|&&x| x >= 0) {
            root_candidate[x as usize] = false;
        }

        let candidates: Vec<_> = root_candidate
            .into_iter()
            .enumerate()
            .filter(|(_, x)| *x)
            .map(|(index, _)| index)
            .collect();

        if candidates.len() != 1 {
            return false;
        }

        let mut queue = candidates;
        let mut visited = vec![false; n as usize];
        visited[queue[0]] = true;

        let mut current = 0;

        while current < queue.len() {
            let x = queue[current];
            current += 1;
            if left_child[x] != -1 {
                let i = left_child[x] as usize;
                if visited[i] {
                    return false;
                }
                queue.push(i);
                visited[i] = true;
            }
            if right_child[x] != -1 {
                let i = right_child[x] as usize;
                if visited[i] {
                    return false;
                }
                queue.push(i);
                visited[i] = true;
            }
        }

        return queue.len() == n as usize;
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result =
            Solution::validate_binary_tree_nodes(4, vec![1, -1, 3, -1], vec![2, -1, -1, -1]);
        assert_eq!(result, true);

        let result =
            Solution::validate_binary_tree_nodes(4, vec![3, -1, 1, -1], vec![-1, -1, 0, -1]);
        assert_eq!(result, true);
    }
}
