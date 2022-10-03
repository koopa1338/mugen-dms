use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "documents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub created: DateTimeWithTimeZone,
    pub last_updated: Option<DateTimeWithTimeZone>,
    pub filetype: Option<String>,
    pub version: i32,
    pub size: i64,
    pub data: Option<Vec<u8>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "Document (")?;
        writeln!(f, "\tid: {}", self.id)?;
        writeln!(f, "\tcreated: {}", self.created)?;
        if let Some(last_updated) = self.last_updated {
            writeln!(f, "\tlast_updated: {}", last_updated)?;
        }
        if let Some(ft) = &self.filetype {
            writeln!(f, "\tfiletype: {}", ft)?;
        }
        writeln!(f, "\tversion: {}", self.version)?;
        writeln!(f, "\tsize: {}", self.size)?;
        write!(f, ")")
    }
}
