use uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub id: String,
    pub name: String,
    pub description: String,
    pub owner: uuid::Uuid,
}

impl Label {
    pub fn new(id: String, name: String, description: String, owner: uuid::Uuid) -> Label {
        Label {
            id: id,
            name: name,
            description: description,
            owner: owner,
        }
    }
}
