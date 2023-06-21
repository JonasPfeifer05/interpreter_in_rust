use interpreter::lexer::keyword_map;

#[test]
fn test_keywords() {
    let keywords = keyword_map();

    assert_eq!(keywords.len(), 10, "Keywords length do not match with the actual amount of keywords!")
}