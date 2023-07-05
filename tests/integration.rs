use stack_queue::{Queue, Stack, StackError};

#[test]
fn test_stack() {
    let mut s: Stack<u32> = Stack::new();

    s.push(1);
    s.push(2);
    s.push(3);
    s.push(4);
    s.push(5);
    s.push(6);

    assert_eq!(s.size(), 6);

    assert_eq!(s.pop(), Ok(6));
    assert_eq!(s.pop(), Ok(5));
    assert_eq!(s.pop(), Ok(4));
    assert_eq!(s.pop(), Ok(3));
    assert_eq!(s.pop(), Ok(2));
    assert_eq!(s.pop(), Ok(1));
    assert_eq!(s.pop(), Err(StackError::EmptyStack));
}

#[test]
fn test_queue() {
    let mut q: Queue<i32> = Queue::new();

    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);

    assert_eq!(q.size(), 3);

    assert_eq!(q.dequeue(), Some(1));
    assert_eq!(q.dequeue(), Some(2));
    assert_eq!(q.dequeue(), Some(3));
    assert_eq!(q.dequeue(), None);

    assert_eq!(q.size(), 0);
}
