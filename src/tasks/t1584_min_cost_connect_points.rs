// https://leetcode.com/problems/min-cost-to-connect-all-points/

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 1 {
            return 0;
        }

        let mut edges = Vec::with_capacity(points.len().pow(2));

        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let len = (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
                edges.push((len, (i << 16 | j) as u32))
            }
        }

        edges.sort_unstable();
        let mut islands: Vec<usize> = (0..points.len()).collect();
        let mut ans = 0;
        let mut added_total = 0;

        for (len, index) in edges {
            let i = (index >> 16) as usize;
            let j = (index & 0xFFFF) as usize;
            let island1 = Self::find_island_number(&mut islands, i);
            let island2 = Self::find_island_number(&mut islands, j);
            if island1 != island2 {
                ans += len;
                islands[island2] = island1;
                added_total = added_total + 1;
                if added_total == points.len() - 1 {
                    return ans;
                }
                Self::optimize_path(&mut islands, i, island1);
                Self::optimize_path(&mut islands, j, island1);
            } else {
                Self::optimize_path(&mut islands, i, island1);
                Self::optimize_path(&mut islands, j, island2);
            }
        }

        unreachable!()
    }

    fn find_island_number(islands: &mut Vec<usize>, mut i: usize) -> usize {
        while islands[i] != i {
            i = islands[i];
        }

        return i;
    }

    fn optimize_path(islands: &mut Vec<usize>, mut i: usize, target: usize) {
        while islands[i] != target {
            let tmp = islands[i];
            islands[i] = target;
            i = tmp
        }
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = Solution::min_cost_connect_points(vec![
            vec![0, 0],
            vec![2, 2],
            vec![3, 10],
            vec![5, 2],
            vec![7, 0],
        ]);

        assert_eq!(result, 20);

        let result = Solution::min_cost_connect_points(vec![vec![3, 12], vec![-2, 5], vec![-4, 1]]);

        assert_eq!(result, 18);
    }
}
