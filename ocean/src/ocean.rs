use crate::beach::Beach;
use crate::prey::{Algae, Clam, Minnow, Shrimp};
use crate::reef::Reef;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice::Iter;

#[derive(Debug)]
pub struct Ocean {
    // TODO: Fill in fields here.
    beaches: Vec<Beach>, 
    reefs: Vec<Rc<RefCell<Reef>>>,
}

impl Ocean {
    pub fn new() -> Ocean {
        Ocean{ beaches: Vec::new(), reefs: Vec::new() }
    }

    pub fn add_beach(&mut self, beach: Beach) {
        self.beaches.push(beach);
    }

    pub fn beaches(&self) -> Iter<Beach> {
        self.beaches.iter()
    }

    pub fn reefs(&self) -> Iter<Rc<RefCell<Reef>>> {
        self.reefs.iter()
    }

    /**
     * Generate a reef with the specified number of each concrete type of prey, and then add it to the ocean.
     *   - Minnows should have a speed of 25.
     *   - Shrimp should have an energy of 1.
     *
     * Returns a reference to the newly created reef.
     */
    pub fn generate_reef(
        &mut self,
        n_minnows: u32,
        n_shrimp: u32,
        n_clams: u32,
        n_algae: u32,
    ) -> Rc<RefCell<Reef>> {
        let reef = Rc::new(RefCell::new(Reef::new()));
        for i in 0..n_minnows {
            reef.borrow_mut().add_prey(Box::new(Minnow::new(25)));
        }
        for i in 0..n_shrimp  {
            reef.borrow_mut().add_prey(Box::new(Shrimp::new(1)));
        }
        for i in 0..n_clams  {
            reef.borrow_mut().add_prey(Box::new(Clam::new()));
        }
        for i in 0..n_algae {
            reef.borrow_mut().add_prey(Box::new(Algae::new()));
        }
        self.reefs.push(reef.clone());
        reef
    }
}
