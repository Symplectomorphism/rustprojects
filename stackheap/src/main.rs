fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);


    let s3 = String::from("hello");  // s3 comes into scope
    
    takes_ownership(s3);             // s3's value moves into the function...
                                    // ... and so is no longer valid here.
    
    let x = 5;                      // x comes into scope.
    
    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward.

    println!("x = {}", x);

//    println!("s = {}", s3);       // this prevents compilation.

    let s4 = gives_ownership();     // gives_ownership moves its return
                                    // value into s4.

    let s5 = String::from("hello"); // s5 comes into scope

    let s6 = takes_and_gives_back(s5);  // s5 is moved into
                                        // taken_and_gives_back, which also
                                        // moves its return value into s6.

    let (s7, len) = calculate_length(s6);

    println!("The length of '{}' is {}.", s7, len);

} // Here, x goes out of scope, then s. But because s3's value was moved, nothing
  // special happens.
  //
  // s6 goes out of scope and is dropped. s5 was moved, so nothing happens. s4
  // goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happpens.


fn gives_ownership() -> String { // gives_ownership will move its
                                 // return value into the function
                                 // that calls it.

    let some_string = String::from("yours");    // some_string comes into scope
                                                
    some_string                                 // some_string is returned and 
                                                // moves out to the calling
                                                // function.

}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string    // a_string is returned and moves out to the calling function.

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
