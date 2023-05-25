// attempts to add two numbers, if the numbers cannot be added without an overflow the second input is returned
fn safe_add(in_x: i32, in_y: i32) -> Result<i32, i32> {    
    let x: i64 = i64::from(in_x);
    let y: i64 = i64::from(in_y);

    let sum: i64 = x + y;
    let result = i32::try_from(sum);
    match result {
        Ok(v) => return Ok(v),
        Err(_) => return Err(in_y)
    };
}
// see safe_add
fn u_safe_add(in_x: u32, in_y: u32) -> Result<u32, u32> {    
    let x: u64 = u64::from(in_x);
    let y: u64 = u64::from(in_y);

    let sum: u64 = x + y;
    let result = u32::try_from(sum);
    match result {
        Ok(v) => return Ok(v),
        Err(_) => return Err(in_y)
    };
}

// attempts to multiply two numbers, if the numbers cannot be multiplied without an overflow the second input is returned
fn safe_multiply(in_x: i32, in_y: i32) -> Result<i32, i32> {
    let x: i64 = i64::from(in_x);
    let y: i64 = i64::from(in_y);

    let product: i64 = x*y;
    let result = i32::try_from(product);
    match result {
        Ok(v) => return Ok(v),
        Err(_) => return Err(in_y)
    };
}

fn u_safe_multiply(in_x: u32, in_y: u32) -> Result<u32, u32> {
    let x: u64 = u64::from(in_x);
    let y: u64 = u64::from(in_y);

    let product: u64 = x*y;
    let result = u32::try_from(product);
    match result {
        Ok(v) => return Ok(v),
        Err(_) => return Err(in_y)
    };
}
