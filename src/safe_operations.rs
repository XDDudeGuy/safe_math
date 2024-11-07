/// Has two variants Add and Multiply with no fields, used in the safe_math!() macro
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Multiply
}

/// Allows conversion from bool to Operation and vice versa true means Add, false means Multiply
impl From<bool> for Operation {
    fn from(value: bool) -> Self {
        match value {
            true => Operation::Add,
            false => Operation::Multiply
        }
    }
}

/// Takes three expressions, two number inputs and one variant of the enum Operation. It combines the two numbers in whatever way was chosen or in the case of most errors evaluates to the Err variant with the field containing the larger of the two input numerical expressions
#[macro_export]
macro_rules! safe_math {
    ($in_x:expr, $in_y:expr, $operation:expr) => {
        {
            let mut error = false;
            let mut set_error = || -> i128 {
                error = true;
                1
            };
            if $in_x > 9223372036854775807 || $in_y > 9223372036854775807 {
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
            if !error {
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
            } else {
                Result::Err(returned_value)
            }
        }
    }
}
