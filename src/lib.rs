use pgrx::prelude::*;
use tiktoken_rs::{r50k_base, o200k_base, cl100k_base, p50k_base, p50k_edit};

pgrx::pg_module_magic!();

fn resolve_encoder_name(s: &str) -> &str {
    match s {
        "gpt-4o" | "gpt-4o-mini" | "gpt-4.1" | "gpt-4.1-mini" |
        "o1" | "o1-mini" | "o3" | "o3-mini" => "o200k_base",
        "gpt-3.5-turbo" | "gpt-4" |
        "text-embedding-3-large" | "text-embedding-3-small" |
        "text-embedding-ada-002" => "cl100k_base",
        "text-davinci-003" | "text-davinci-002" |
        "text-curie-001" | "text-babbage-001" | "text-ada-001" => "p50k_base",
        "code-davinci-edit-001" | "text-davinci-edit-001" => "p50k_edit",
        "gpt2" => "r50k_base",
        "o200k_base" | "cl100k_base" | "p50k_base" | "p50k_edit" | "r50k_base" => s,
        _ => s,
    }
}

fn encode_with_model(encoding_selector: &str, text: &str) -> Vec<u32> {
    let encoder_name = resolve_encoder_name(encoding_selector);
    let encoder = match encoder_name {
        "o200k_base" => o200k_base(),
        "cl100k_base" => cl100k_base(),
        "r50k_base" | "gpt2" => r50k_base(),
        "p50k_base" => p50k_base(),
        "p50k_edit" => p50k_edit(),
        _ => error!("'{encoding_selector}': unknown model or encoder"),
    }.unwrap();

    encoder.encode_with_special_tokens(text)
}

#[pg_extern]
fn tiktoken_encode(encoding_selector: &str, text: &str) -> Vec<i64> {
    encode_with_model(encoding_selector, text)
        .into_iter()
        .map(|x| x as i64)
        .collect()
}

#[pg_extern]
fn tiktoken_count(encoding_selector: &str, text: &str) -> i64 {
    encode_with_model(encoding_selector, text).len() as i64
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_tiktoken_encode() {
        let s = "This is a test         with a lot of spaces<|endoftext|>";
        assert_eq!(
            crate::tiktoken_encode("p50k_base", s),
            vec![1212, 318, 257, 1332, 50263, 351, 257, 1256, 286, 9029, 50256]
        );
        assert_eq!(
            crate::tiktoken_encode("p50k_edit", s),
            vec![1212, 318, 257, 1332, 50263, 351, 257, 1256, 286, 9029, 50256]
        );
        assert_eq!(
            crate::tiktoken_encode("r50k_base", s),
            vec![1212, 318, 257, 1332, 220, 220, 220, 220, 220, 220, 220, 220, 351, 257, 1256, 286, 9029, 50256]
        );
        assert_eq!(
            crate::tiktoken_encode("cl100k_base", s),
            vec![2028, 374, 264, 1296, 260, 449, 264, 2763, 315, 12908, 100257]
        );
        assert!(crate::tiktoken_encode("o200k_base", s).len() > 0);

        let s = "A long time ago in a galaxy far, far away";
        assert_eq!(
            crate::tiktoken_encode("text-davinci-002", s),
            crate::tiktoken_encode("p50k_base", s)
        );
        assert_eq!(
            crate::tiktoken_encode("gpt-3.5-turbo", s),
            crate::tiktoken_encode("cl100k_base", s)
        );
        assert_eq!(
            crate::tiktoken_encode("gpt2", s),
            crate::tiktoken_encode("r50k_base", s)
        );
        assert_eq!(
            crate::tiktoken_encode("code-davinci-edit-001", s),
            crate::tiktoken_encode("p50k_edit", s)
        );
        assert_eq!(
            crate::tiktoken_encode("gpt-4o", s),
            crate::tiktoken_encode("o200k_base", s)
        );
        assert_eq!(
            crate::tiktoken_encode("gpt-4o-mini", s),
            crate::tiktoken_encode("o200k_base", s)
        );
    }

    #[pg_test]
    fn test_tiktoken_count() {
        let s = "A long time ago in a galaxy far, far away";
        assert_eq!(crate::tiktoken_count("p50k_base", s), 11);
        assert_eq!(crate::tiktoken_count("cl100k_base", s), 11);
        assert_eq!(crate::tiktoken_count("o200k_base", s), 11);
        assert_eq!(crate::tiktoken_count("r50k_base", s), 11);
        assert_eq!(crate::tiktoken_count("p50k_edit", s), 11);
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}
    pub fn postgresql_conf_options() -> Vec<&'static str> { vec![] }
}
