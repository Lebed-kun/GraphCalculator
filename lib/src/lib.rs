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

    pub fn addVertex(&mut self, vertex: &'a str) {
        self.verticies.insert(
            vertex,
            self.verticies.len() as u8
        );
    }

    fn padVertex(&mut self, vertex: &'a str) {
        let containsVertex = self.verticies.contains_key(vertex);
        if !containsVertex {
            self.addVertex(vertex);
        }
    }

    fn padAdjacents(&mut self, id: u8) {
        let verticiesCount = self.adjacentMatrix.len();
        if id >= verticiesCount as u8 {
            self.adjacentMatrix.insert(
                id as usize,
                Vec::with_capacity(verticiesCount)
            );

            for i in 0..self.adjacentMatrix.len() {
                self.adjacentMatrix[i].insert(
                    verticiesCount,
                    0
                )
            }
        }
    }

    pub fn addEdge(&mut self, source: &'a str, destination: &'a str) {
        self.padVertex(source);
        self.padVertex(destination);

        let mut sourceId: u8 = 0;
        match self.verticies.get(&source) {
            Some(&source) => {
                sourceId = source
            },
            _ => ()
        }

        let mut destinationId: u8 = 0;
        match self.verticies.get(&destination) {
            Some(&destination) => {
                destinationId = destination
            },
            _ => ()
        }

        self.padAdjacents(sourceId);
        self.padAdjacents(destinationId);

        self.adjacentMatrix[
            sourceId as usize
        ][
            destinationId as usize
        ] = 1;
    }
}