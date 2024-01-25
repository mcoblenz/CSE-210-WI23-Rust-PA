use std::slice::Iter;

#[derive(Debug)]
pub struct Clan {
    id: String,
    members: Vec<String>,
}

impl Clan {
    pub fn new(id: String) -> Clan {
        Clan { id, members: Vec::new() }
    }

    pub fn add_member(&mut self, crab_name: String) {
        self.members.push(crab_name);
    }

    pub fn get_id(&self)-> String {
        self.id.clone()
    }

    pub fn get_members(&self) -> &Vec<String> {
        &self.members
    }
    // pub fn members(&self) -> Iter<String> {
    //     self.members.iter()
    // }
}

#[derive(Debug)]
pub struct ClanSystem {
    // TODO: add necessary fields
    clans: Vec<Clan>
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        ClanSystem {clans: Vec::new()}
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        let mut result = Vec::new();
        for clan in &self.clans {
            if clan_id == clan.id {
                for member in clan.members.iter() {
                    result.push(member.clone());
                }
                break;
            }
        }
        return result
    }

    /**
     * Returns the number of clans currently in existence.
     */
    pub fn get_clan_count(&self) -> usize {
        self.clans.len()
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
        for clan in &self.clans {
            if clan_id == clan.id {
                return clan.members.len();
            }
        }
        return 0;
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        if self.clans.len() < 1 {
            return None;
        }

        let mut largest_size = self.clans[0].members.len();
        let mut largest_id = self.clans[0].id.clone();
        for clan in &self.clans {
            let size = clan.members.len();
            if size > largest_size {
                largest_size = size;
                largest_id = clan.id.clone();
            }
        }

        return Some(largest_id);
    }

    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        for clan in &mut self.clans {
            if clan.id == clan_id {
                clan.add_member(crab_name.to_string());
                return
            }
        }

        //Otherwise, add new clan
        self.clans.push(Clan::new(clan_id.to_string()));
        let clan_count = self.get_clan_count();
        self.clans[clan_count-1].add_member(crab_name.to_string());
    }
    
    pub fn clans(&self) -> Iter<Clan> {
        self.clans.iter()
    }
}