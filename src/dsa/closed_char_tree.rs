//! Closed Char Tree is like Char Tree, but all possible values are predifined as an enum.
//!
//! ```
//! use crate::closed_char_tree::{ClosedTree, Vehicle, CarType};
//!
//! let mut tree = ClosedTree::new();
//! tree.insert("ABC", &Vehicle::Car(CarType::Nissan));
//! tree.insert("ABCD", &Vehicle::Car(CarType::Infiniti));
//! let result = tree.get("ABC").unwrap();
//! println!("ABC is a WMI for {} ", result);
//! assert_eq!(result, &Vehicle::Car(CarType::Nissan));
//! tree.deep_delete("ABC");
//! assert_eq!(tree.get("ABC"), None);
//! ```

use core::fmt;

#[derive(Debug, PartialEq)]
pub enum CarType {
    Audi,
    Bmw,
    Nissan,
    Infiniti,
    Mercedes,
}

#[derive(Debug, PartialEq)]
pub enum MotorcycleType {
    HarleyDavidson,
    Suzuki,
}

#[derive(Debug, PartialEq)]
pub enum TruckType {
    Iveco,
    Scania,
    Mercedes,
}

#[derive(Debug, PartialEq)]
pub enum Vehicle {
    Car(CarType),
    Motorcycle(MotorcycleType),
    Truck(TruckType),
}

impl fmt::Display for Vehicle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Vehicle::Car(CarType::Audi) => write!(f, "Audi"),
            Vehicle::Car(CarType::Bmw) => write!(f, "BMW"),
            Vehicle::Car(CarType::Infiniti) => write!(f, "Infiniti"),
            Vehicle::Car(CarType::Nissan) => write!(f, "Nissan"),
            Vehicle::Car(CarType::Mercedes) => write!(f, "Mercedes-Benz"),
            Vehicle::Motorcycle(MotorcycleType::HarleyDavidson) => write!(f, "Harley-Davidson"),
            Vehicle::Motorcycle(MotorcycleType::Suzuki) => write!(f, "Suzuki"),
            Vehicle::Truck(TruckType::Iveco) => write!(f, "Iveco"),
            Vehicle::Truck(_) => write!(f, "Some other truck"),
        }
    }
}

#[derive(Debug)]
struct Node {
    name: char,
    value: Option<&'static Vehicle>,
    children: Vec<Box<Node>>,
}

impl Node {
    fn new(name: char) -> Self {
        println!("creating new node: {}", &name);
        Node {
            name: name,
            value: None,
            children: Vec::new(),
        }
    }

    fn get_child_ref(&self, name: char) -> Option<&Box<Node>> {
        self.children.iter().find(|node| node.name == name)
    }

    fn get_child_mut(&mut self, name: char) -> Option<&mut Box<Node>> {
        self.children.iter_mut().find(|node| node.name == name)
    }
}

/// The Tree struct allows you to store &str values on a provided char path;
/// Use insert(path: &str, value: &str) to insert value and
/// get(path: &str) to retireve it.
pub struct ClosedTree {
    root: Vec<Box<Node>>,
}

impl ClosedTree {
    pub fn new() -> Self {
        ClosedTree { root: Vec::new() }
    }

    fn insert_recursive(mut path: &str, value: &'static Vehicle, mut current_node: &mut Box<Node>) {
        if path.is_empty() {
            current_node.value = Some(value);
            return;
        }
        let first_char = path.chars().next().unwrap();
        path = &path[1..];
        if let Some(child) = current_node.get_child_mut(first_char) {
            Self::insert_recursive(path, value, child)
        } else {
            current_node.children.push(Box::new(Node::new(first_char)));
            Self::insert_recursive(path, value, current_node.children.last_mut().unwrap())
        }
    }

    /// Inserts given valia to a given path.
    pub fn insert(&mut self, mut path: &str, value: &'static Vehicle) {
        if path.is_empty() {
            return;
        }

        let first_char = path.chars().next().unwrap();
        path = &path[1..];
        if self.root.is_empty() {
            let new_node = Box::new(Node::new(first_char));
            self.root.push(new_node);
            Self::insert_recursive(path, value, self.root.iter_mut().last().unwrap());
            return;
        }

        if let Some(current_node) = self.root.iter_mut().find(|n| n.name == first_char) {
            Self::insert_recursive(path, value, current_node);
        } else {
            let new_node = Box::new(Node::new(first_char));
            self.root.push(new_node);
            Self::insert_recursive(path, value, self.root.iter_mut().last().unwrap());
        }
    }

