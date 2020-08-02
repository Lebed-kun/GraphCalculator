use std::collections::HashMap;

#[allow(non_snake_case)]
struct Graph<'a> {
    verticies: HashMap<
        &'a str,
        u8
    >,
    adjacentMatrix: Vec<Vec<u8>>
}

#[allow(non_snake_case)]
impl<'a> Graph<'a> {
    pub fn new(verticiesCount: u8) -> Box<Graph<'a>> {
        let verticies = HashMap::new();

        let mut adjacentMatrix = Vec::with_capacity(verticiesCount as usize);

        for _ in 0..verticiesCount {
            adjacentMatrix.push(
                Vec::with_capacity(verticiesCount as usize)
            );
        }

        let graph = Box::new(Graph {
            verticies: verticies,
            adjacentMatrix: adjacentMatrix
        });

        return graph;
    }
}