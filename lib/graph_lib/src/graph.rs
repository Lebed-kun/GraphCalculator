pub mod graph {
    use std::collections::HashMap;
    
    #[allow(non_snake_case)]
    pub struct Graph<'a> {
        verticies: HashMap<
            &'a str,
            u8
        >,
        adjacentMatrix: Vec<Vec<u8>>
    }

    #[allow(non_snake_case)]
    impl<'a> Graph<'a> {
        pub fn new(verticiesCount: u8) -> Graph<'a> {
            let verticies = HashMap::with_capacity(verticiesCount as usize);

            let mut adjacentMatrix = Vec::with_capacity(verticiesCount as usize);

            for i in 0..verticiesCount {
                adjacentMatrix.push(
                    Vec::with_capacity(verticiesCount as usize)
                );

                for _ in 0..verticiesCount {
                    adjacentMatrix[i as usize].push(0);
                }
            }

            let graph = Graph {
                verticies: verticies,
                adjacentMatrix: adjacentMatrix
            };

            return graph;
        }

        pub fn addVertex(&mut self, vertex: &'a str) {
            self.verticies.insert(
                vertex,
                self.verticies.len() as u8
            );
        }

        pub fn getVertexId(&self, vertex: &'a str) -> u8 {
            return self.verticies[vertex];
        }

        pub fn getAdjacents(&self, vertex: &'a str) -> &Vec<u8> {
            let vertexId = self.verticies[vertex];
            return &self.adjacentMatrix[vertexId as usize];
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
                    self.adjacentMatrix[id as usize].push(0);
                    
                    self.adjacentMatrix[i].insert(
                        id as usize,
                        0
                    )
                }
            }
        }

        pub fn addEdge(&mut self, source: &'a str, destination: &'a str) {
            self.padVertex(source);
            self.padVertex(destination);

            let sourceId: u8 = self.verticies[source];
            let destinationId: u8 = self.verticies[destination];

            self.padAdjacents(sourceId);
            self.padAdjacents(destinationId);

            self.adjacentMatrix[
                sourceId as usize
            ][
                destinationId as usize
            ] = 1;
        }

        /*
        pub fn distance(&self, source: &'a str, destination: &'a str) {
            // TODO
        } */
    }
}