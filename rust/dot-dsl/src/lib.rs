pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;
    #[derive(Default, PartialEq, Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph::default()
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<HashMap<String, String>>();
            self
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == name)
        }
    }
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            use super::node::Node;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                left: Node,
                right: Node,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(left: &str, right: &str) -> Self {
                    Edge {
                        left: Node::new(left),
                        right: Node::new(right),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Default, PartialEq, Debug)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        ..Default::default()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect::<HashMap<String, String>>();
                    self
                }

                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(|n| n.as_ref())
                }
            }
        }
    }
}
