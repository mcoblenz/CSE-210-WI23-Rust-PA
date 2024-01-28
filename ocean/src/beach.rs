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
        // unimplemented!();
        Beach{
            crabs: Vec::new(),
            clans: ClanSystem::new(),
        }
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        // unimplemented!();
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
        // unimplemented!();
        self.crabs.push(crab)
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        // unimplemented!();
        &self.crabs[index]
    }

    pub fn crabs(&self) -> Iter<Crab> {
        // unimplemented!();
        self.crabs.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        // unimplemented!();
        if self.crabs.is_empty(){
            return None;
        } else{
            return self.crabs.iter().max_by_key(|crab| crab.speed());
        }
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        // unimplemented!();
        let mut selected_crabs = Vec::new();
        for crab in self.crabs.iter() {
            if crab.name() == name {
                selected_crabs.push(crab);
            }
        }
        return selected_crabs;
        
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        // unimplemented!();
        let crab_1 = &self.crabs[i];
        let crab_2 = &self.crabs[j];
        let breed_color = Color::cross(crab_1.color(), crab_2.color());
        let breed_speed = 1;
        let breed_diet = Diet::random_diet();
        let breed_crab = Crab::new(name, breed_speed, breed_color, breed_diet);
        self.add_crab(breed_crab);
        
    }

    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        // unimplemented!();
        &self.clans
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        // unimplemented!();
        self.clans.add_member_to_clan(clan_id,crab_name)
    }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
    pub fn get_winner_clan(&self, id1: &str, id2: &str) -> Result<Option<String>, String> {
        // unimplemented!();
        let clan_1 = self.clans.get_clan_member_names(id1);
        let clan_2 = self.clans.get_clan_member_names(id2);

        if clan_1.len() < 1 || clan_2.len() < 1{
            return Err("Invalid clan IDs".to_string());
        }

        let mut avg_speed_1 = 0;
        let mut avg_speed_2 = 0;

        for name in clan_1{
            let crabs_1 = self.find_crabs_by_name(&name);
            for crab in crabs_1{
                avg_speed_1 = avg_speed_1 + crab.speed();
            }
        }

        for name in clan_2{
            let crabs_2 = self.find_crabs_by_name(&name);
            for crab in crabs_2{
                avg_speed_2 = avg_speed_2 + crab.speed();
            }
        }

        let len_1 = self.clans.get_clan_member_count(id1);
        let len_2 = self.clans.get_clan_member_count(id2);

        avg_speed_1 = avg_speed_1 / (len_1 as u32);
        avg_speed_2 = avg_speed_2 / (len_2 as u32);

        if avg_speed_1 > avg_speed_2 {
            Ok(Some(id1.to_string()))
        } else if avg_speed_2 > avg_speed_1 {
            Ok(Some(id2.to_string()))
        } else {
            Ok(None)
        }



        
    }
}
