use sqlx::{sqlite::*, Sqlite, FromRow};
use crate::db::Db;
use super::{
    now_ts, Model,
    link::{ItemFieldLink},
};

// NOTE: typ can mean cardinally valued (ie checkboxes), stateful, textbox, etc.
#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Field {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub name: String,
    pub typ: String,
    pub value: String,
    pub private: bool, 
    #[serde(default = "now_ts")]
    pub created_at: i32,
}

#[derive(Default, FromRow, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct FieldEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub eeid: i32,
    pub fid: i32, 
    pub content: String,
} //created at contained in EntryEntryid,

impl Field {
    pub async fn new() {}

    pub async fn add_to_item(self, 
        db: &Db,
        item_id: i32,
        priority: Option<i32>
    ) -> sqlx::Result<Self> {
        ItemFieldLink::create(&db, 
            item_id, 
            self.id.unwrap(), 
            priority).await?;
        Ok(self)
    }
}

pub enum FieldTypes {
    Text,
    TextBox,
    EnumSingle,
    EnumMultiple,
    Boolean,
    Range,
}
