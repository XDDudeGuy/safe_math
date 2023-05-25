// attempts to add two numbers, if the numbers cannot be added without an overflow the second input is returned
fn safe_add(in_x: u32, in_y: u32) -> Result<u32, u32> {
    let x: u64 = u64::from(in_x);
    let y: u64 = u64::from(in_y);

    let sum: u64 = x + y;
    let result = u32::try_from(sum);
    match result {
        Ok(v) => return Ok(v),
        Err(_) => return Err(in_y)
    };
}