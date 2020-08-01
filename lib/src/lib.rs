use std::collections::HashMap;

#[allow(non_snake_case)]
struct Graph<'a> {
    verticies: HashMap<
        &'a str,
        u8
    >,
    adjacentMatrix: Vec<Vec<u8>>
}

impl<'a> Graph<'a> {
    fn new(
        verticies: &'a[&'a str],
        
    ) -> &Graph<'a> {
        
    }
}