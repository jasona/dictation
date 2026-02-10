// Rule-based text cleanup (filler removal, capitalization, punctuation)

use regex::Regex;
use std::sync::LazyLock;

use super::TextCleaner;

/// Compiled regex patterns for filler word removal and text normalization.
/// Uses `LazyLock` for one-time compilation.
static RE_PURE_FILLERS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(um|uh|uhh|umm|hmm|erm|ah)\b").unwrap()
});

static RE_CONTEXTUAL_LIKE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(was|is|am|were|just|really)\s+like\b").unwrap()
});

static RE_YOU_KNOW_PROTECTED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(do|did|don't|doesn't|does)\s+you\s+know").unwrap()
});

static RE_YOU_KNOW: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\byou know\b").unwrap()
});

static RE_SO_SENTENCE_START: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?mi)(^\s*|[.!?]\s+)so\s+").unwrap()
});

static RE_CLAUSE_FILLERS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?mi)(^\s*|[.!?]\s+|,\s*)(basically|actually|i mean)\s+").unwrap()
});

static RE_MULTI_SPACES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r" {2,}").unwrap()
});

static RE_SENTENCE_START: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(^|[.!?]\s+)([a-z])").unwrap()
});

static RE_PRONOUN_I: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\bi\b").unwrap()
});

const PLACEHOLDER: &str = "\x00YKPROTECT\x00";

/// Rule-based text cleaner.
/// Applies a pipeline of regex transformations to remove filler words,
/// normalize whitespace, and fix capitalization.
pub struct RuleCleaner;

impl TextCleaner for RuleCleaner {
    fn clean(&self, text: &str) -> Result<String, String> {
        if text.is_empty() {
            return Ok(String::new());
        }

        let mut result = text.to_string();

        // 1. Remove pure fillers: um, uh, uhh, umm, hmm, erm, ah
        result = RE_PURE_FILLERS.replace_all(&result, "").to_string();

        // 2. Remove contextual "like" (was like, is like, etc.)
        //    Keep the prefix word, remove " like"
        result = RE_CONTEXTUAL_LIKE
            .replace_all(&result, |caps: &regex::Captures| {
                caps[1].to_string()
            })
            .to_string();

        // 3. Remove "you know" — protect "do you know", "did you know", etc.
        //    Replace protected forms with placeholder
        result = RE_YOU_KNOW_PROTECTED
            .replace_all(&result, |caps: &regex::Captures| {
                format!("{}{}", &caps[1], PLACEHOLDER)
            })
            .to_string();
        //    Remove remaining "you know"
        result = RE_YOU_KNOW.replace_all(&result, "").to_string();
        //    Restore protected forms
        result = result.replace(PLACEHOLDER, " you know");

        // 4. Remove "so" at sentence starts
        result = RE_SO_SENTENCE_START
            .replace_all(&result, |caps: &regex::Captures| {
                caps[1].to_string()
            })
            .to_string();

        // 5. Remove "basically"/"actually"/"I mean" at sentence/clause starts
        result = RE_CLAUSE_FILLERS
            .replace_all(&result, |caps: &regex::Captures| {
                caps[1].to_string()
            })
            .to_string();

        // 6. Normalize whitespace (collapse multiple spaces, trim)
        result = RE_MULTI_SPACES.replace_all(&result, " ").to_string();
        result = result.trim().to_string();

        // 7. Capitalize sentence starts
        result = RE_SENTENCE_START
            .replace_all(&result, |caps: &regex::Captures| {
                format!("{}{}", &caps[1], caps[2].to_uppercase())
            })
            .to_string();

        // 8. Capitalize pronoun "I"
        result = RE_PRONOUN_I.replace_all(&result, "I").to_string();

        // Final trim
        result = result.trim().to_string();

        Ok(result)
    }
}

#[cfg(test)]
mod tests;
