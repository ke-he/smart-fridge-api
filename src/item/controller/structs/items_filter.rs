use serde::Deserialize;

#[derive(Deserialize)]
pub struct ItemsFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub item_type_id: Option<i32>,
}
