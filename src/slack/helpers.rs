use regex::Regex;

pub fn extract_links(text: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<([^|>]*)").unwrap();
    }

    RE.captures_iter(text)
        .filter_map(|cap| cap.get(1))
        .map(|cap| cap.as_str().to_string())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_links() {
        assert_eq!(
            extract_links("<http://github.com|github.com>")[0],
            "http://github.com"
        );
        assert_eq!(
            extract_links("<http://github.com/vigetlabs/code_review_bot/pulls/30>")[0],
            "http://github.com/vigetlabs/code_review_bot/pulls/30"
        );

        let text =
      "This is a link <http://github.com> and this is one too <http://example.com|example.com>";
        assert_eq!(extract_links(text)[0], "http://github.com");
        assert_eq!(extract_links(text)[1], "http://example.com");
    }

}
