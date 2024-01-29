use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use crate::clans::ClanSystem;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    // TODO: Declare the fields of the Beach struct here.
    all_crabs: Vec<Crab>,
    clans: ClanSystem,
}

impl Beach {
    pub fn new() -> Beach  {
        Beach{all_crabs: Vec::new(), clans: ClanSystem::new()}
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        self.all_crabs.len()    
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.all_crabs.push(crab)
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        &self.all_crabs[index]
    }

    pub fn crabs(&self) -> Iter<Crab> {
        self.all_crabs.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        if self.size() == 0 {
            None
        } else {
            let mut speed = 0;
            let mut idx = 0;
            for(index, crab) in self.crabs().enumerate() {
                if(crab.speed > speed) {
                    speed = crab.speed;
                    idx = index;
                }
            }
            Some(self.get_crab(idx))
        }
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        let mut named_crabs: Vec<&Crab> = Vec::new();  
        for(index, crab) in self.crabs().enumerate() {
            if(crab.name == name) {
                named_crabs.push(crab);
            }
        }
        named_crabs
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        if(self.all_crabs.get(i).is_none() || self.all_crabs.get(j).is_none()) {
            panic!()
        } else {
            let diet = Diet::random_diet();
            let color = Color::cross(&self.get_crab(i).color, &self.get_crab(j).color);
            self.add_crab(Crab::breed(&name, color, diet));
        }
    }

    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        return &self.clans;
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        let clans: &mut ClanSystem = &mut self.clans;
        clans.add_member_to_clan(clan_id, crab_name);    
    }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
    pub fn get_winner_clan(&self, id1: &str, id2: &str) -> Result<Option<String>, String> {
        let clans = self.get_clan_system();
        if(!clans.contains_clan(id1) || !clans.contains_clan(id2)) {
            return Err("Invalid Inputs".to_string());
        }
        let list1 = clans.get_clan_member_names(id1);
        let list2 = clans.get_clan_member_names(id2);
        let mut speed1 = 0;
        let mut speed2 = 0;
        for(index, name) in list1.iter().enumerate() {
            let currCrab = self.find_crabs_by_name(name)[0];
            speed1 += currCrab.speed();
        }
        speed1/=(list1.len() as u32);
        for(index, name) in list2.iter().enumerate() {
            let currCrab = self.find_crabs_by_name(name)[0];
            speed2 += currCrab.speed();
        }
        speed2/=(list2.len() as u32);
        if(speed1 == speed2) {
            return Ok(None);
        } else {
            if(speed1 > speed2) {
                return Ok(Some(id1.to_string()));
            } else {
                return Ok(Some(id2.to_string()));
            }
        }
    }
}
