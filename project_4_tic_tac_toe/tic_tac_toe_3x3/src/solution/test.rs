use std::collections::HashMap;

struct FrequencyTracker {
    frequencies: HashMap<i32, i32>,
    occurrences: HashMap<i32, i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrequencyTracker {

    fn new() -> Self {
        FrequencyTracker {
            frequencies: HashMap::new(),
            occurrences: HashMap::new()
        }
    }
    
    fn add(&mut self, number: i32) {
        let old_frequency =
            if self.frequencies.contains_key(&number) { *self.frequencies.get(&number).unwrap() }
            else { 0 };
        let new_frequency = old_frequency + 1;
        self.frequencies.insert(number, new_frequency);

        if self.occurrences.contains_key(&old_frequency) {
            let occurrence = *self.occurrences.get(&old_frequency).unwrap();
            self.occurrences.insert(old_frequency, occurrence - 1);
        };
        
        if self.occurrences.contains_key(&new_frequency) {
            let occurrence = *self.occurrences.get(&new_frequency).unwrap();
            self.occurrences.insert(new_frequency, occurrence + 1);
        }
        else {
            self.occurrences.insert(new_frequency, 1);
        };
    }
    
    fn delete_one(&mut self, number: i32) {
        let old_frequency =
            if self.frequencies.contains_key(&number) { *self.frequencies.get(&number).unwrap() }
            else { 0 };
        if old_frequency == 0 { return; }
        let new_frequency = old_frequency - 1;
        self.frequencies.insert(number, new_frequency);

        if self.occurrences.contains_key(&old_frequency) {
            let occurrence = *self.occurrences.get(&old_frequency).unwrap();
            self.occurrences.insert(old_frequency, occurrence - 1);
        };
        
        if self.occurrences.contains_key(&new_frequency) {
            let occurrence = *self.occurrences.get(&new_frequency).unwrap();
            self.occurrences.insert(new_frequency, occurrence + 1);
        }
        else {
            self.occurrences.insert(new_frequency, 1);
        };
    }
    
    fn has_frequency(&self, frequency: i32) -> bool {
        if !self.occurrences.contains_key(&frequency) { return false; }
        let occurrence = *self.occurrences.get(&frequency).unwrap();
        return occurrence > 0;
    }
}

// /**
//  * Your FrequencyTracker object will be instantiated and called as such:
//  * let obj = FrequencyTracker::new();
//  * obj.add(number);
//  * obj.delete_one(number);
//  * let ret_3: bool = obj.has_frequency(frequency);
//  */