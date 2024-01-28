
#[derive(Debug)]
pub struct ClanSystem {
    // TODO: add necessary fields
    clans: Vec<(String, Vec<String>)>,
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        // unimplemented!();
        ClanSystem { clans: Vec::new() }
        
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        // unimplemented!();
        for (id,members) in &self.clans{
            if id == clan_id{
                return members.clone();
            }
        }
        //return an empty vec if not found
        return Vec::new();
    }

    /**
     * Returns the number of clans currently in existence.
     */
    pub fn get_clan_count(&self) -> usize {
        // unimplemented!();
        self.clans.len()
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
        // unimplemented!();
        for (id,members) in &self.clans{
            if id == clan_id{
                return members.len();
            }
        }
        return 0; 
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        // unimplemented!();
        let mut max_id: Option<String> = None;
        let mut max_num = 0;

        for (id,members) in &self.clans{
            if members.len() > max_num{
                max_num = members.len();
                max_id = Some(id.clone());
            }
        }

        return max_id; 


    }

    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str){
        let mut found_id = false;
        for (id,members) in &mut self.clans{
            if id == clan_id{
                members.push(crab_name.to_string());
                found_id = true;
            }
        }

        // add a new clan with the crab into clans
        if !found_id{
            let mut new_member: Vec<String> = Vec::new();
            new_member.push(crab_name.to_string());

            self.clans.push((clan_id.to_string(), new_member));

        }
    }
}