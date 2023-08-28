use std::collections::HashMap;

pub struct TimeMap {
    map: HashMap<String, Vec<(String, i32)>>,
}

impl Default for TimeMap {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((value, timestamp));
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        self.map
            .get(&key)
            .and_then(|v| match v.partition_point(|&(_, t)| t <= timestamp) {
                0 => None,
                i => Some(&v[i - 1].0),
            })
            .cloned()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_time_map() {
        let mut time_map = TimeMap::new();
        time_map.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(time_map.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(time_map.get("foo".to_string(), 3), "bar".to_string());
        time_map.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(time_map.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(time_map.get("foo".to_string(), 5), "bar2".to_string());
    }
}
