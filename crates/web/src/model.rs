#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Check {
    pub check_list_id: String,
    pub item_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckList {
    pub id: String,
    pub date: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Item {
    pub id: String,
    pub name: String,
}
