use Comparison::*;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_lst: &[T], second_lst: &[T]) -> Comparison {
    if first_lst == second_lst {
        Equal
    } else if is_sublist(first_lst, second_lst) {
        Sublist
    } else if is_sublist(second_lst, first_lst) {
        Superlist
    } else {
        Unequal
    }
}

// Check if first_lst occurs in second_lst
fn is_sublist<T: PartialEq>(first_lst: &[T], second_lst: &[T]) -> bool {
    if first_lst.len() <= second_lst.len() {
        for offset in 0..=second_lst.len() - first_lst.len() {
            if second_lst[offset..].starts_with(first_lst) {
                return true;
            }
        }
    }
    false
}
