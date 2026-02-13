use rand::Rng;

pub fn transform(input: &str) -> String {
    let mut text = input.to_string();

    text = replace_vocabulary(&text);
    text = replace_greetings(&text);
    text = text.replace('〜', "〜〜〜💨");
    text = text.replace('！', "🚬💨");
    text = text.replace('!', "🚬💨");
    text = text.replace('。', "🚬");
    text = text.replace('、', "🚬");
    text = transform_sentence_endings(&text);

    text
}

fn replace_vocabulary(text: &str) -> String {
    let replacements = [
        ("美味しい", "ニコチンが染みる"),
        ("最高", "一服の至福"),
        ("休憩", "一服タイム"),
    ];
    let mut result = text.to_string();
    for (from, to) in replacements {
        result = result.replace(from, to);
    }
    result
}

fn replace_greetings(text: &str) -> String {
    let replacements = [
        ("おはよう", "おはヤニ"),
        ("おやすみ", "おやすヤニ"),
    ];
    let mut result = text.to_string();
    for (from, to) in replacements {
        result = result.replace(from, to);
    }
    result
}

fn transform_sentence_endings(text: &str) -> String {
    let mut result = String::new();
    let mut rng = rand::thread_rng();
    let yani_endings = ["ヤニ", "ヤニね", "ヤニよ"];
    let fillers = ["ふぅ...", "一服..."];

    let chars: Vec<char> = text.chars().collect();
    let text_len = chars.len();
    let mut i = 0;
    let mut sentence_count = 0;

    while i < text_len {
        let remaining: String = chars[i..].iter().collect();
        if remaining.starts_with("🚬💨") {
            let ending = yani_endings[rng.gen_range(0..yani_endings.len())];
            result.push_str(ending);
            result.push_str("🚬💨");
            let marker_chars: Vec<char> = "🚬💨".chars().collect();
            i += marker_chars.len();
            sentence_count += 1;

            if sentence_count > 0 && i < text_len {
                if rng.gen_bool(0.4) {
                    let filler = fillers[rng.gen_range(0..fillers.len())];
                    result.push_str(filler);
                }
            }
        } else if remaining.starts_with("🚬") {
            let ending = yani_endings[rng.gen_range(0..yani_endings.len())];
            result.push_str(ending);
            result.push_str("🚬");
            let marker_chars: Vec<char> = "🚬".chars().collect();
            i += marker_chars.len();
            sentence_count += 1;

            if sentence_count > 0 && i < text_len {
                if rng.gen_bool(0.4) {
                    let filler = fillers[rng.gen_range(0..fillers.len())];
                    result.push_str(filler);
                }
            }
        } else if remaining.starts_with('\n') {
            let ending = yani_endings[rng.gen_range(0..yani_endings.len())];
            if !result.is_empty() && !result.ends_with('\n') && !result.ends_with("🚬") && !result.ends_with("🚬💨") {
                result.push_str(ending);
            }
            result.push('\n');
            i += 1;
            sentence_count += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    if !result.is_empty()
        && !result.ends_with("🚬")
        && !result.ends_with("🚬💨")
        && !result.ends_with('\n')
    {
        let ending = yani_endings[rng.gen_range(0..yani_endings.len())];
        result.push_str(ending);
    }

    result
}

#[cfg(test)]
fn transform_deterministic(input: &str) -> String {
    let mut text = input.to_string();

    text = replace_vocabulary(&text);
    text = replace_greetings(&text);
    text = text.replace('〜', "〜〜〜💨");
    text = text.replace('！', "🚬💨");
    text = text.replace('!', "🚬💨");
    text = text.replace('。', "🚬");
    text = text.replace('、', "🚬");
    text = transform_sentence_endings_deterministic(&text);

    text
}

#[cfg(test)]
fn transform_sentence_endings_deterministic(text: &str) -> String {
    let mut result = String::new();
    let ending = "ヤニ";

    let chars: Vec<char> = text.chars().collect();
    let text_len = chars.len();
    let mut i = 0;

    while i < text_len {
        let remaining: String = chars[i..].iter().collect();
        if remaining.starts_with("🚬💨") {
            result.push_str(ending);
            result.push_str("🚬💨");
            let marker_chars: Vec<char> = "🚬💨".chars().collect();
            i += marker_chars.len();
        } else if remaining.starts_with("🚬") {
            result.push_str(ending);
            result.push_str("🚬");
            let marker_chars: Vec<char> = "🚬".chars().collect();
            i += marker_chars.len();
        } else if remaining.starts_with('\n') {
            if !result.is_empty() && !result.ends_with('\n') && !result.ends_with("🚬") && !result.ends_with("🚬💨") {
                result.push_str(ending);
            }
            result.push('\n');
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    if !result.is_empty()
        && !result.ends_with("🚬")
        && !result.ends_with("🚬💨")
        && !result.ends_with('\n')
    {
        result.push_str(ending);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vocabulary_replacement() {
        let result = transform_deterministic("美味しい料理");
        assert!(result.contains("ニコチンが染みる"));
        assert!(!result.contains("美味しい"));
    }

    #[test]
    fn test_vocabulary_saikou() {
        let result = transform_deterministic("最高の一日");
        assert!(result.contains("一服の至福"));
        assert!(!result.contains("最高"));
    }

    #[test]
    fn test_vocabulary_kyukei() {
        let result = transform_deterministic("休憩しよう");
        assert!(result.contains("一服タイム"));
        assert!(!result.contains("休憩"));
    }

    #[test]
    fn test_greeting_ohayou() {
        let result = transform_deterministic("おはよう");
        assert!(result.contains("おはヤニ"));
    }

    #[test]
    fn test_greeting_oyasumi() {
        let result = transform_deterministic("おやすみ");
        assert!(result.contains("おやすヤニ"));
    }

    #[test]
    fn test_smoke_expression() {
        let result = transform_deterministic("いい天気だ〜");
        assert!(result.contains("〜〜〜💨"));
    }

    #[test]
    fn test_exclamation_mark_fullwidth() {
        let result = transform_deterministic("すごい！");
        assert!(result.contains("🚬💨"));
        assert!(!result.contains('！'));
    }

    #[test]
    fn test_exclamation_mark_halfwidth() {
        let result = transform_deterministic("すごい!");
        assert!(result.contains("🚬💨"));
        assert!(!result.contains('!'));
    }

    #[test]
    fn test_period_replacement() {
        let result = transform_deterministic("今日はいい天気。");
        assert!(result.contains("🚬"));
        assert!(!result.contains('。'));
    }

    #[test]
    fn test_comma_replacement() {
        let result = transform_deterministic("今日は、いい天気。");
        assert!(result.contains("🚬"));
        assert!(!result.contains('、'));
    }

    #[test]
    fn test_sentence_ending_added() {
        let result = transform_deterministic("こんにちは");
        assert!(result.ends_with("ヤニ"));
    }

    #[test]
    fn test_sentence_ending_at_period() {
        let result = transform_deterministic("今日はいい天気。明日もいい天気。");
        assert!(result.contains("ヤニ🚬"));
    }

    #[test]
    fn test_full_transform() {
        let result = transform_deterministic("おはよう。今日は美味しいご飯を食べた。最高！");
        assert!(result.contains("おはヤニ"));
        assert!(result.contains("ニコチンが染みる"));
        assert!(result.contains("一服の至福"));
        assert!(!result.contains('。'));
        assert!(result.contains("🚬"));
        assert!(result.contains("🚬💨"));
    }

    #[test]
    fn test_empty_input() {
        let result = transform_deterministic("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_no_special_chars() {
        let result = transform_deterministic("テスト");
        assert_eq!(result, "テストヤニ");
    }

    #[test]
    fn test_combined_greeting_and_smoke() {
        let result = transform_deterministic("おはよう〜");
        assert!(result.contains("おはヤニ"));
        assert!(result.contains("〜〜〜💨"));
    }

    #[test]
    fn test_multiple_sentences() {
        let result = transform_deterministic("一つ。二つ。三つ。");
        let count = result.matches("ヤニ🚬").count();
        assert_eq!(count, 3, "Each sentence should end with ヤニ🚬, got: {}", result);
    }

    #[test]
    fn test_random_transform_runs() {
        let result = transform("おはよう。美味しいご飯！休憩しよう〜");
        assert!(!result.is_empty());
        assert!(result.contains("おはヤニ"));
        assert!(result.contains("ニコチンが染みる"));
        assert!(result.contains("一服タイム"));
        assert!(result.contains("〜〜〜💨"));
    }
}
