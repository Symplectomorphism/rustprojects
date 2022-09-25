fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);

    println!("The length of '{}' is {}.", s1, calculate_length(&s1));

    {
        let r1 = &mut s1;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s1;

    println!("{}", r2);

    let r3 = &s1;
    let r4 = &s1; 
    println!("{}, and {}", r3, r4);
    // variables r3 and r4 will not be used after this point.

    let r5 = &mut s1; // no problem.
    println!("{}", r5);

}

fn calculate_length(s: &String) -> usize { // s is a  reference to a String
    s.len()
} // Here, s goes out of scope. But because it odes not have ownership of what 
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
