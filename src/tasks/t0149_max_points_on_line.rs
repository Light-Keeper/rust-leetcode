// https://leetcode.com/problems/max-points-on-a-line/description/

use std::collections::{HashMap};
use std::mem;

#[derive(Hash, Clone, Copy, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Hash, Clone, Copy, Eq, PartialEq)]
struct Line {
    vector: Point,
}

impl Line {
    fn new(mut a: Point, b: Point) -> Line {
        a.x -= b.x;
        a.y -= b.y;

        if a.x == 0 {
            return Line{ vector: Point{ x: 0, y: 1 }}
        }

        if a.y == 0 {
            return Line{ vector: Point{ x: 1, y: 0 }}
        }

        if a.y < 0 {
            a.x *= -1;
            a.y *= -1;
        }

        let gcd = Self::gcd(a.x.abs(), a.y.abs());

        Line {
            vector: Point {
                x: a.x / gcd,
                y: a.y / gcd
            }
        }
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        if b < a {
            mem::swap(&mut a, &mut b)
        }

        while a != 0 {
            let c = b % a;
            b = a;
            a = c;
        }

        b
    }
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..points.len() {
            let a = Point {x: points[i][0], y: points[i][1]};
            let mut lines = HashMap::new();

            for j in i + 1..points.len() {
                let line = Line::new(a,  Point{x: points[j][0], y: points[j][1]});
                let cnt = lines.entry(line).or_insert(0i32);
                *cnt += 1;
                ans = ans.max(*cnt)
            }
        }

        ans + 1
    }
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, Solution::max_points(vec![vec![1,1],vec![3,2],vec![5,3],vec![4,1],vec![2,3],vec![1,4]]))
    }
}