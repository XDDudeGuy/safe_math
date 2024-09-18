///This Attempts to add two numbers, if they cannot be added without overflow it returns the larger number
pub fn safe_add<T>(in_x: T, in_y: T) -> Result<i64, T> where i128: From<T>, T: Clone + Copy {    
    let x: i128 = i128::from(in_x);
    let y: i128 = i128::from(in_y);
    let returned_value = match x > y {
        true => in_x,
        false => in_y
    };

    let sum: i128 = x + y;
    let result = i64::try_from(sum);
    match result {
        Ok(v) => return Ok(v),
        Err(_) => return Err(returned_value)
    };
}

///This Attempts to multiply two numbers, if they cannot be added without overflow it returns the larger number
pub fn safe_multiply<T>(in_x: T, in_y: T) -> Result<i64, T> where i128: From<T>, T: Clone + Copy {
    let x: i128 = i128::from(in_x);
    let y: i128 = i128::from(in_y);
    let returned_value = match x > y {
        true => in_x,
        false => in_y
    };

    let product: i128 = x*y;
    let result = i64::try_from(product);
    match result {
        Ok(v) => return Ok(v),
        Err(_) => return Err(returned_value)
    };
}
