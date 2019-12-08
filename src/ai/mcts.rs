use std::collections::HashMap;

use crate::game::action::Action;
use crate::game::game_state::GameState;
use crate::util::non_nan::NonNan;

#[derive(Clone)]
pub struct Node {
    children: HashMap<Action, Option<Node>>,
    children_constructed: bool,
    action: Option<Action>,
    parent_visits: usize,
    visits: usize,
    value: f64,
}

impl Node {
    pub fn new(action: Option<Action>, parent_visits: usize) -> Node {
        Node {
            children: HashMap::new(),
            children_constructed: false,
            action,
            parent_visits,
            visits: 0,
            value: 0.,
        }
    }

    pub fn children(&self) -> &HashMap<Action, Option<Node>> {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut HashMap<Action, Option<Node>> {
        &mut self.children
    }

    pub fn best_child_mut(&mut self) -> Action {
        let (action, _) = &mut self.children_mut().iter().max_by_key(|(_, v)| {
            let weight = match v {
                None => panic!("panic!"),
                Some(node) => node.search_weight(),
            };

            NonNan::new(weight).unwrap()
        }).unwrap();

        action.clone()
    }

    pub fn weight(&self) -> f64 {
        if self.visits == 0 { 0. } else { self.value / self.visits as f64 }
    }

    pub fn search_weight(&self) -> f64 {
        self.weight() + (2. * (self.parent_visits as f64).ln() / self.visits as f64).sqrt()
    }

    pub fn fully_expanded(&self) -> bool {
        if self.children_constructed {
            for value in self.children().values() {
                if let None = value {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    pub fn expand(&mut self, game_state: &GameState) -> Action {
        if !self.children_constructed {
            self.children = action_hash_map(game_state);
        }

        let mut unexpanded_action: Option<Action> = None;

        for (key, value) in self.children().iter() {
            if let None = value { unexpanded_action = Some(key.clone()); }
        }

        if let Some(action) = unexpanded_action {
            let new_node = Node::new(Some(action.clone()), self.visits);

            self.children_mut().insert(action.clone(), Some(new_node));
            return action;
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

pub fn mcts_rec(root: &mut Node, game_state: &mut GameState) {
    if root.fully_expanded() {
        let action = root.best_child_mut();
        let best_child = root.children_mut().get_mut(&action).unwrap();

        match best_child {
            Some(best_child) => mcts_rec(best_child, game_state),
            None => panic!("panic!"),
        }
    } else {
        let action = root.expand(game_state);
    }
}

pub fn mcts(credits: usize) {
    let game_state = GameState::new();
    let root = Node::new(
        None,
        0,
    );
    let tree = Tree::new(root);


}
