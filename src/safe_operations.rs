/// This allows you to choose whether you are adding or multiplying in the safe_math!() macro
pub enum Operation {
    Add,
    Multiply
}

#[macro_export]
macro_rules! safe_math {
    ($in_x:expr, $in_y:expr, $operation:expr) => {
        {
            let mut error = false;
            let mut set_error = || -> i128 {
                error = true;
                1
            };
            if $in_x > 18446744073709551614_u64 || $in_y > 18446744073709551614_u64 {
                _ = set_error();
            }
            let result_x: Result<i128, _> = i128::try_from($in_x);
            let x = match result_x {
                Ok(v) => v,
                Err(_) => set_error()
            };
            let result_y: Result<i128, _> = i128::try_from($in_y);
            let y = match result_y {
                Ok(v) => v,
                Err(_) => set_error()
            };
            let returned_value = match x < y {
                true => y,
                false => x
            };
            // looks like shit but I don't know how I could do it otherwise with the restrictions of macros
            if error==false {
                match $operation {
                    Operation::Add => {
                        let sum: i128 = x+y;
                        let result_sum = i64::try_from(sum);
                        match result_sum {
                            Ok(v) => Result::Ok(v),
                            Err(_) => Result::Err(returned_value)
                        }
                    },
                    Operation::Multiply => {
                        let product: i128 = x*y;
                        let result_product = i64::try_from(product);
                        match result_product {
                            Ok(v) => Result::Ok(v),
                            Err(_) => Result::Err(returned_value)
                        }
                    }
                }
            }
            else {
                Result::Err(returned_value)
            }
        }
    }
}