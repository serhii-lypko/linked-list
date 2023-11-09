/*
  TODO: implement LRU cache?

  The typical implementation of an LRU cache uses a combination of a hash map and
  a doubly-linked list. The hash map allows for quick access (insertion, deletion, lookup)
  to the elements in the cache by key, while the doubly-linked list maintains the order
  of elements by their recency of use. When a new element is accessed or added, it is moved
  to the front of the list. If the cache is full and a new element needs to be added,
  the element at the back of the list (the least recently used) is removed.

  TODO:

  - Doubly linked list?

  - ⭐️ Iterators: Implementing an iterator for a linked list, which involves
  understanding Rust's lifetime semantics and the borrow checker.

  - Concurrency: Making your linked list thread-safe using Rust's concurrency primitives, like Mutex or Arc.
*/

// TODO: trait with a set of rules of LinkedList?

// TODO: tests

use std::fmt::Debug;
use std::ops::Deref;

/// Definitely not the most correct and efficient implementation of Linked List

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T>
where
    T: Clone + Debug,
{
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn peek(&self) {
        todo!();
    }

    fn clear(&mut self) {
        self.head = None;
    }

    fn len(&self) -> usize {
        self.head
            .as_ref()
            .map_or(0, |head_node| head_node.count_len())
    }

    fn print(&self) {
        println!("-- -- Print linked list -- --");

        if let Some(head_node) = &self.head {
            head_node.print();
        }

        println!("-- -- -- -- -- -- -- -- -- --");
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|boxed_node| {
            self.head = boxed_node.next;
            boxed_node.value
        })
    }

    fn push(&mut self, value: T) {
        let new_node = Node {
            next: self.head.take(),
            value,
        };

        self.head = Some(Box::new(new_node));
    }
}

#[derive(Debug, Clone)]
struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T,
}

impl<T> Node<T>
where
    T: Clone + Debug,
{
    fn print_recur(&self, counter: u16) {
        println!("Node #{} with value: {:?}", counter, self.value);

        if let Some(next) = &self.next {
            next.print_recur(counter + 1);
        }
    }

    pub fn print(&self) {
        self.print_recur(0);
    }

    pub fn count_len(&self) -> usize {
        self.next
            .as_ref()
            .map_or(1, |next_node| next_node.count_len() + 1)
    }
}

#[derive(Clone, Debug)]
struct Figure {
    area: usize,
}

fn main() {
    let mut ll: LinkedList<Figure> = LinkedList::new();

    ll.push(Figure { area: 0 });
    ll.push(Figure { area: 10 });
    ll.push(Figure { area: 20 });

    ll.print();

    println!("-- now pop --");
    let head_popped = ll.pop();
    // dbg!(head_popped);

    ll.print();

    // dbg!(ll);

    // let len = ll.len();
    // dbg!(len);
}
