#[derive(Debug, PartialEq)]
struct Node<N> {
    data: N,
    next: Option<Box<Node<N>>>,
}

impl<N> Node<N> {
    fn new(data: N) -> Self {
        Self { data, next: None }
    }
}

pub struct Queue<Q> {
    head: Option<Box<Node<Q>>>,
    tail: *mut Node<Q>,
}

impl<Q> Queue<Q> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: std::ptr::null_mut(),
        }
    }

    pub fn enqueue(&mut self, data: Q) {
        let new_node = Box::new(Node::new(data));
        let raw_node = Box::into_raw(new_node);

        unsafe {
            if !self.tail.is_null() {
                (*self.tail).next = Some(Box::from_raw(raw_node));
            } else {
                // Tail is only Null when Queue is empty.
                self.head = Some(Box::from_raw(raw_node));
            }
            self.tail = raw_node;
        }
    }

    pub fn dequeue(&mut self) -> Option<Q> {
        self.head.take().map(|current_head| {
            let current_head_node = *current_head;
            self.head = current_head_node.next;

            if self.head.is_none() {
                self.tail = std::ptr::null_mut()
            }

            current_head_node.data
        })
    }

    pub fn size(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;

        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }

    pub fn iter(&self) -> QueueIterator<Q> {
        QueueIterator {
            current: &self.head,
        }
    }
}

impl<Q> Default for Queue<Q> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct QueueIterator<'a, Q> {
    current: &'a Option<Box<Node<Q>>>,
}

impl<'a, Q> Iterator for QueueIterator<'a, Q> {
    type Item = &'a Q;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(boxed_node) => {
                self.current = &boxed_node.next;
                Some(&boxed_node.data)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_queue() {
        let q: Queue<i32> = Queue::new();

        assert_eq!(None, q.head);
    }

    #[test]
    fn test_enqueue_dequeue() {
        let mut q: Queue<i32> = Queue::new();

        let range = 0..30;

        for i in range.clone() {
            q.enqueue(i);
        }

        assert_eq!(30, q.size());

        for i in range {
            assert_eq!(Some(i), q.dequeue())
        }
        assert_eq!(0, q.size());
        assert_eq!(None, q.dequeue());
    }

    #[test]
    fn test_iterator() {
        let mut q: Queue<i32> = Queue::new();

        let range = 0..10;
        for i in range.clone() {
            q.enqueue(i);
        }

        assert_eq!(q.size(), range.clone().len());

        let mut iter = q.iter();

        for j in range {
            assert_eq!(iter.next(), Some(&j));
        }

        assert_eq!(iter.next(), None);
    }
}
