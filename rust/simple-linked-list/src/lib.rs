#[derive(Default)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Self { data, next }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut curr_node = &self.head;
        while let Some(node) = curr_node {
            count += 1;
            curr_node = &node.next;
        }
        count
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node::new(element, self.head.take())));
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(mut curr_head) = self.head.take() {
            self.head = curr_head.next.take();
            Some(curr_head.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(curr_head) = &self.head {
            Some(&curr_head.data)
        } else {
            None
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut list = Self::new();
        let mut curr_node = &self.head;
        while let Some(node) = curr_node {
            list.push(node.data.clone());
            curr_node = &node.next;
        }
        list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut list = Self::new();
        for element in item {
            list.push(element.clone());
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut list = self;
        let mut vec = Vec::new();
        while let Some(data) = list.pop() {
            vec.push(data);
        }
        vec.reverse();
        vec
    }
}
