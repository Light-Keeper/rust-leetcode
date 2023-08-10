struct DataStream {
    len: i32,
    value: i32,
    k: i32
}

impl DataStream {
    fn new(value: i32, k: i32) -> Self {
        DataStream{len: 0, value, k}
    }

    fn consec(&mut self, num: i32) -> bool {
        if num == self.value {
            self.len += 1
        } else {
            self.len = 0
        }

        self.len >= self.k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut stream = DataStream::new(4, 3);
        let a = &[
            stream.consec(4),
            stream.consec(4),
            stream.consec(4),
            stream.consec(3),
            stream.consec(4),
            stream.consec(4),
            stream.consec(4),
            stream.consec(4),
        ];

        assert_eq!(a, &[
            false,
            false,
            true,
            false,
            false,
            false,
            true,
            true
        ])
    }
}