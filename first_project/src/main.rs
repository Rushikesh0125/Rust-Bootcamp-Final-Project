fn concatenate_strings(str1: &str, str2: &str) -> String {
    
    let mut result = String::new();

    result.push_str(str1);

    result.push_str(str2);

    result
}

fn main() {
    // Example usage
    let string1 = "Hello, ";
    let string2 = "world!";
    
    let concatenated_string = concatenate_strings(string1, string2);

    println!("Concatenated String: {}", concatenated_string);
}
