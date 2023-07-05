#[derive(PartialEq, Debug)]
struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
}

#[derive(Debug, PartialEq)]
pub enum StackError {
    EmptyStack,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { top: None }
    }

    pub fn push(&mut self, item: T) {
        let new_node = Box::new(Node {
            item,
            next: self.top.take(),
        });

        self.top = Some(new_node);
    }

    pub fn pop(&mut self) -> Result<T, StackError> {
        if self.is_empty() {
            return Err(StackError::EmptyStack);
        }

        self.top
            .take()
            .map(|node| {
                self.top = node.next;
                node.item
            })
            .ok_or(StackError::EmptyStack)
    }

    pub fn is_empty(&self) -> bool {
        self.top.is_none()
    }

    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.item)
    }

    pub fn size(&self) -> usize {
        let mut count = 0;
        let mut current = &self.top;

        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }

        count
    }

    pub fn iter(&self) -> StackItereator<T> {
        StackItereator::new(&self.top)
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct StackItereator<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}

impl<'a, T> StackItereator<'a, T> {
    fn new(top: &'a Option<Box<Node<T>>>) -> Self {
        Self { current: top }
    }
}

// Implement Iterator trait for StackIterator.
impl<'a, T> Iterator for StackItereator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                self.current = &node.next;
                Some(&node.item)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_empty() {
        let mut stack: Stack<i32> = Stack::new();

        assert_eq!(true, stack.is_empty());

        let _ = stack.push(2);
        assert_eq!(false, stack.is_empty());
    }

    #[test]
    fn test_push_and_pop() {
        let mut stack: Stack<i32> = Stack::new();

        let range = 0..30;

        for i in range.clone() {
            let _ = stack.push(i);
        }

        assert_eq!(30, stack.size());

        for i in range.rev() {
            assert_eq!(Ok(i), stack.pop())
        }
        assert_eq!(0, stack.size());
        assert_eq!(Err(StackError::EmptyStack), stack.pop());
    }

    #[test]
    fn test_iterator() {
        let mut stack: Stack<i32> = Stack::new();

        let range = 0..10;

        for i in range.clone() {
            let _ = stack.push(i);
        }

        assert_eq!(stack.size(), 10);

        let mut iter = stack.iter();

        for j in range.rev() {
            assert_eq!(iter.next(), Some(&j));
        }

        assert_eq!(iter.next(), None);
        // We're only iterating, not popping the item.
        assert_eq!(stack.size(), 10);
    }
}
