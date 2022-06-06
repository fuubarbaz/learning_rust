/* A string slice (&str) is a immutable view of a string, String type is growable mutable owned UTF-8 encoded string.
With String type underlying string will always be allocated in heap.
*/
fn main() {
    // create a string literal and store into a variable s1, string literals are string slices and are stored in applications binary.
    let s1 = "foo bar baz :-)";
    // String that is stored in heap
    let s2 = String::from("hello from heap");
    // Convert to String from string literal
    let s3 = "convert me".to_string();
    let s4 = "convert again".to_owned();
    // string slice that references a owned string
    let s5 = &s4[..];
    // To manipulate string first make it mutable
    let mut s6 = String::from("some text");
    println!("s6 :: {}", s6);
    s6.push_str(" additional text");
    println!("change s6 {}", s6);

    // Replace .. denotes full range
    s6.replace_range(.., "overridden");
    println!("s6 replaced :: {}", s6);

    // Concatenating using + operator
    let s7 = String::from(" string 7");
    let s8 = String::from(" string 8");
    // s7 is moved to s9 and then the content of s8 is appended to it.
    let s9 = s7 + &s8;
    println!("s9 :: {}", s9);

    // Using format macro ..(less efficient as needs to copy all), it can take string slices and type both
    let s10 = String::from(" fuu");
    let s11 = String::from(" bar");
    let s12 = String::from(" baz");

    let s13 = format!("{}{}{}{}", s10, s11, s12, " string type");
    println!("s13 {}", s13);
    // We can use concat, put two string types into an array and call concat
    let s14 = ["part one ", "part two"].concat();
    println!("s14 :: {}", s14);

    // We can use format to get String type from string slices
    let s15 = format!("{}{}", "one ", "two");
    println!("s15 :: {}", s15);

    // If we want to concat two string slices and want the result to be string slice then we can use concat macro
    let s16 = concat!("head ", "tail");
    println!("s16:: {}", s16);
}
