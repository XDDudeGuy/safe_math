# Safe Math
## Math without overflow!
Adds two functions ```safe_add()``` and ```safe_multiply()```
### [src/safe_math.rs](https://github.com/XDDudeGuy/safe_math/blob/master/src/safe_operations.rs)
#### safe_add():
```
pub fn safe_add<T>(in_x: T, in_y: T) -> Result<i64, T> where
  i128: From<T>,
  T: Clone + Copy
```
Takes two generic inputs of the same type, provided they are numbers or can be converted to 128 bit signed integers and implement the Copy trait, and adds them, if there is overflow it returns the larger of the two numbers in the Err Variant of the Result enum

#### safe_multiply():
```
pub fn safe_multiply<T>(in_x: T, in_y: T) -> Result<i64, T> where
  i128: From<T>,
  T: Clone + Copy
```
Does the same as [safe_add()](#header-4) but instead of adding it multiplies
