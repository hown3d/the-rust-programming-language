// creating a struct which encapsulates the internal implementation.
// Code that interacts with the data should acccess it via the public API.
pub struct AveragedCollection {
    // fields are private, while struct is public
    list: Vec<i32>,
    average: f64,
}

pub fn new_averaged_collection() -> AveragedCollection {
    AveragedCollection {
        list: vec![],
        average: 0.0,
    }
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value)
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(val) => {
                self.update_average();
                Some(val)
            }
            None => None,
        }
    }
    pub fn average(self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        let len = self.list.len();
        if len == 0 {
            self.average = 0.0;
            return;
        }
        self.average = total as f64 / len as f64;
    }
}
