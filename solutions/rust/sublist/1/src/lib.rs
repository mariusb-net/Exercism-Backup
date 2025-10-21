#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    fn is_subslice(hay: &[i32], needle: &[i32]) -> bool {
        if needle.is_empty() {
            return true;
        }
        if needle.len() > hay.len() {
            return false;
        }
        hay.windows(needle.len()).any(|w| w == needle)
    }

    if first_list == second_list {
        Comparison::Equal
    } else if is_subslice(second_list, first_list) {
        Comparison::Sublist
    } else if is_subslice(first_list, second_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
