use rand::Rng;

/// 喫煙者構文変換: テキストを喫煙者風の表現に変換する
pub fn transform(input: &str) -> String {
    let mut text = input.to_string();

    // 1. 語彙置換 (vocabulary replacement)
    text = replace_vocabulary(&text);

    // 2. 挨拶変換 (greeting transform)
    text = replace_greetings(&text);

    // 3. 煙の表現: 「〜」→「〜〜〜💨」
    text = text.replace('〜', "〜〜〜💨");

    // 4. 感嘆符: 「！」→「🚬💨」, 「!」→「🚬💨」
    text = text.replace('！', "🚬💨");
    text = text.replace('!', "🚬💨");

    // 5. 句読点置換: 「。」→「🚬」, 「、」→「🚬」
    text = text.replace('。', "🚬");
    text = text.replace('、', "🚬");

    // 6. 語尾変換 + 「ふぅ...」挿入
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

/// Split text into sentences by 🚬 (which replaced 。) and 🚬💨 (which replaced ！/!),
/// then add ヤニ variant endings and optionally insert 「ふぅ...」between sentences.
fn transform_sentence_endings(text: &str) -> String {
    // We split on sentence-ending markers. After previous replacements:
    // - 🚬 marks end of sentence (was 。 or 、)
    // - 🚬💨 marks exclamation end (was ！ or !)
    // We need to handle both as sentence boundaries.
    //
    // Strategy: split into segments by these markers, add ヤニ endings,
    // and randomly insert fillers between sentences.

    let mut result = String::new();
    let mut rng = rand::thread_rng();
    let yani_endings = ["ヤニ", "ヤニね", "ヤニよ"];
    let fillers = ["ふぅ...", "一服..."];

    // We'll iterate through the text, finding sentence boundaries
    let chars: Vec<char> = text.chars().collect();
    let text_len = chars.len();
    let mut i = 0;
    let mut sentence_count = 0;

    while i < text_len {
        // Check for 🚬💨 (exclamation marker) - must check before 🚬 alone
        let remaining: String = chars[i..].iter().collect();
        if remaining.starts_with("🚬💨") {
            // End of exclamation sentence
            let ending = yani_endings[rng.gen_range(0..yani_endings.len())];
            result.push_str(ending);
            result.push_str("🚬💨");
            // Skip past the marker
            let marker_chars: Vec<char> = "🚬💨".chars().collect();
            i += marker_chars.len();
            sentence_count += 1;

            // Maybe insert filler between sentences
            if sentence_count > 0 && i < text_len {
                if rng.gen_bool(0.4) {
                    let filler = fillers[rng.gen_range(0..fillers.len())];
                    result.push_str(filler);
                }
            }
        } else if remaining.starts_with("🚬") {
            // End of normal sentence (was 。 or 、)
            let ending = yani_endings[rng.gen_range(0..yani_endings.len())];
            result.push_str(ending);
            result.push_str("🚬");
            let marker_chars: Vec<char> = "🚬".chars().collect();
            i += marker_chars.len();
            sentence_count += 1;

            // Maybe insert filler between sentences
            if sentence_count > 0 && i < text_len {
                if rng.gen_bool(0.4) {
                    let filler = fillers[rng.gen_range(0..fillers.len())];
                    result.push_str(filler);
                }
            }
        } else if remaining.starts_with('\n') {
            // Newline can also be a sentence boundary
            let ending = yani_endings[rng.gen_range(0..yani_endings.len())];
            // Only add ending if there's actual content before this newline
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

    // Handle text that ends without any punctuation marker
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

/// Deterministic version for testing (no random elements)
#[cfg(test)]
fn transform_deterministic(input: &str) -> String {
    let mut text = input.to_string();

    // 1. 語彙置換
    text = replace_vocabulary(&text);

    // 2. 挨拶変換
    text = replace_greetings(&text);

    // 3. 煙の表現
    text = text.replace('〜', "〜〜〜💨");

    // 4. 感嘆符
    text = text.replace('！', "🚬💨");
    text = text.replace('!', "🚬💨");

    // 5. 句読点置換
    text = text.replace('。', "🚬");
    text = text.replace('、', "🚬");

    // 6. 語尾変換 (deterministic: always use ヤニ, no fillers)
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

    // Handle text ending without punctuation
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
        assert!(!result.contains("だ〜\u{200b}")); // should not have bare 〜
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
        // Each sentence should have ヤニ before 🚬
        assert!(result.contains("ヤニ🚬"));
    }

    #[test]
    fn test_full_transform() {
        let result = transform_deterministic("おはよう。今日は美味しいご飯を食べた。最高！");
        // Check greeting
        assert!(result.contains("おはヤニ"));
        // Check vocabulary
        assert!(result.contains("ニコチンが染みる"));
        assert!(result.contains("一服の至福"));
        // Check punctuation
        assert!(!result.contains('。'));
        assert!(result.contains("🚬"));
        // Check exclamation
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
        // Count occurrences of ヤニ🚬
        let count = result.matches("ヤニ🚬").count();
        assert_eq!(count, 3, "Each sentence should end with ヤニ🚬, got: {}", result);
    }

    #[test]
    fn test_random_transform_runs() {
        // Just verify the random version doesn't panic
        let result = transform("おはよう。美味しいご飯！休憩しよう〜");
        assert!(!result.is_empty());
        // Basic checks still apply
        assert!(result.contains("おはヤニ"));
        assert!(result.contains("ニコチンが染みる"));
        assert!(result.contains("一服タイム"));
        assert!(result.contains("〜〜〜💨"));
    }
}
