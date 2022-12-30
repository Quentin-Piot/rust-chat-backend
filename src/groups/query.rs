use sea_orm::{
    ColumnTrait, DbConn, DbErr, EntityTrait,
    QueryFilter,
};

use crate::entities::{group::Entity as Group, group_user, group_user::Entity as GroupUser};
use crate::entities::group::Model;

pub struct GroupQuery;

impl GroupQuery {
    pub async fn find_single_group(db: &DbConn, id: i32) -> Result<Option<Model>, DbErr> {
        Group::find_by_id(id).one(db).await
    }

    pub async fn find_group_users(db: &DbConn, id: i32) -> Result<Vec<group_user::Model>, DbErr> {
        GroupUser::find()
            .filter(group_user::Column::User.eq(id))
            .all(db)
            .await
    }
}
