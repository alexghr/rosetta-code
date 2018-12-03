// Finally understood closure lifetimes from this blog post
// https://stevedonovan.github.io/rustifications/2018/08/18/rust-closures-are-hard.html
// ❤
pub struct Set<'a> {
    pred: Box<Fn(f64) -> bool + 'a>
}

impl<'a> Set<'a> {
    pub fn new<P>(pred: P) -> Set<'a> 
    where P: Fn(f64) -> bool + 'a {
        Set {
            pred: Box::new(pred)
        }
    }

    pub fn contains(&self, el: f64) -> bool {
        (self.pred)(el)
    }

    pub fn union(first: &'a Set, second: &'a Set) -> Set<'a> {
        let c = move |x| { first.contains(x) || second.contains(x) };
        Set::new(c)
    }

    pub fn intersection(first: &'a Set, second: &'a Set) -> Set<'a> {
        let c = move |x| { first.contains(x) && second.contains(x) };
        Set::new(c)
    }

    pub fn difference(first: &'a Set, second: &'a Set) -> Set<'a> {
        let c = move |x: f64| { first.contains(x) && !second.contains(x) };
        Set::new(c)
    }
}

#[cfg(test)]
mod tests {
    use super::Set;

    #[test]
    fn test_contains() {
        let set = Set::new(|x| { 0.0 < x && x < 1.0 });
        assert_eq!(set.contains(0.5), true);
        assert_eq!(set.contains(0.0), false);
        assert_eq!(set.contains(1.0), false);
    }

    #[test]
    fn test_union() {
        let set_1 = Set::new(|x| { 0.0 < x && x <= 1.0 });
        let set_2 = Set::new(|x| { 0.0 <= x && x < 2.0 });

        let result = Set::union(&set_1, &set_2);

        assert_eq!(result.contains(0.0), true);
        assert_eq!(result.contains(1.0), true);
        assert_eq!(result.contains(2.0), false);
    }

    #[test]
    fn test_intersection() {
        let set_1 = Set::new(|x| { 0.0 <= x && x < 2.0 });
        let set_2 = Set::new(|x| { 1.0 < x && x <= 2.0 });

        let result = Set::intersection(&set_1, &set_2);

        assert_eq!(result.contains(0.0), false);
        assert_eq!(result.contains(1.0), false);
        assert_eq!(result.contains(2.0), false);
    }

    #[test]
    fn test_diff_1() {
        let set_1 = Set::new(|x| { 0.0 <= x && x < 3.0 });
        let set_2 = Set::new(|x| { 0.0 < x && x < 1.0 });

        let result = Set::difference(&set_1, &set_2);

        assert_eq!(result.contains(0.0), true);
        assert_eq!(result.contains(1.0), true);
        assert_eq!(result.contains(2.0), true);
    }

    #[test]
    fn test_diff_2() {
        let set_1 = Set::new(|x| { 0.0 <= x && x < 3.0 });
        let set_2 = Set::new(|x| { 0.0 <= x && x <= 1.0 });

        let result = Set::difference(&set_1, &set_2);

        assert_eq!(result.contains(0.0), false);
        assert_eq!(result.contains(1.0), false);
        assert_eq!(result.contains(2.0), true);
    }
}
