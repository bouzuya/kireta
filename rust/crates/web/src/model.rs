use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use time::Date;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Check {
    pub check_list_id: String,
    pub item_id: String,
}

impl Distribution<Check> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Check {
        Check {
            check_list_id: Uuid::from_bytes(rng.gen()).to_string(),
            item_id: Uuid::from_bytes(rng.gen()).to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckList {
    pub id: String,
    pub date: String,
}

impl Distribution<CheckList> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CheckList {
        CheckList {
            id: Uuid::from_bytes(rng.gen()).to_string(),
            date: rng.gen::<Date>().to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Item {
    pub id: String,
    pub name: String,
}

impl Distribution<Item> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Item {
        Item {
            id: Uuid::from_bytes(rng.gen()).to_string(),
            // TODO: generate random name
            name: Uuid::from_bytes(rng.gen()).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use super::*;

    #[test]
    fn test_impl_distribution_check_for_standard() {
        let mut rng = thread_rng();
        let check1 = rng.gen::<Check>();
        let check2 = rng.gen::<Check>();
        assert_ne!(check1, check2);
    }

    #[test]
    fn test_impl_distribution_check_list_for_standard() {
        let mut rng = thread_rng();
        let check_list1 = rng.gen::<CheckList>();
        let check_list2 = rng.gen::<CheckList>();
        assert_ne!(check_list1, check_list2);
    }

    #[test]
    fn test_impl_distribution_item_for_standard() {
        let mut rng = thread_rng();
        let item1 = rng.gen::<Item>();
        let item2 = rng.gen::<Item>();
        assert_ne!(item1, item2);
    }
}
