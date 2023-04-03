fn main() {
    // two data type subsets are: scalar and compound
    // rust is statically typed, meaning that it must know the types of
    // all variables at compile time

    // SCALAR TYPES
    // a scalar type represents a single value
    // rust has four primary scalar types:
    // integer
    // floating-point number
    // boolean
    // character

    // integers
    // i8       u8
    // i16      u16
    // i32      u32
    // i64      u64
    // i128     u128
    // isize    usize
    // size corresponds to the architecture you're using
    // a 64-bit system will have isize use 64 bits
    // a 32-bit system will have isize use 32 bits
    //
    // integer literals can be written as:
    // decimal: 90_213
    // hex:     0xff
    // octal:   0o77
    // binary:  0b1111_0000
    // byte:    b'A'
    //
    // in release mode, rust will integer overflow back to 0
    // e.g. 255 + 1: u8 --> 0
    // there are several ways to deal with this:
    // wrap in all modes with the wrapping_* methods, e.g. wrapping_add
    // return None if there is overflow with the checked_* methods
    // return the value and a boolean indicating if there was an overflow
    //   using the overflowing_* methods
    // saturate at the values minimum/maximum with the saturating_* methods

    // floating-point types
    // all floating types are signed
    // there is only f32 and f64
    // f64 is the default, because it's just as fast as f32 and more precise
    let p = 2.0; // f64
    let q: f32 = 3.0; // f32
                      //
    let sum = 5 + 10;
    let diff = 12 - 3;
    let prod = 10.1 * 12.2;
    let quot = 103 / 12;
    let quot_trunc = 5 / 3;
    let rem = 43 % 7;

    // booleans
    let t = true;
    let f: bool = false;

    // characters
    let c = 'Z';
    let z: char = 'Z';

    // COMPOUND TYPES
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 212);
    let (x, y, z) = tup;
    println!("{z}, {y}, {x}");
    //
    // indexing is done using a period
    println!("the second entry in the tuple is {}", tup.1);
    // a tuple with not values is called a unit
    // the value and corresponding type are denoted with ()
    // expressions implicitly return the unit value if they
    // don't return any other value

    // array
    // unlike a tuple, every element of an array must have the same type
    let a = [1, 2, 3, 4, 5];
}
