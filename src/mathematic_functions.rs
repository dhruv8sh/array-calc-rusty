
    pub fn average<T: std::ops::Add<T, Output=T> + std::ops::Div<Output=T>>(arr: &Vec<T>) -> T {
        let mut res: T;
        for x in arr {
            res = res + *x;
        }
        res / arr.len()
    }

    pub fn max_num<T: std::ops::Div<Output=T>>(arr: &Vec<T>) -> T {
        let mut res = T::MIN;
        for x in arr {
            if res < *x {
                res = *x;
            }
        }
        res
    }

    pub fn min_num<T: std::ops::Div<Output=T>>(arr: &Vec<T>) -> T {
        let mut res = T::MAX;
        for x in arr {
            if res > *x {
                res = *x;
            }
        }
        res
    }

    pub fn print_all<T>(arr: &Vec<T>) -> bool{
        if arr.len() == 0{
            return false;
        }
        for x in arr {
            print!("{}\t",x);
        }
        println!();
        true
    }
