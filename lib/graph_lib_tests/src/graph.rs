#[cfg(test)]
#[allow(non_snake_case)]
pub mod graph {
    use graph_lib::graph::graph::Graph;

    #[test]
    pub fn createsGraphWithVerticies() {
        let verticiesCount: u8 = 5;
        let mut graph: Box<Graph> = Box::new(
            Graph::new(verticiesCount)
        );
    
        graph.addVertex("abc");
        graph.addVertex("def");
        graph.addVertex("123");
        graph.addVertex("456");
    
        assert_eq!(
            graph.getVertexId("abc"),
            0
        );
    
        assert_eq!(
            graph.getVertexId("def"),
            1
        );
    
        assert_eq!(
            graph.getVertexId("123"),
            2
        );
    
        assert_eq!(
            graph.getVertexId("456"),
            3
        );
    }

    #[test]
    pub fn createsGraphWithEdges() {
        let verticiesCount: u8 = 5;
        let mut graph: Box<Graph> = Box::new(
            Graph::new(verticiesCount)
        );
    
        graph.addVertex("abc");
        graph.addVertex("def");
        graph.addVertex("123");
        graph.addVertex("456");
        graph.addVertex("()");
    
        graph.addEdge("abc", "123");
        graph.addEdge("123", "abc");
        graph.addEdge("def", "456");
        graph.addEdge("123", "()");
    
        let adjacentsOfAbc = graph.getAdjacents("abc");
        assert_eq!(
            adjacentsOfAbc[
                graph.getVertexId("123") as usize
            ],
            1
        );
    
        let adjacentsOf123 = graph.getAdjacents("123");
        assert_eq!(
            adjacentsOf123[
                graph.getVertexId("abc") as usize
            ],
            1
        );
        assert_eq!(
            adjacentsOf123[
                graph.getVertexId("()") as usize
            ],
            1
        );
    
        let adjacentsOfDef = graph.getAdjacents("def");
        assert_eq!(
            adjacentsOfDef[
                graph.getVertexId("456") as usize
            ],
            1
        );
    
        let adjacentsOf456 = graph.getAdjacents("456");
        assert_ne!(
            adjacentsOf456[
                graph.getVertexId("def") as usize
            ],
            1
        );
    
        let adjacentsOfBracets = graph.getAdjacents("()");
        assert_ne!(
            adjacentsOfBracets[
                graph.getVertexId("123") as usize
            ],
            1
        );
    }
} 
