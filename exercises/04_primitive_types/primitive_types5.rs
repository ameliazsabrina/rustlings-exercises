fn main() {
    let cat: (&str, f64) = ("Furry McFurson", 3.5);
    let name = cat.0;
    let age = cat.1;

    // TODO: Destructure the `cat` tuple in one statement so that the println works.

    println!("{name} is {age} years old");
}
