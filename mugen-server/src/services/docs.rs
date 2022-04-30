use entity::document::{Entity as Document, Model as DocumentModel};
use sea_orm::{prelude::*, DatabaseConnection, DeleteResult, IntoActiveModel, Set};

pub async fn get_docs(conn: &DatabaseConnection) -> Result<Vec<DocumentModel>, DbErr> {
    Document::find().all(conn).await
}

pub async fn get_doc_by_id(id: i64, conn: &DatabaseConnection) -> Result<DocumentModel, DbErr> {
    Document::find_by_id(id)
        .one(conn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound(format!("No Document with id {id} found")))
}

pub async fn create_doc(
    data: DocumentModel,
    conn: &DatabaseConnection,
) -> Result<DocumentModel, DbErr> {
    Document::insert(data.into_active_model())
        .exec_with_returning(conn)
        .await
}

pub async fn update_doc(
    data: DocumentModel,
    id: i64,
    conn: &DatabaseConnection,
) -> Result<DocumentModel, DbErr> {
    let mut active_model = data.into_active_model();
    active_model.id = Set(id);
    Document::update(active_model).exec(conn).await
}

pub async fn delete_doc(id: i64, conn: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
    let document = get_doc_by_id(id, conn).await?;

    Document::delete(document.into_active_model())
        .exec(conn)
        .await
}
