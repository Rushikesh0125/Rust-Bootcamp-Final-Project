fn concatenate_strings(str1: &str, str2: &str) -> String {
    //creating a result variable
    let mut result = String::new();

    //using push_str to add the string one to result
    result.push_str(str1);
    //adding second string will concat it with second string
    result.push_str(str2);
    //returning result
    result
}

fn main() {
    // Two string slices
    let string1 = "Hello, ";
    let string2 = "world!";
    //calling concatenate strings function and passing to strings
    let concatenated_string = concatenate_strings(string1, string2);

    println!("Concatenated String: {}", concatenated_string);
}
