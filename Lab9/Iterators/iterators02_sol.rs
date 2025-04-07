// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(|word| capitalize_first(word))
        .collect()
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    words
        .iter()
        .map(|word| capitalize_first(word))
        .collect()
}

fn main() {
    // You can optionally experiment here.
    // Example usage:
    let words = vec!["hello", "world"];
    let words2 = vec!["hello", " ", "world"];
    let capitalized_words = capitalize_words_vector(&words);
    println!("{:?}", capitalized_words); // Output: ["Hello", "World"]
    let combined_string = capitalize_words_string(&words2);
    println!("{}", combined_string); // Output: "Hello World"
    let single_word = "hello";
    let capitalized_word = capitalize_first(single_word);
    println!("{}", capitalized_word); // Output: "Hello"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
