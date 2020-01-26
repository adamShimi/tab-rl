use std::collections::HashMap;

use std::hash::Hash;

pub fn find_optimal<'a,S,A>(_prob : &MDP<'a,S,A>) -> Policy
  where S : State,
        A : Action {
  unimplemented!("Still everything to do!");
}

fn policy_evaluation<'a,S,A,M>(prob : &MDP<'a,S,A>,
                               pol : &Policy,
                               thresh : f64) -> StateValue
  where S : State,
        A : Action {

  let mut max_diff = thresh;
  let mut value = vec![0.0; prob.states.len()];

  while max_diff >= thresh - std::f64::EPSILON {
    for index in 0..prob.states.len() {
      let (reward,index_next) = prob.dynamics.get(&(index,
                                                    *pol.choice.get(index).unwrap()
                                                   )
                                             )
                                             .unwrap();
      let update : f64 = (*reward as f64)+prob.discount*value[*index_next];
      max_diff = max_diff.max((update-value[index]).abs());
      value[index] = update;
    }
  }
  StateValue {value}
}

fn policy_improvement<'a,S,A,M>(_prob : &MDP<'a,S,A>,
                                _pol : &Policy,
                                _val : &StateValue) -> Policy
  where S : State,
        A : Action {

  unimplemented!("Second block of General Policy Iteration");
}

pub trait State : Eq + Hash + Copy {}
pub trait Action : Eq + Hash + Copy {}

// Deterministic policy that gives the index of the chosen action
// for the indexed state.
pub struct Policy {
  choice : Vec<usize>,
}

pub struct StateValue {
  value : Vec<f64>,
}

// Deterministic MDP
pub struct MDP<'a, S : State, A : Action> {
  pub states : &'a [S],
  pub actions : &'a [A],
  pub discount : f64,
  // Link a tuple (index state,index action) deterministically
  // to the tuple (reward, index next state).
  pub dynamics : HashMap<(usize,usize),(isize,usize)>,
}

#[cfg(test)]
mod tests {

  use super::*;

  struct GridState {
    abs : usize,
    ord : usize,
  }

  impl State for GridState {}

  enum GridAction {
    Up,
    Down,
    Left,
    Right,
  }

  impl Action for GridAction {}

  static GRID_TEST : MDP<'static,GridState,GRidAction> = MDP {
    states : [GridState {abs : 0, ord: 0},
              GridState {abs : 0, ord: 1},
              GridState {abs : 1, ord: 0},
              GridState {abs : 1, ord: 1}],
    actions : [GridAction::Up,
               GridAction::Down,
               GridAction::Left,
               GridAction::Right],
    discount : 1.0,
    dynamics : [((0,0),(-1,2)),
                ((0,1),(-1,0)),
                ((0,2),(-1,0)),
                ((0,3),(-1,1)),

                ((1,0),(-1,1)),
                ((1,1),(-1,0)),
                ((1,2),(-1,1)),
                ((1,3),(0,3)),

                ((2,0),(0,3)),
                ((2,1),(-1,2)),
                ((2,2),(-1,0)),
                ((2,3),(-1,2)),

                ((3,0),(0,3)),
                ((3,1),(0,3)),
                ((3,2),(0,3)),
                ((3,3),(0,3))].iter().cloned().collect(),
  };

  #[test]
  fn it_works() {

      assert_eq!(2 + 2, 4);
  }
}
