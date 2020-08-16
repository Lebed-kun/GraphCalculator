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

    #[test]
    pub fn calculatesDistanceCorrectly() {
        let verticiesCount: u8 = 6;
        let mut graph: Box<Graph> = Box::new(
            Graph::new(verticiesCount)
        );

        graph.addVertex("a");
        graph.addVertex("b");
        graph.addVertex("c");
        graph.addVertex("1");
        graph.addVertex("2");
        graph.addVertex("3");

        graph.addEdge("a", "b");
        graph.addEdge("b", "c");
        graph.addEdge("b", "1");
        graph.addEdge("1", "a");
        graph.addEdge("1", "2");
        graph.addEdge("2", "c");
        graph.addEdge("2", "3");

        let realDistance1: i32 = graph.distance("a", "b");
        let expectedDistance1: i32 = 1;
        assert_eq!(
            realDistance1,
            expectedDistance1
        );

        let realDistance2: i32 = graph.distance("a", "c");
        let expectedDistance2: i32 = 2;
        assert_eq!(
            realDistance2,
            expectedDistance2
        );

        let realDistance3: i32 = graph.distance("a", "3");
        let expectedDistance3: i32 = 4;
        assert_eq!(
            realDistance3,
            expectedDistance3
        );

        let realDistance4: i32 = graph.distance("b", "3");
        let expectedDistance4: i32 = 3;
        assert_eq!(
            realDistance4,
            expectedDistance4
        );

        let realDistance5: i32 = graph.distance("b", "2");
        let expectedDistance5: i32 = 2;
        assert_eq!(
            realDistance5,
            expectedDistance5
        );

        let realDistance6: i32 = graph.distance("c", "1");
        let expectedDistance6: i32 = -1;
        assert_eq!(
            realDistance6,
            expectedDistance6
        );

        let realDistance7: i32 = graph.distance("3", "a");
        let expectedDistance7: i32 = -1;
        assert_eq!(
            realDistance7,
            expectedDistance7
        );
    } 
} 
