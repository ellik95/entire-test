struct StringUtils;

impl StringUtils {
    fn reverse(s: &str) -> String {
        s.chars().rev().collect()
    }

    fn is_palindrome(s: &str) -> bool {
        let cleaned: String = s.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect();
        cleaned == cleaned.chars().rev().collect::<String>()
    }

    fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }

    fn word_count(s: &str) -> usize {
        s.split_whitespace().count()
    }

    fn truncate(s: &str, max_len: usize, ellipsis: bool) -> String {
        if s.len() <= max_len {
            return s.to_string();
        }
        let end = if ellipsis { max_len.saturating_sub(3) } else { max_len };
        let truncated = &s[..end];
        if ellipsis { format!("{}...", truncated) } else { truncated.to_string() }
    }

    fn to_snake_case(s: &str) -> String {
        let mut result = String::new();
        for (i, c) in s.char_indices() {
            if c.is_uppercase() && i > 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        }
        result
    }
}

fn main() {
    let str = StringUtils::reverse("Hello, world!");
    println!("reverse {}", str);
    let str = StringUtils::is_palindrome("racecar");
    println!("is_palindrome {}", str);
    let str = StringUtils::capitalize("hello, world!");
    println!("capitalize {}", str);
    let str = StringUtils::word_count("Hello, world!");
    println!("word_count {}", str);
    let str = StringUtils::truncate("Hello, world!", 5, true);
    println!("truncate {}", str);
    let str = StringUtils::to_snake_case("Hello, world!");
    println!("to_snake_case {}", str);
}
