fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s);

    // why can String be mutated but literals cannot?
    // the difference lies in how each type interacts with memory
    // in the case of a string literal, the text is hardcoded into
    // the compiled executable file, making it fast and efficient.
    // but for a mutable object, we cannot put a pre-determined blob
    // of memory for it because the size is unknown at compile time
    // and may change while we run the program.
    // in the case of the String type, in order to support a growing,
    // mutable piece of text, we need to allocate memory on the heap,
    // unknown at compile time, to hold the contents.
    // this means:
    //   the memory must be requested from the memory allocator at runtime
    //   this memory must be returned to the allocator when the String is done
    // the first part is handled by calling String::from
    // this requests the memory that it needs.
    // for the second part, rust automatically returns the memory once the
    // variable that 'owns' it goes out of scope
    {
        let s = String::from("hello"); // s is valid now
                                       // do stuff
    } // the scope is now over, and s is no longer valid
      // when the variable goes out of scope, rust automatically calls the
      // special function 'drop'

    // here y is also equal to 5, but if we add 3 to x and print y after
    // y will still be shown as 5
    let x = 5;
    let y = x;

    // this behaves as a reference type, meaning the String object itself
    // is not copied: s2 and s1 are pointing to the same "hello"
    let s1 = String::from("hello");
    let s2 = s1;
    // this poses a problem: if s1 and s2 are pointing to the same memory block,
    // when we go out of scope 'drop' will be called twice and the same
    // memory will try to be freed.
    // this is called a 'duoble free' error and can potentially curropt memory.
    // to ensure memory safety, after we declare s2 rust will consider
    // s1 to no longer be valid.
    // for example:
    // println!("{}", s1); will throw a compiling ownership error
    //
    // we say that s1 was 'moved' into s2.
    // similar to a shallow copy (like in react useState) but also
    // invalidates the first variable.
    // rust will never automatically create a "deep" copy of data
    //
    // if we really want to create a deep copy we can use the 'clone' method
    let s3 = s2.clone();

    // for data types with a known size, such as i32, which get put on the stack
    // they all have a Copy trait associated with them.
    // if a type has a Drop trait associated with it, however, we cannot
    // use the Copy trait on it.
    //
    // a tuple can have the Copy trait only if all elements inside of the tuple
    // also have the Copy trait.


    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.