pub struct InfiniteIterator {
}

impl<T> Iterator for InfiniteIterator<T> where T: Chainable {
    fn next(&mut self) -> Option<Self::Item> {
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
         // <usize>::max_value()
        (usize::MAX, None)
    }
}

