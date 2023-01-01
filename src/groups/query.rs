use futures::StreamExt;
use sea_orm::{ColumnTrait, DbConn, DbErr, EntityTrait, ModelTrait, QueryFilter};

use crate::entities::group::Model;
use crate::entities::{
    group::Entity as Group, group_user, group_user::Entity as GroupUser,
    message::Entity as Message, user,
};
use crate::groups::dto::PopulatedGroup;
use crate::messages::types::SerializableMessage;
use crate::users::types::SerializableUser;

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

    pub async fn get_user_groups(
        db: &DbConn,
        id: i32,
    ) -> Result<Vec<PopulatedGroup>, &'static str> {
        let groups_found = GroupUser::find()
            .filter(group_user::Column::User.eq(id))
            .all(db)
            .await;

        if groups_found.is_err() {
            return Err("Error while fetching groups");
        }

        let group_users = groups_found.unwrap();

        let futures = group_users.into_iter().map(|group_user| async move {
            let group = Group::find_by_id(group_user.group)
                .one(db)
                .await
                .unwrap()
                .unwrap();
            let user = user::Entity::find_by_id(group_user.user)
                .one(db)
                .await
                .unwrap()
                .unwrap();
            let messages = group.find_related(Message).all(db).await;
            let mut serializable_group = PopulatedGroup {
                id: group.id,
                is_duo: group.is_duo,
                created_by: SerializableUser {
                    id: group_user.id,
                    email: user.email,
                    firstname: user.firstname,
                    lastname: user.lastname,
                },
                users: vec![],
                messages: vec![],
            };
            if messages.is_ok() {
                let messages = messages.unwrap();
                let messages: Vec<SerializableMessage> = messages
                    .into_iter()
                    .map(|v| SerializableMessage {
                        id: v.id,
                        content: v.content,
                        group: v.group,
                        user: v.user,
                    })
                    .collect();
                serializable_group.messages = messages;
            }
            return serializable_group;
        });

        let stream = futures::stream::iter(futures).buffer_unordered(10);

        let populated_groups = stream.collect::<Vec<_>>().await;

        return Ok(populated_groups);
    }
}
