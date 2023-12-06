//! A Rust package that provides utility functions inspired by or ported from other programming languages.
//! 
//! rusty_utils aims to simplify intricate and hard-to-read instructions in Rust, allowing you to express complex operations with clarity and brevity.
//! Inspired by the conciseness of other programming languages, rusty_utils provides a set of functions that compact multiple steps into a single, readable call.


/// Compacts the standard ` if (condition) {} else {} ` into a single call, to improve readability and please users accustomed  to the ternary operator in other languages.
/// 
/// # Arguments
///
/// - `condition` - A boolean value representing the condition to check.
/// - `if_true` - The value returned if the `condition` is true.
/// - `if_false` - The value returned if the `condition` is false.
///
///      ### Notes
///     
///      - The type of `if_true` and `if_false` must be the same.
///     
///      This function allows you to use any type for the `if_true` and `if_false` arguments,
///      but it requires both arguments to be of the same type.
///
/// # Returns
///
/// The result of the ternary operation, which is either `if_true` or `if_false`, depending on the condition's result.
/// 
/// # Panics
///
/// This function does not panic under normal circumstances.
/// 
/// # Examples
/// In these examples, the `ternary_operator` function is used to make decisions based on different conditions, types, and nesting scenarios.
/// Remember that while the function provides flexibility, excessive nesting might affect code readability.
///
/// - Applying a discount based on age:
///
///      ```rust
///      use rusty_utils::ternary_operator;
///     
///      fn calculate_discount(price: f32, age: u32) -> f32 {
///          const ELIGIBLE_AGE: u32 = 60;
///          const DISCOUNT_PERCENTAGE: f32 = 0.10;
///     
///          let discount = ternary_operator(age > ELIGIBLE_AGE, DISCOUNT_PERCENTAGE, 0.0);
///          price * (1.0 - discount)
///      }
///     
///      let price = 100.0;
///      let discounted_price = calculate_discount(price, 65);
///      assert_eq!(discounted_price, 90.0);
///     
///      let regular_price = calculate_discount(price, 50);
///      assert_eq!(regular_price, 100.0);
///      ```
/// <br>
/// 
/// - Selecting a type dynamically:
///
///      ```rust
///      use rusty_utils::ternary_operator;
///     
///      fn get_default<T: Default>(value: Option<T>) -> T {
///          ternary_operator(value.is_some(), value.unwrap_or_default(), T::default())
///      }
///     
///      let integer_value: Option<i32> = Some(42);
///      let result_int = get_default(integer_value);
///      assert_eq!(result_int, 42);
///     
///      let string_value: Option<&str> = None;
///      let result_str = get_default(string_value);
///      assert_eq!(result_str, "");
///      ```
/// <br>
/// 
/// - Deciding based on boolean conditions:
///
///      ```rust
///      use rusty_utils::ternary_operator;
///     
///      fn is_even_or_odd(number: i32) -> &'static str {
///          ternary_operator(number % 2 == 0, "Even", "Odd")
///      }
///     
///      assert_eq!(is_even_or_odd(4), "Even");
///      assert_eq!(is_even_or_odd(7), "Odd");
///      ```
/// <br>
/// 
/// - Combining conditions with nesting (not recommended for readability):
///
///      ```rust
///      use rusty_utils::ternary_operator;
///     
///      fn complicated_calculation(a: i32, b: i32, use_a: bool) -> i32 {
///          ternary_operator(use_a, ternary_operator(b > 0, a, 0), 2)
///      }
///     
///      assert_eq!(complicated_calculation(1, -1, true), 0);
///      assert_eq!(complicated_calculation(1, -1, false), 2);
///      ```

pub fn ternary_operator <T> (condition: bool, if_true: T, if_false: T) -> T {
    if condition { if_true } else { if_false }
}

/// Compacts the standar `string_value.chars().rev().collect()` into a single call, to improve
/// readability.
///
/// # Arguments
///
/// - `string_value` - The string to be reversed by the function.
///
/// # Returns
///
/// The reversed input string.
///
/// # Panics
///
/// This function does not panic under normal circumstances.
///
/// # Examples
///
/// - Reversing a input string:
///
///      ```rust
///      use rusty_utils::reverse_string;
///     
///      let mut input: &str = "Hello, World";
///
///      //It's not necesary to trim the string, since it will work regardless,
///      //but is more aesthetically pleasing.
///      let sentence = input.trim();
///
///      let reversed_sentence = reverse_string(sentence);
///      assert_eq!(reversed_sentence, "dlroW ,olleH");
///      ```
/// <br>

pub fn reverse_string (string_value: &str) -> String {
    string_value.chars().rev().collect()
}

//? ===========
//? = [Tests] =
//? ===========

#[cfg(test)]

//? -------------------------------
//? - [Tests] - Ternary operators -
//? -------------------------------

mod ternary_operator_tests {
    use super::*;

    // Test for int values.
    #[test]
    fn ternary_operator_integers() {
        // True.
        let result = ternary_operator(true, 2, 6);
        assert_eq!(result, 2);
        
        // False.
        let result = ternary_operator(false, 2, 6);
        assert_eq!(result, 6)
    }

    // Test for float values.
    #[test]
    fn ternary_operator_floats() {
        // True.
        let result = ternary_operator(true, 2.1, 6.6);
        assert_eq!(result, 2.1);
        
        // False.
        let result = ternary_operator(false, 2.1, 6.6);
        assert_eq!(result, 6.6)
    }

    // Test for bool values.
    #[test]
    fn ternary_operator_bools() {
        // True.
        let result = ternary_operator(true, true, false);
        assert_eq!(result, true);
        
        // False.
        let result = ternary_operator(false, true, false);
        assert_eq!(result, false)
    }

    // Test for nested expresions.
    // !! Not recommended, it can affect readabilty.
    #[test]
    fn ternary_operator_nested() {
        // True.
        let result = ternary_operator(true, ternary_operator(true, 1, 0), 2);
        assert_eq!(result, 1);
        
        // False.
        let result = ternary_operator(true, ternary_operator(false, 1, 0), 2);
        assert_eq!(result, 0)
    }

}

#[cfg(test)]

//? ----------------------------
//? - [Tests] - Reverse String -
//? ----------------------------

mod reverse_string_tests {
    use super::*;

    //Test reversing normal string
    #[test]
    fn reverse_string_normal() {
        let string_to_reverse: &str = "test";
        let result = reverse_string(string_to_reverse);
        assert_eq!(result, "tset");
    }

    //Test reversing string with spaces
    #[test]
    fn reverse_string_phrase() {
        let string_to_reverse: &str = "test phrase";
        let result = reverse_string(string_to_reverse);
        assert_eq!(result, "esarhp tset");
    }

    //Test untrimmed string
    #[test]
    fn reverse_string_untrimmed() {
        let string_to_reverse: &str = " test ";
        let result = reverse_string(string_to_reverse);
        assert_eq!(result, " tset ");
    }
}
