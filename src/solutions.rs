pub mod solutions {
    pub fn problem1() -> i32 {
        let mut x = 0;
        for n in 1..1000 {
            match n {
                of5 if of5 % 5 == 0 => x = x + of5,
                of3 if of3 % 3 == 0 => x = x + of3,
                _ => ()
            }
        }
        return x;
    }

    pub fn problem2() -> i64 {
        fn fibonacci(a: i64, b:i64, max: i64, acc: i64) -> i64 {
            if b <= max {
                let next = a + b;
                let new_acc = match next {
                    even if even % 2 == 0 => acc + even,
                    odd => acc,
                };

                return fibonacci(b, a + b, max, new_acc);
            } else {
                return acc;
            }
        }


        let ans = fibonacci(1, 2, 4000000, 2);
        return ans;
    }

    pub fn problem101() -> i64 {
        fn poly_fn(n: i64) -> i64 {
            let ans = 1 - n + n.pow(2) - n.pow(3)
                + n.pow(4) - n.pow(5) + n.pow(6) - n.pow(7) + n.pow(8) - n.pow(9) + n.pow(10);
            return ans
        }
        panic!("Not implemented");
    }

    use std::collections::HashSet;
    pub fn problem52() -> i64 {
        fn num_to_set(num: i64) -> HashSet<char> {
            let mut set = HashSet::new();
            for x in num.to_string().chars() {
                set.insert(x);
            }
            return set;
        }

        let mut find_x = false;
        let mut n = 1;
        while !find_x {
            n = n + 1;
            let factors = vec![1,2,3,4,5,6];
            let sets: Vec<HashSet<char>> = factors.iter()
                .map(|x| num_to_set(x * n))
                .collect();

            find_x = (sets[0] == sets[1])
                && (sets[1] == sets[2])
                && (sets[2] == sets[3])
                && (sets[3] == sets[4])
                && (sets[4] == sets[5]);
        }
        return n;
    }
}
