use super::gaddag::gaddag;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::cell::RefCell;

use std::collections::HashMap;
use std::rc::{Rc, Weak};

use std::str::Chars;

struct Node {
    parent : Option<Weak<RefCell<Node>>>,
    children : HashMap<char, NodeRef>,
    letter : Option<char>,
    terminal : bool
}

struct NodeRef (Rc<RefCell<Node>>);

pub struct Dico {
    first_node : NodeRef,
}

impl Dico {
    pub fn new(filename : &str) -> Dico {
        let file = File::open(filename);
        let reader = BufReader::new(file.unwrap());

        let first_node = NodeRef::new(None, None, false);
        let dico = Dico {
            first_node : (first_node),
        };

        for line in reader.lines() {
            let gaddaged = gaddag(line.unwrap());
            for gad in gaddaged {
                dico.add_word(gad.as_str());
            }
        }
        dico
    }

    pub fn exists(&self, word : &str) -> bool {
        self.first_node.exists(word.chars())
    }

    fn add_word(&self, word : &str) {
        self.first_node.add_nexts(word.chars());
    }
}

impl Node {
    fn new(letter : Option<char>, parent : Option<NodeRef>, terminal: bool) -> Node {
        let weak_parent : Option<Weak<RefCell<Node>>>;
        if let Some(x) = parent {
            weak_parent = Some(Rc::downgrade(&x.0));
        }
        else {
            weak_parent = None;
        }
        Node {
            parent : weak_parent,
            children : HashMap::new(),
            letter,
            terminal,
        }
    }

}

impl NodeRef {
    fn new(letter : Option<char>, parent : Option<NodeRef>,
            terminal : bool) -> NodeRef {
        return NodeRef {
            0 : Rc::new(RefCell::new(Node::new(letter, parent, terminal))),
        }
    }

    fn copy(&self) -> Self {
        let cloned = Rc::clone(&self.0);
        return Self {
            0 : cloned,
        };
    }

    fn add_nexts(&self, mut chars: Chars) {
        let first_letter = chars.next();
        if let None = first_letter {
            self.0.borrow_mut().terminal = true;
            return;
        }
        let first_letter = first_letter.unwrap();
        let mut current_node = self.0.borrow_mut();
        let next_node = current_node
                .children.entry(first_letter)
                .or_insert(NodeRef::new(Some(first_letter), Some(self.copy()), false));
        next_node.add_nexts(chars);
    }

    fn exists(&self, mut word : Chars) -> bool {
        let next = word.next();
        if let None = next {
            return self.0.borrow().terminal;
        }
        let next = next.unwrap();
        let current_node = self.0.borrow();
        let child : Option<&NodeRef> = current_node.children.get(&next);
        match child {
            None => false,
            Some(x) => return x.exists(word)
        }
    }
}
