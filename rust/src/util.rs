/// Remove the `count` highest elements from the `list`
/// If `all_found` is set then all instances of the highest element is removed
/// otherwise only the first is
/// ```
/// let list = vec![30, 10, 30, 40];
/// assert_eq!(filter_highest(&list, 1, true), vec![30, 10, 30]);
/// assert_eq!(filter_highest(&list, 2, true), vec![10]);
/// assert_eq!(filter_highest(&list, 3, true), vec![]);
/// assert_eq!(filter_highest(&list, 1, false), vec![30, 10, 30]);
/// assert_eq!(filter_highest(&list, 2, false), vec![10, 30]);
/// assert_eq!(filter_highest(&list, 3, false), vec![10]);
/// assert_eq!(filter_highest(&list, 4, false), vec![]);
/// let empty_list: Vec<usize> = vec![];
/// assert_eq!(filter_highest(&empty_list, 2, false), vec![]);
/// ```
pub fn filter_highest<T: Ord + Clone>(list: &[T], mut count: usize, all_found: bool) -> Vec<T> {
    let mut list = list.to_vec();
    while count > 0 && !list.is_empty() {
        if all_found {
            let highest = list.iter().max().unwrap();
            list = list.iter().filter(|&num| num != highest).cloned().collect();
        } else {
            let idx = index_of_max(&list).unwrap();
            list.remove(idx);
        }
        count -= 1;
    }
    list
}

/// Returns index of first highest element in the `list`
/// ```
/// let list = vec![30, 10, 30, 40];
/// assert_eq!(index_of_max(list), Some(0));
/// let empty_list = vec![];
/// assert_eq!(index_of_max(empty_list), None);
/// ```
pub fn index_of_max<T: PartialOrd>(list: &[T]) -> Option<usize> {
    let mut idx = None;
    let mut value = None;
    for (li, lv) in list.iter().enumerate() {
        if idx == None || lv > value.unwrap() {
            idx = Some(li);
            value = Some(lv);
        }
    }
    idx
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_index_of_max() {
        let list1 = vec![10, 20, 30];
        let list2 = vec![43243, 333, 30];
        let list3 = vec![90.0, 2.0, 12.0];
        let list4 = vec!['a', 'b', '3'];

        assert_eq!(index_of_max(&list1), Some(2));
        assert_eq!(index_of_max(&list2), Some(0));
        assert_eq!(index_of_max(&list3), Some(0));
        assert_eq!(index_of_max(&list4), Some(1));

        let list5: Vec<usize> = vec![];
        assert_eq!(index_of_max(&list5), None);
    }

    #[test]
    fn test_filter_highest() {
        let list = vec![30, 10, 30, 40];
        assert_eq!(filter_highest(&list, 1, true), vec![30, 10, 30]);
        assert_eq!(filter_highest(&list, 2, true), vec![10]);
        assert_eq!(filter_highest(&list, 3, true), vec![]);

        assert_eq!(filter_highest(&list, 1, false), vec![30, 10, 30]);
        assert_eq!(filter_highest(&list, 2, false), vec![10, 30]);
        assert_eq!(filter_highest(&list, 3, false), vec![10]);
        assert_eq!(filter_highest(&list, 4, false), vec![]);

        let empty_list: Vec<usize> = vec![];
        assert_eq!(filter_highest(&empty_list, 2, false), vec![]);
    }
}