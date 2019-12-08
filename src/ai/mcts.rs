use std::collections::HashMap;

use crate::game::action::Action;
use crate::game::game_state::GameState;

#[derive(Clone)]
pub struct Node {
    children: Option<HashMap<Action, Option<Node>>>,
    action: Option<Action>,
    parent_visits: usize,
    visits: usize,
    value: f64,
}

impl Node {
    pub fn new(children: Option<HashMap<Action, Option<Node>>>, action: Option<Action>, parent_visits: usize) -> Node {
        Node {
            children,
            action,
            parent_visits,
            visits: 0,
            value: 0.,
        }
    }

    pub fn children(&self) -> &Option<HashMap<Action, Option<Node>>> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Option<HashMap<Action, Option<Node>>> {
        &mut self.children
    }

    pub fn weight(&self) -> f64 {
        if self.visits == 0 { 0. } else { self.value / self.visits as f64 }
    }

    pub fn search_weight(&self) -> f64 {
        self.weight() + (2. * (self.parent_visits as f64).ln() / self.visits as f64).sqrt()
    }

    pub fn fully_expanded(&self) -> bool {
        match self.children() {
            None => false,
            Some(children) => {
                for value in children.values() {
                    if let None = value {
                        return false;
                    }
                }

                true
            }
        }
    }

    pub fn expand(&mut self, game_state: &GameState) -> Action {
        if let None = self.children() {
            self.children = Some(action_hash_map(game_state));
        }

        let mut unexpanded_action: Option<Action> = None;

        if let Some(children) = self.children() {
            for (key, value) in children.iter() {
                if let None = value { unexpanded_action = Some(key.clone()); }
            }
        }

        if let Some(action) = unexpanded_action {
            let new_node = Node::new(None, Some(action.clone()), self.visits);

            if let Some(children) = self.children_mut() {
                children.insert(action.clone(), Some(new_node));
                return action;
            }
        }

        panic!("All children were already expanded!");
    }
}

pub struct Tree {
    root: Node,
}

impl Tree {
    pub fn new(root: Node) -> Tree {
        Tree {
            root,
        }
    }
}

fn action_hash_map(game_state: &GameState) -> HashMap<Action, Option<Node>> {
    game_state.possible_actions()
        .iter()
        .map(|action| action.clone())
        .zip(None)
        .collect()
}

pub fn mcts() {
    let game_state = GameState::new();
    let root = Node::new(
        Some(action_hash_map(&game_state)),
        None,
        0,
    );
    let tree = Tree::new(root);


}
