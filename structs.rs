use regex::bytes::Regex;

enum Options{
    AFTER{lines: usize},
    BEFORE(usize),
    CONTEXT(usize),
    COUNT,
    IgnoreCase = 1,
    INVERT = 3,
    FIXED = 2,
    LineNum
}