fn main() {
    let x = 5;

    println!("The value of x is {x}");

    {
        let x = x * 2;
        println!("The value of x is {x}")
    }

    let x = x + 1;
    println!("The value of x is {x}");

    println!("Floating point numbers");
    let y = 5.0;
    let z = y / 2.0;
    println!("The value for y: {y}, z: {z}");

    println!("Integer division");
    let y = 5;
    let z = y / 2;
    println!("The value for y: {y}, z: {z}");

    let heart_eyed_cat = '😻';
    println!("Heart Eyed Cat {heart_eyed_cat}");

    let foo = (1, "joe");
    let foo_name = foo.1;
    println!("Foo Name: {foo_name}");

    let months: [char; 12] = ['j', 'f', 'm', 'a', 'm', 'j', 'j', 'a', 's', 'o', 'n', 'd'];
    let jan = months[0];
    println!("Months {jan}")
}