    /// This method gets a value from a given path.
    pub fn get(&self, mut path: &str) -> Option<&'static Vehicle> {
        if self.root.is_empty() {
            return None;
        }
        let first_char = path.chars().next().unwrap();
        path = &path[1..];
        let mut current_node = self.root.iter().find(|&n| n.name == first_char)?;
        while !path.is_empty() {
            let first_char = path.chars().next().unwrap();
            path = &path[1..];
            if let Some(child) = current_node.get_child_ref(first_char) {
                current_node = child;
            };
        }
        current_node.value
    }

    /// This a legacy shallow delete method, use deep_delete() instead.
    pub fn shallow_delete(&mut self, mut path: &str) {
        if self.root.is_empty() || path.is_empty() {
            return;
        }
        let first_char = path.chars().next().unwrap();
        path = &path[1..];
        let mut current_node = match self.root.iter_mut().find(|n| n.name == first_char) {
            Some(node) => node,
            None => {
                return;
            }
        };
        while !path.is_empty() {
            let first_char = path.chars().next().unwrap();
            path = &path[1..];
            current_node = match current_node.get_child_mut(first_char) {
                Some(node) => node,
                None => {
                    return;
                }
            };
        }
        current_node.value = None;
    }

    /// This is the main method for deletions. It deletes not just values, but not used nodes as well.
    pub fn deep_delete(&mut self, mut path: &str) {
        if path.is_empty() {
            return;
        }
        // Start deletion from the root nodes
        let first_char = path.chars().next().unwrap();
        path = &path[1..];
        if let Some(node) = self.root.iter_mut().find(|n| n.name == first_char) {
            Self::deep_delete_recursive(node, path);
        }
    }

    fn deep_delete_recursive(node: &mut Box<Node>, mut path: &str) -> bool {
        if path.is_empty() {
            node.value = None;
            return node.children.is_empty();
        }

        let first_char = path.chars().next().unwrap();
        path = &path[1..];

        if let Some(next) = node.get_child_mut(first_char) {
            if Self::deep_delete_recursive(next, path) {
                // If the child node is no longer needed (returned true), remove it
                let pos = node
                    .children
                    .iter()
                    .position(|n| n.name == first_char)
                    .unwrap();
                node.children.remove(pos);
            }

            // If node has no value and no children, it can be deleted
            return node.value.is_none() && node.children.is_empty();
        }

        false // Node with the specified path was not found
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_node() {
        let mut node = Node::new('a');
        node.children.push(Box::new(Node::new('b')));
        assert_eq!(node.get_child_mut('b').unwrap().name, 'b');
    }

    #[test]
    fn test_tree() {
        let mut tree = ClosedTree::new();
        tree.insert("", &Vehicle::Car(CarType::Audi));

        tree.insert("abc", &Vehicle::Car(CarType::Audi));
        tree.insert("ab", &Vehicle::Motorcycle(MotorcycleType::Suzuki));
        tree.insert("abc", &Vehicle::Car(CarType::Bmw));
        tree.insert("abcde", &Vehicle::Truck(TruckType::Iveco));
        tree.insert("aba", &Vehicle::Car(CarType::Mercedes));
        tree.insert("edc", &Vehicle::Car(CarType::Infiniti));
        tree.insert("edcb", &Vehicle::Car(CarType::Nissan));

        assert_eq!(tree.get("a"), None);
        assert_eq!(
            tree.get("ab").unwrap(),
            &Vehicle::Motorcycle(MotorcycleType::Suzuki)
        );
        assert_eq!(tree.get("abc").unwrap(), &Vehicle::Car(CarType::Bmw));
        assert_eq!(tree.get("aba").unwrap(), &Vehicle::Car(CarType::Mercedes));
        assert_eq!(tree.get("edc").unwrap(), &Vehicle::Car(CarType::Infiniti));
        assert_eq!(tree.get("edcb").unwrap(), &Vehicle::Car(CarType::Nissan));

        tree.deep_delete("ab");
        tree.deep_delete("abc");
        tree.deep_delete("abcd");
        tree.deep_delete("abcde");

        assert_eq!(tree.get("a"), None);
        assert_eq!(tree.get("ab"), None);
        assert_eq!(tree.get("abc"), None);
        assert_eq!(tree.get("abcd"), None);
        assert_eq!(tree.get("abcde"), None);
        assert_eq!(tree.get("aba").unwrap(), &Vehicle::Car(CarType::Mercedes));
        assert_eq!(tree.get("edc").unwrap(), &Vehicle::Car(CarType::Infiniti));
        assert_eq!(tree.get("edcb").unwrap(), &Vehicle::Car(CarType::Nissan));
    }
}
