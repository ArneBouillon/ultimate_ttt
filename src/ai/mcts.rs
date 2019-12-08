use std::collections::HashMap;

use crate::game::action::Action;
use crate::game::game_state::GameState;

#[derive(Clone)]
pub struct Node<'a> {
    parent: Option<&'a Node<'a>>,
    children: Option<HashMap<Action, Option<Node<'a>>>>,
    action: Option<Action>,
    visits: usize,
    value: f64,
}

impl<'a> Node<'a> {
    pub fn new(parent: Option<&'a Node<'a>>, children: Option<HashMap<Action, Option<Node<'a>>>>, action: Option<Action>) -> Node<'a> {
        Node {
            parent,
            children,
            action,
            visits: 0,
            value: 0.,
        }
    }

    pub fn weight(&self) -> f64 {
        if self.visits == 0 { 0. } else { self.value / self.visits as f64 }
    }

    pub fn search_weight(&self) -> f64 {
        self.weight() + (2. * (self.parent.unwrap().visits as f64).ln() / self.visits as f64).sqrt()
    }

    pub fn fully_expanded(&self) -> bool {
        for value in self.children.unwrap().values() {
            if let None = value {
                return false;
            }
        }

        return true;
    }

    pub fn expand(&'a mut self) -> Node<'a> {
        for (key, value) in self.children.unwrap().iter_mut() {
            if let None = value {
                let new_node = Node::new(Some(&self), None, Some(key.clone()));
                self.children.unwrap().insert(*key, Some(new_node));

                return new_node;
            }
        }

        panic!("All children were already expanded!");
    }
}

pub struct Tree<'a> {
    root: Node<'a>,
}

impl<'a> Tree<'a> {
    pub fn new(root: Node<'a>) -> Tree<'a> {
        Tree {
            root,
        }
    }
}

fn action_hash_map<'a>(game_state: &GameState) -> HashMap<Action, Option<Node<'a>>> {
    game_state.possible_actions()
        .iter()
        .map(|action| action.clone())
        .zip(None)
        .collect()
}

pub fn mcts() {
    let game_state = GameState::new();
    let root = Node::new(
        None,
        Some(action_hash_map(&game_state)),
        None,
    );
    let tree = Tree::new(root);


}
