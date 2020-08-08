extern crate graph_lib;

use graph_lib::graph::Graph;
    
    #[test]
    pub fn createsGraphWithVerticies() {
        let verticiesCount: u8 = 5;
        let graph: Box<Graph> = Box::new(
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
    
        graph.addVertex("()");
    
        graph.addEdge("abc", "123");
        graph.addEdge("123", "abc");
        graph.addEdge("def", "456");
        graph.addEdge("123", "()");
    
        let adjacentsOfAbc = graph.getAdjacents("abc");
        assert_eq!(
            adjacentsOfAbc[
                graph.getVertexId("123")
            ],
            1
        );
    
        let adjacentsOf123 = graph.getAdjacents("123");
        assert_eq!(
            adjacentsOf123[
                graph.getVertexId("abc")
            ],
            1
        );
        assert_eq!(
            adjacentsOf123[
                graph.getVertexId("()")
            ],
            1
        );
    
        let adjacentsOfDef = graph.getAdjacents("def");
        assert_eq!(
            adjacentsOfDef[
                graph.getVertexId("456")
            ],
            1
        );
    
        let adjacentsOf456 = graph.getAdjacents("456");
        assert_ne!(
            adjacentsOf456[
                graph.getVertexId("def")
            ],
            1
        );
    
        let adjacentsOfBracets = graph.getAdjacents("()");
        assert_ne!(
            adjacentsOfBracets[
                graph.getVertexId("123")
            ],
            1
        );
    } 
