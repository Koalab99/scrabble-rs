use super::gaddag::gaddag;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::cell::RefCell;

use std::collections::HashMap;
use std::rc::{Rc, Weak};

use std::str::Chars;

/// A Node/Branch/Graph used by `Dico`.
struct Node {
    /// Reference to its parent
    parent : Option<Weak<RefCell<Node>>>,
    /// HashMap of its children
    children : HashMap<char, NodeRef>,
    /// Its letter
    letter : Option<char>,
    /// Whether its terminal
    terminal : bool
}

/// A shortcut.
struct NodeRef (Rc<RefCell<Node>>);

/// An interface to the graph of `Node`.
pub struct Dico {
    /// The only `Node` with no parent and no letter.
    first_node : NodeRef,
}

impl Dico {
    /// Generate a new dictionnary from `filename`
    ///
    /// Panic:
    /// * The file cannot be opened (Maybe it does not exist)
    pub fn new(filename : &str) -> Dico {
        let file = File::open(filename);
        let reader = BufReader::new(file.unwrap());

        let first_node = NodeRef::new(None, None, false);
        let dico = Dico {
            first_node : (first_node),
        };

        for line in reader.lines() {
            dico.add_word(line.unwrap().as_str());
        }
        dico
    }

    /// Tells if the word is present in the dico.
    pub fn exists(&self, word : &str) -> bool {
        self.first_node.exists(word.chars())
    }

    /// Add `word` to the dico
    fn add_word(&self, word : &str) {
        self.first_node.add_nexts(word.chars());
    }
}

impl Node {
    /// Create a new node with an optional reference to its parent.
    ///
    /// # Arguments
    /// * `letter` - The letter of the node.
    /// * `parent` - A reference to its parent NodeRef
    /// * `terminal` - whether this node is terminal or not
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
    /// Create a new NodeRef
    ///
    /// # Arguments
    /// Same as Node's
    fn new(letter : Option<char>, parent : Option<NodeRef>,
            terminal : bool) -> NodeRef {
        return NodeRef {
            0 : Rc::new(RefCell::new(Node::new(letter, parent, terminal))),
        }
    }

    /// Get copy
    ///
    /// This is a special copy, NodeRef uses Rc<RefCell<Node>>, so the data
    /// in the copy points to the same address and is mutable. Use with care.
    fn copy(&self) -> Self {
        let cloned = Rc::clone(&self.0);
        return Self {
            0 : cloned,
        };
    }

    /// Add a child to the children list
    ///
    /// It will add a child with the first element of `chars` if needed and
    /// pass the remaining of `chars` of the appropriate child.
    ///
    /// # Argument
    /// * `chars` - A char iterator
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

    /// Whether there is a terminal node for this word
    ///
    /// # Argument
    /// * `chars` - A char iterator
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
