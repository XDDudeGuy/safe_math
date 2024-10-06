# Safe Math
## Math without overflow!
Adds a macro and an enum ```safe_math!()``` and ```Operation```
### [src/safe_math.rs](https://github.com/XDDudeGuy/safe_math/blob/master/src/safe_operations.rs)
#### Operation:
```
pub enum Operation {
  ...
}
```
Has two variants Add and Multiply with no fields, used in the [safe_math!()](#safe_math) macro to determine whether you are adding or multiplying the inputted numbers
#### safe_math!():
```
#[macro_export]
macro_rules! safe_math {
  ($in_x:expr, $in_y:expr, $operation:expr)
}
```
This takes three expression arguments, the first two being the numbers you wish to use and the third being the operation you wish to apply to the numbers whether that be addition or multiplication. The first two expressions should be below the unsigned 64 bit integer limit and the third should be a variant of the [Operation](#Operation) enum. Most errors will just return the larger of the two number input variables in the Err() variant of the Result enum

