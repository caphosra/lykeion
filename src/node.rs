use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::Deref;

pub trait Node: Debug + ToString {
    fn evaluate(&self, props: &HashMap<&str, bool>) -> bool;
    fn retrieve(&self) -> HashSet<&str>;

    fn get_all_propositions(&self) -> Vec<&str> {
        let mut props = self.retrieve().iter().cloned().collect::<Vec<&str>>();
        props.sort();
        props
    }

    fn is_tautology(&self, props: &Vec<&str>) -> Option<bool> {
        if props.len() > 15 {
            None
        } else {
            for i in 0..1 << props.len() {
                let mut map = HashMap::new();
                for (c, p) in props.iter().enumerate() {
                    map.insert(p.deref(), ((i >> c) & 1) == 1);
                }
                if !self.evaluate(&map) {
                    return Some(false);
                }
            }
            Some(true)
        }
    }
}

#[derive(Debug)]
pub struct AndNode {
    pub children: Vec<Box<dyn Node>>,
}

impl ToString for AndNode {
    fn to_string(&self) -> String {
        let children = self
            .children
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ∧ ");

        if self.children.len() == 1 {
            children
        } else {
            format!("({})", children)
        }
    }
}

impl Node for AndNode {
    fn evaluate(&self, props: &HashMap<&str, bool>) -> bool {
        for child in &self.children {
            if !child.evaluate(props) {
                return false;
            }
        }
        true
    }

    fn retrieve(&self) -> HashSet<&str> {
        let mut props = HashSet::new();
        for child in &self.children {
            let child_props = child.retrieve();
            props.extend(child_props);
        }
        props
    }
}

impl AndNode {
    pub fn new(children: Vec<Box<dyn Node>>) -> Box<dyn Node> {
        Box::new(AndNode { children })
    }
}

#[derive(Debug)]
pub struct OrNode {
    pub children: Vec<Box<dyn Node>>,
}

impl ToString for OrNode {
    fn to_string(&self) -> String {
        let children = self
            .children
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ∨ ");

        if self.children.len() == 1 {
            children
        } else {
            format!("({})", children)
        }
    }
}

impl Node for OrNode {
    fn evaluate(&self, props: &HashMap<&str, bool>) -> bool {
        for child in &self.children {
            if child.evaluate(props) {
                return true;
            }
        }
        false
    }

    fn retrieve(&self) -> HashSet<&str> {
        let mut props = HashSet::new();
        for child in &self.children {
            let child_props = child.retrieve();
            props.extend(child_props);
        }
        props
    }
}

impl OrNode {
    pub fn new(children: Vec<Box<dyn Node>>) -> Box<dyn Node> {
        Box::new(OrNode { children })
    }
}

#[derive(Debug)]
pub struct IfThenNode {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
}

impl ToString for IfThenNode {
    fn to_string(&self) -> String {
        format!("({} → {})", self.left.to_string(), self.right.to_string())
    }
}

impl Node for IfThenNode {
    fn evaluate(&self, props: &HashMap<&str, bool>) -> bool {
        !self.left.evaluate(props) || self.right.evaluate(props)
    }

    fn retrieve(&self) -> HashSet<&str> {
        let mut props = self.left.retrieve();

        let child_props = self.right.retrieve();
        props.extend(child_props.iter());

        props
    }
}

impl IfThenNode {
    pub fn new(left: Box<dyn Node>, right: Box<dyn Node>) -> Box<dyn Node> {
        Box::new(IfThenNode { left, right })
    }
}

#[derive(Debug)]
pub struct NotNode {
    pub child: Box<dyn Node>,
}

impl ToString for NotNode {
    fn to_string(&self) -> String {
        format!("¬{}", self.child.to_string())
    }
}

impl Node for NotNode {
    fn evaluate(&self, props: &HashMap<&str, bool>) -> bool {
        !self.child.evaluate(props)
    }

    fn retrieve(&self) -> HashSet<&str> {
        self.child.retrieve()
    }
}

impl NotNode {
    pub fn new(child: Box<dyn Node>) -> Box<dyn Node> {
        Box::new(NotNode { child })
    }
}

#[derive(Debug)]
pub struct PropositionNode {
    pub name: String,
}

impl ToString for PropositionNode {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl Node for PropositionNode {
    fn evaluate(&self, props: &HashMap<&str, bool>) -> bool {
        props.get(&self.name.as_ref()).unwrap().clone()
    }

    fn retrieve(&self) -> HashSet<&str> {
        let mut set = HashSet::new();
        set.insert(self.name.as_ref());
        set
    }
}

impl PropositionNode {
    pub fn new(name: String) -> Box<dyn Node> {
        assert_ne!(name, "X".to_string());

        Box::new(PropositionNode { name })
    }
}

#[derive(Debug)]
pub struct ContradictionNode;

impl ToString for ContradictionNode {
    fn to_string(&self) -> String {
        "⊥".to_string()
    }
}

impl Node for ContradictionNode {
    fn evaluate(&self, _props: &HashMap<&str, bool>) -> bool {
        false
    }

    fn retrieve(&self) -> HashSet<&str> {
        HashSet::new()
    }
}

impl ContradictionNode {
    pub fn new() -> Box<dyn Node> {
        Box::new(ContradictionNode {})
    }
}
