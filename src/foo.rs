use std::collections::HashSet;

struct Foo {
    item_id: i32,
    tag_ids: HashSet<i32>,
}

impl Foo {
    fn new(item_id: i32, tag_ids: HashSet<i32>) -> Self {
        Self {
            item_id: item_id,
            tag_ids: tag_ids,
        }
    }
}

struct S {
    foos: Vec<Foo>,
}

impl S {
    fn new(foos: Vec<Foo>) -> Self {
        Self { foos: foos }
    }

    fn all_of(&self, tag_ids: &HashSet<i32>) -> HashSet<i32> {
        self.foos
            .iter()
            .filter(|x| {
                x.tag_ids
                    .intersection(tag_ids)
                    .collect::<HashSet<_>>()
                    .len()
                    == tag_ids.len()
            })
            .map(|x| x.item_id)
            .collect()
    }

    fn any_of(&self, tag_ids: &HashSet<i32>) -> HashSet<i32> {
        self.foos
            .iter()
            .filter(|x| {
                !x.tag_ids
                    .intersection(tag_ids)
                    .collect::<HashSet<_>>()
                    .is_empty()
            })
            .map(|x| x.item_id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_of() {
        let s = S::new(vec![
            Foo::new(100, [1, 2, 3].iter().cloned().collect()),
            Foo::new(101, [1, 2].iter().cloned().collect()),
            Foo::new(102, [1, 3, 5].iter().cloned().collect()),
            Foo::new(103, [2, 4, 5].iter().cloned().collect()),
        ]);
        let result = s.all_of(&[1, 2].iter().cloned().collect());
        assert_eq!(2, result.len());
        assert!(result.contains(&100));
        assert!(result.contains(&101))
    }

    #[test]
    fn test_any_of() {
        {
            let s = S::new(vec![
                Foo::new(100, [1, 2, 3].iter().cloned().collect()),
                Foo::new(101, [1, 2].iter().cloned().collect()),
                Foo::new(102, [1, 3, 5].iter().cloned().collect()),
                Foo::new(103, [2, 4, 5].iter().cloned().collect()),
            ]);
            let result = s.any_of(&[3, 4].iter().cloned().collect());
            assert_eq!(3, result.len());
            assert!(result.contains(&100));
            assert!(result.contains(&102));
            assert!(result.contains(&103))
        }

        {
            let s = S::new(vec![
                Foo::new(100, [1, 2, 3].iter().cloned().collect()),
                Foo::new(101, [1, 2].iter().cloned().collect()),
                Foo::new(102, [1, 3, 5].iter().cloned().collect()),
                Foo::new(103, [2, 4, 5].iter().cloned().collect()),
            ]);
            let result = s.any_of(&[4].iter().cloned().collect());
            assert_eq!(1, result.len());
            assert!(result.contains(&103))
        }

        {
            let s = S::new(vec![
                Foo::new(100, [1, 2, 3].iter().cloned().collect()),
                Foo::new(101, [1, 2].iter().cloned().collect()),
                Foo::new(102, [1, 3, 5].iter().cloned().collect()),
                Foo::new(103, [2, 4, 5].iter().cloned().collect()),
            ]);
            let result = s.any_of(&[6].iter().cloned().collect());
            assert!(result.is_empty())
        }
    }
}
