struct Point {
    x: i32,
    y: i32,
}

struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum MessageNested {
    Quit,
    Move { x: i32, y: i32  },
    Write(String),
    ChangeColor(Color),

}

enum Message {
    Quit,
    Move { x: i32, y: i32  },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum MessageSimple {
    Hello { id: i32 },
}

fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // Multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching ranges of values with ..=
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructuring Structs
    let p = Point {x: 0, y: 7};

    // let Point { x: a, y: b } = p;
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // Destructuring Enums

    // let msg = Message::ChangeColor(0, 160, 255);
    // let msg = Message::Write(String::from("Wassup bi-yatch?"));
    let msg = Message::Move{ x: 1, y: -2 };
    // let msg = Message::Quit;

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("The message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }

    // Destructuring nested Structs and Enums
    
    let msg = MessageNested::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        MessageNested::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        MessageNested::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    // Destructuring Structs and Tuples

    let ((feet, inches), Point { x, y }) = ((3, 10), Point {x: 3, y: -10});

    // Ignoring Values in a Pattern
    //
    // Ignoring an Entire Value with _

    foo(3, 4);

    // Ignoring Parts of a Value with a Nested _

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // Ignoring an Unused Variable by Starting its Name with _
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    // Ignoring Remaining Parts of a Value with ..

    let origin = Point3 { x: 0, y: 0, z: 0 };

    match origin {
        Point3 {x, .. } => println!("x is {}", x),
    }

    match numbers {
        (first, .., last) => {
            println!("Some numbers {first}, {last}");
        }
    }

    // Extra Conditionals with Match Guards

    let num = Some(5);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even.", x),
        Some(x) => println!("The number {} is odd.", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    let x = 5;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ Bindings

    let msg = MessageSimple::Hello { id: 5 };

    match msg {
        MessageSimple::Hello {
            id: id_variable @ 3 ..=7,
        } => println!("Found an id in range: {}", id_variable),
        MessageSimple::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        MessageSimple::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
