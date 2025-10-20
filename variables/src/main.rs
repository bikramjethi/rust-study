const X: i32 = 5;
fn main() {
    println!("Hello, world! {}", X);
    test_references();
}

/**
 * This is just a test function I've created as I was going through the
 * references section of the rust handbook
 *
 * I just need this to test a few hypothesis I have while I am reading the handbook
 */
fn test_references() {
    println!("Test references is being called");

    let mut s1 = String::from("hello");

    let s2 = &mut s1;
    let s3 = &mut s1;

    println!("{s2}, {s3}");
}
