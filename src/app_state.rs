pub struct AppState {
    is_leader: bool,
}

impl AppState {
    pub fn new(is_leader: bool) -> Self {
        Self {
            is_leader: is_leader,
        }
    }
    pub fn become_leader(&mut self) {
        self.is_leader = true;
    }
    pub fn become_follower(&mut self) {
        self.is_leader = false;
    }   
}