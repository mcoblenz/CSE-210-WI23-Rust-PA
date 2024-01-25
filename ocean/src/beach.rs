use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use crate::clans::ClanSystem;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    // TODO: Declare the fields of the Beach struct here.
    crabs: Vec<Crab>,
    clans: ClanSystem,
}

impl Beach {
    pub fn new() -> Beach {
        let crabs = Vec::new();
        Beach {crabs: crabs, clans: ClanSystem::new()}
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        self.crabs.len()
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.crabs.push(crab)
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        &self.crabs[index]
    }

    pub fn crabs(&self) -> Iter<Crab> {
        self.crabs.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        if (self.size()) == 0 {
            return None;
        }
        let mut fastest = &self.crabs[0]; 
        for item in self.crabs.iter() { 
            if item.speed() > fastest.speed() { 
                fastest = item; 
            } 
        } 
        Some(&fastest)
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        let mut named_crabs = Vec::new();
        
        for crab in self.crabs.iter() {
            if crab.name() == name {
                named_crabs.push(crab)
            }
        }
        return named_crabs
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        let parent1 = self.get_crab(i);
        let parent2 = self.get_crab(j);

        let child = Crab::breed(name, parent1, parent2);
        self.add_crab(child);
    }

    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        return &self.clans
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        self.clans.add_member_to_clan(clan_id, crab_name)
    }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
    pub fn get_winner_clan(&self, id1: &str, id2: &str) -> Result<Option<String>, String> {
        let mut speed1:usize = 0;
        let mut speed2 = 0;

        let mut found_clans = 0;
        for clan in self.clans.clans() {
            if clan.get_id() == id1 {
                for crab_name in clan.get_members() {
                    let crab = self.get_crab_by_name(crab_name);
                    if crab.is_none() {
                        return Err("Crab name couldn't be found".to_string());
                    }
                    speed1 += crab.expect("Handled None").speed() as usize;
                }
                let len = clan.get_members().len();
                speed1 /= len;
                found_clans += 1
            }
            //We don't do an else if here since we may get both clans with the same id. In this case we should return None since they will tie
            if clan.get_id() == id2 {
                for crab_name in clan.get_members() {
                    let crab = self.get_crab_by_name(crab_name);
                    if crab.is_none() {
                        return Err("Crab name couldn't be found".to_string());
                    }
                    speed2 += crab.expect("Handled None").speed() as usize;
                }
                let len = clan.get_members().len();
                speed2 /= len;
                found_clans += 1
            }
        }

        if found_clans < 2 {
            //Didn't find both clans
            return Err("Couldn't find both clans".to_string());
        }

        if speed1 == speed2 {
            return Ok(None);
        }

        if speed1 > speed2 {
            return Ok(Some((*id1).to_string()));
        }
        //Only posibility left is clan2 won 
        return Ok(Some((*id2).to_string()));

    }

    pub fn get_crab_by_name(&self, name: &str) -> Option<&Crab> {
        for crab in &self.crabs {
            if crab.name() == name {
                return Some(&crab)
            }
        }

        return None
    }    
}
