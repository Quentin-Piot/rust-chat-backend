use sea_orm::{ActiveModelTrait, ActiveValue, DbConn, DbErr, DeleteResult};

use crate::entities::group;
use crate::entities::group_user::ActiveModel;
use crate::groups::dto::CreateGroup;
use crate::groups::query::GroupQuery;

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

    pub async fn join_group(db: &DbConn, id: i32, user_id: i32) -> Result<ActiveModel, DbErr> {
        let users_result = GroupQuery::find_group_users(db, user_id).await;

        if users_result.is_err() {
            return Err(users_result.err().unwrap());
        }

        let users = users_result.unwrap();

        let user_found = users.iter().find(|&x| x.user == user_id);

        if user_found.is_none() {
            let group_user = ActiveModel {
                group: ActiveValue::Set(id),
                user: ActiveValue::Set(user_id),
                ..Default::default()
            };

            return group_user.save(db).await;
        } else {
            return Err(DbErr::Custom("User already exists".into()));
        }
    }

    pub async fn delete_group(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let group = group::ActiveModel {
            id: ActiveValue::Set(id),
            ..Default::default()
        };
        group.delete(db).await
    }
}
