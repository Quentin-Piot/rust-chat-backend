use sea_orm::{ActiveModelTrait, ActiveValue, DbConn, DbErr, DeleteResult};

use crate::entities::group;
use crate::groups::dto::CreateGroup;

pub struct GroupMutation;

impl GroupMutation {
    pub async fn create_group(
        db: &DbConn,
        form_data: CreateGroup,
    ) -> Result<group::ActiveModel, DbErr> {
        let group = group::ActiveModel {
            is_duo: ActiveValue::Set(form_data.is_duo),
            created_by: ActiveValue::Set(form_data.created_by),
            ..Default::default()
        };

        group.save(db).await
    }

    pub async fn delete_group(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let group = group::ActiveModel {
            id: ActiveValue::Set(id),
            ..Default::default()
        };
        group.delete(db).await
    }
}
