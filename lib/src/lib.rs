use std::collections::HashMap;
use std::collections::HashSet;

#[allow(non_snake_case)]
struct Graph<'a> {
    adjacentVerticies: &'a HashMap<
        &'a str,
        &'a HashSet<&'a str>
    >
}

impl<'a> Graph<'a> {
    fn new() -> &Graph<'a> {
        
    }
}