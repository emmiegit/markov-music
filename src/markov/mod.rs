pub use iter::InfiniteIterator;
pub use traits::Chainable;
use node::Node;
use rand::Rng;
use std::collection::HashMap;

mod node;
mod iter;
mod traits;

pub struct Chain<T>
    where T: Chainable
{
    nodes: HashMap<T, Node<T>>,
    state: &T,
    mutation: f32,
    rng: Rng,
}

impl<T> Chain<T>
    where T: Chainable
{
    pub fn new(initial: T, mutation: f32) {
        let mut chain = Chain {
            nodes: HashMap::new(),
            state: &initial,
            mutation: mutation,
            rng: Rng::thread_rng(),
        };

        chain.nodes.insert(initial, Node::new(&chain));
    }

    pub fn iter(&self) -> InfiniteIterator {}

    pub fn gen(&mut self) -> T {}

    // TODO work on API/operations
}
