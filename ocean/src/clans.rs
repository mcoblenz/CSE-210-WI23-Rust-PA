use std::collections::HashMap;

#[derive(Debug)]
pub struct ClanSystem {
    // TODO: add necessary fields
    my_map: HashMap<String, Vec<String>>,
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        ClanSystem{ my_map : HashMap::new() }
    }

    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        if(self.my_map.contains_key(clan_id)) {
            let list = self.my_map.get_mut(clan_id);
            list.unwrap().push(crab_name.to_string());
        } else {
            let mut list: Vec<String> = Vec::new();
            list.push(crab_name.to_string());
            self.my_map.insert(clan_id.to_string(), list);
        }
    }

    pub fn contains_clan(&self, clan_id: &str) -> bool {
        return self.my_map.contains_key(clan_id);
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        if(self.my_map.contains_key(clan_id)) {
            return self.my_map.get(clan_id).unwrap().to_vec();
        } else {
            return Vec::new();
        }
    }

    /**
     * Returns the number of clans currently in existence.
     */
    pub fn get_clan_count(&self) -> usize {
        return self.my_map.len();
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
        if(self.my_map.contains_key(clan_id)) {
            return self.my_map.get(clan_id).unwrap().len();
        } else {
            return 0;
        }
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        let mut largest = 0;
        let mut largest_idx = None;
        if(self.my_map.is_empty()) {
            return None;
        }
        for (key, value) in self.my_map.iter() {
            if(value.len() > largest) {
                largest = value.len();
                largest_idx = Some(key);
            }
        }
        if(largest_idx.is_none()) { return None; }
        return Some(largest_idx?.to_string());
    }
}
