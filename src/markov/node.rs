use std::collections::HashMap;

struct Node<T>
    where T: Chainable
{
    children: HashMap<T, f32>,
    parent: &Chain,
}

impl Node {
    pub fn new(parent: &Chain) -> Node {
        Node {
            children: HashMap::new(),
            parent: parent,
        }
    }

    pub fn next(&self) -> &T {
        if self.parent.rng.gen::<f32>() < self.parent.mutation {}
    }
}
