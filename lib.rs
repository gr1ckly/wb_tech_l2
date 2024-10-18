mod structs;

use std::collections::hash_map::Keys;
use std::fs::File;
struct Config{
    keys: Vec<String>,
    temple: String,
    file: File,
}

