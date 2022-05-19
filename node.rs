

struct Node {
    before: Node,
    after: Node,
    value: String
}

impl Node {
    fn new (bef: &Node, aft: &Node, val: &str ) -> Node {
        Node {
            before: bef,
            after: aft,
            value: val.to_string()
        }
    }
}