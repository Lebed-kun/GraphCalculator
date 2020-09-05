pub mod graph {
    use std::collections::HashMap;

    use crate::math::math;
    
    #[allow(non_snake_case)]
    pub struct Graph<'a> {
        verticies: HashMap<
            &'a str,
            u8
        >,
        adjacentMatrix: Vec<Vec<u8>>
    }

    #[allow(non_snake_case)]
    #[no_mangle]
    impl<'a> Graph<'a> {
        pub extern fn new(verticiesCount: u8) -> Graph<'a> {
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

        pub extern fn addVertex(&mut self, vertex: &'a str) {
            self.verticies.insert(
                vertex,
                self.verticies.len() as u8
            );
        }

        pub extern fn getVertexId(&self, vertex: &'a str) -> u8 {
            return self.verticies[vertex];
        }

        pub extern fn getAdjacents(&self, vertex: &'a str) -> &Vec<u8> {
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

        pub extern fn addEdge(&mut self, source: &'a str, destination: &'a str) {
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

        fn createVisitedList(&self) -> Box<Vec<u8>> {
            let verticiesCount: usize = self.verticies.len();
            let mut result: Box<Vec<u8>> = Box::new(
                Vec::with_capacity(verticiesCount)
            );

            for _ in 0..verticiesCount {
                result.push(0);
            }

            return result;
        }

        fn distanceById(&self, source: u8, destination: u8) -> i32 {
            let mut result: i32 = -1;

            if source >= self.verticies.len() as u8 {
                return result;
            }

            if destination >= self.verticies.len() as u8 {
                return result;
            }

            let mut currVertexId: u8 = source;
            let mut currLength: i32 = 0;
            let mut visited: Box<Vec<u8>> = self.createVisitedList();
            let mut stack: Box<Vec<(u8, i32)>> = Box::new(
                Vec::new()
            );

            loop {
                // If there's neighbors to lookup
                if stack.len() > 0 {
                    let tuple = stack.pop().unwrap();
                    currVertexId = tuple.0;
                    currLength = tuple.1;
                }

                // If destination vertex is reached
                if currVertexId == destination {
                    result = math::positiveMin(currLength, result);
                }

                // Mark current vertex as visited
                visited[currVertexId as usize] = 1;

                // Push unvisited neighbors to stack
                let neighbors: &Vec<u8> = &self.adjacentMatrix[currVertexId as usize];
                let neightborsCount: usize = neighbors.len();
                for id in 0..neightborsCount {
                    if visited[id] == 0 && neighbors[id] == 1 {
                        stack.push((id as u8, currLength + 1));
                    }
                }

                // If stack is still empty 
                // then there's no verticies to lookup
                if stack.len() == 0 {
                    return result;
                }
            }
        }

        pub extern fn distance(&self, source: &'a str, destination: &'a str) -> i32 {
            let mut result: i32 = -1;

            if !self.verticies.contains_key(source) {
                return result;
            }

            if !self.verticies.contains_key(destination) {
                return result;
            }

            let sourceId: u8 = self.verticies[source];
            let destinationId: u8 = self.verticies[destination];

            return self.distanceById(sourceId, destinationId);
        }

        fn createVerticiesMatrix(&self) -> Box<Vec<Vec<i32>>> {
            let mut result: Box<Vec<Vec<i32>>> = Box::new(
                Vec::with_capacity(self.verticies.len())
            );

            for i in 0..self.verticies.len() {
                result.push(Vec::with_capacity(self.verticies.len()));

                for _ in 0..self.verticies.len() {
                    result[i].push(0);
                }
            }

            return result;
        }

        pub extern fn distanceMatrix(&self) -> Box<Vec<Vec<i32>>> {
            let mut result: Box<Vec<Vec<i32>>> = self.createVerticiesMatrix();

            for sourceId in 0..self.verticies.len() {
                for destinationId in 0..self.verticies.len() {
                    let distance = self.distanceById(sourceId as u8, destinationId as u8);
                    result[sourceId][destinationId] = distance;
                }
            }

            return result;
        }
    }
}