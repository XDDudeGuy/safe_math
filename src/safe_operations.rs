///This Attempts to add two numbers, if they cannot be added without overflow it returns the larger number
// pub fn safe_add<T>(in_x: T, in_y: T) -> Result<i64, T> where i128: From<T>, T: Clone + Copy {    
//     let x: i128 = i128::from(in_x);
//     let y: i128 = i128::from(in_y);
//     let returned_value = match x > y {
//         true => in_x,
//         false => in_y
//     };

//     let sum: i128 = x + y;
//     let result = i64::try_from(sum);
//     match result {
//         Ok(v) => return Ok(v),
//         Err(_) => return Err(returned_value)
//     };
// }

// ///This Attempts to multiply two numbers, if they cannot be multiplied without overflow it returns the larger number
// pub fn safe_multiply<T>(in_x: T, in_y: T) -> Result<i64, T> where i128: From<T>, T: Clone + Copy {
//     let x: i128 = i128::from(in_x);
//     let y: i128 = i128::from(in_y);
//     let returned_value = match x > y {
//         true => in_x,
//         false => in_y
//     };

//     let product: i128 = x*y;
//     let result = i64::try_from(product);
//     match result {
//         Ok(v) => return Ok(v),
//         Err(_) => return Err(returned_value)
//     };
// }

pub enum Operation {
    Add,
    Multiply
}

#[macro_export]
macro_rules! safe_math {
    ($in_x:expr, $in_y:expr, $operation:expr) => {
        {
            let data_error = "The type and data of the first two inputs must be able to be turned into a 128 bit signed integer and should not be 128 bit integers";
            if $in_x > 18446744073709551615 || $in_y > 18446744073709551615 {
                panic!(data_error);
            }
            let result_x: i128 = i128::try_from($in_x);
            let x = match result_x {
                Ok(v) => v,
                Err(_) => panic!(data_error)
            };
            let result_y: i128 = i128::try_from($in_y);
            let y = match result_y {
                Ok(v) => v,
                Err(_) => panic!(data_error)
            };
            let returned_value = match x < y {
                true => y,
                false => x
            }
            match $operation {
                Add => {
                    let sum: i128 = x+y;
                    let result_sum = i64::try_from(sum);
                    match result_sum {
                        Ok(v) => v,
                        Err(_) => returned_value
                    }
                },
                Multiply => {
                    let product: i128 = x*y;
                    let result_product = i64::try_from(sum);
                    match result_product {
                        Ok(v) => v,
                        Err(_) => returned_value
                    }
                }
            }
        }
    }
}