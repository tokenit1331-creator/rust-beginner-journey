#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_enumerate() {
        let items = vec!["a", "b", "c"];
        let result: Vec<(usize, &&str)> = items.iter().enumerate().collect();
        assert_eq!(result, vec![(0, &"a"), (1, &"b"), (2, &"c")]);
    }

    #[test]
    fn test_enumerate_starts_at_zero() {
        let items = vec![10, 20, 30];
        let first = items.iter().enumerate().next();
        assert_eq!(first, Some((0, &10)));
    }

    #[test]
    fn test_filter_before_enumerate() {
        let items = vec![1, 2, 3, 4, 5];
        let result: Vec<(usize, &i32)> = items.iter()
            .filter(|x| **x % 2 == 0)
            .enumerate()
            .collect();
        assert_eq!(result, vec![(0, &2), (1, &4)]);
    }

    #[test]
    fn test_enumerate_before_filter() {
        let items = vec![1, 2, 3, 4, 5];
        let result: Vec<(usize, &i32)> = items.iter()
            .enumerate()
            .filter(|(_, x)| **x % 2 == 0)
            .collect();
        assert_eq!(result, vec![(1, &2), (3, &4)]);
    }

    #[test]
    fn test_enumerate_collect_hashmap() {
        use std::collections::HashMap;
        let items = vec!["x", "y", "z"];
        let map: HashMap<usize, &&str> = items.iter().enumerate().collect();
        assert_eq!(map.get(&0), Some(&&"x"));
        assert_eq!(map.get(&1), Some(&&"y"));
        assert_eq!(map.get(&2), Some(&&"z"));
    }

    #[test]
    fn test_position() {
        let items = vec!["send", "receive", "send", "stake"];
        assert_eq!(items.iter().position(|s| *s == "send"), Some(0));
        assert_eq!(items.iter().position(|s| *s == "stake"), Some(3));
        assert_eq!(items.iter().position(|s| *s == "swap"), None);
    }

    #[test]
    fn test_rposition() {
        let items = vec!["send", "receive", "send", "stake"];
        assert_eq!(items.iter().rposition(|s| *s == "send"), Some(2));
    }

    #[test]
    fn test_enumerate_mut() {
        let mut items = vec![1, 2, 3];
        for (i, val) in items.iter_mut().enumerate() {
            *val += i;
        }
        assert_eq!(items, vec![1, 3, 5]);
    }

    #[test]
    fn test_pagination() {
        let items = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let page: Vec<(usize, &i32)> = items.iter()
            .enumerate()
            .skip(2)
            .take(3)
            .collect();
        assert_eq!(page, vec![(2, &2), (3, &3), (4, &4)]);
    }

    #[test]
    fn test_zip_as_enumerate() {
        let items = vec!["a", "b", "c"];
        let result: Vec<(usize, &&str)> = items.iter().zip(0..).collect();
        assert_eq!(result, vec![(0, &"a"), (1, &"b"), (2, &"c")]);
    }

    #[test]
    fn test_zip_with_offset() {
        let items = vec!["a", "b", "c"];
        let result: Vec<(usize, &&str)> = items.iter().zip(1..).collect();
        assert_eq!(result, vec![(1, &"a"), (2, &"b"), (3, &"c")]);
    }

    #[test]
    fn test_empty_enumerate() {
        let empty: Vec<i32> = vec![];
        let result: Vec<(usize, &i32)> = empty.iter().enumerate().collect();
        assert!(result.is_empty());
    }

    #[test]
    fn test_enumerate_map() {
        let items = vec![10, 20, 30];
        let result: Vec<String> = items.iter()
            .enumerate()
            .map(|(i, val)| format!("[{}]:{}", i, val))
            .collect();
        assert_eq!(result, vec!["[0]:10", "[1]:20", "[2]:30"]);
    }

    #[test]
    fn test_reverse_map() {
        use std::collections::HashMap;
        let items = vec!["a", "b", "c"];
        let rev: HashMap<&&str, usize> = items.iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect();
        assert_eq!(rev.get(&"a"), Some(&0));
        assert_eq!(rev.get(&"c"), Some(&2));
    }

    #[test]
    fn test_unicode_chars() {
        let s = "привет";
        let result: Vec<(usize, char)> = s.chars().enumerate().collect();
        assert_eq!(result.len(), 6);
        assert_eq!(result[0], (0, 'п'));
        assert_eq!(result[3], (3, 'в'));
    }
}
