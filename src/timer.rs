use std::time::Duration;

pub struct Timer {
    pub duration: Duration,
    pub time_left: Duration,
    pub ready: bool,
}


impl Timer {
    pub fn from_millis(millis: u64) -> Self {
        Timer {
            duration: Duration::from_millis(millis),
            time_left: Duration::from_millis(millis),
            ready: false, 
        }
    }

    pub fn update(&mut self, delta: Duration) {
        if ! self.ready {
            if let Some(time_left) = self.time_left.checked_sub(delta) {
                self.time_left = time_left;
            } else {
                self.time_left = Duration::from_millis(0);
                self.ready = true;
            }
        } 
    }

    pub fn reset(&mut self) {
        self.ready = false;
        self.time_left = self.duration;
    }
}

