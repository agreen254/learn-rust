fn main() {
    // immutable variables cannot be changed
    // let x = 5;
    // x = 6;

    let mut x = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");

    // a constant must have its type declared
    const PI: f32 = 3.14159;
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // more on shadowing
    // if the first variable is shadowed by the other it means that the compiler
    // will use the second variable, effectively having it overshadow the first
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("y in the inner scope = {y}");
    }

    println!("y in the outer scope = {y}");

    // using the let keyword effectively creates a new variable
    // we can override a previous variable by re-using its name
    // in the second let declaration
    let spaces = "    ";
    let spaces = spaces.len();
    println!("There are {spaces} spaces.")
    // contrasted with:
    // this will give an error because the len of sp is a number
    // and the first sp declaration is a String
    // let mut sp = "    ";
    // sp = sp.len();
}
