use sea_orm::{ActiveModelTrait, ActiveValue, DbConn, DbErr, DeleteResult};

use crate::entities::message;
use crate::messages::dto::CreateMessage;

pub struct MessageMutation;

impl MessageMutation {
    pub async fn create_message(
        db: &DbConn,
        form_data: CreateMessage,
    ) -> Result<message::ActiveModel, DbErr> {
        let message = message::ActiveModel {
            content: ActiveValue::Set(form_data.content),
            group: ActiveValue::Set(form_data.group),
            user: ActiveValue::Set(form_data.user),
            ..Default::default()
        };

        message.save(db).await
    }

    pub async fn delete_message(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let message = message::ActiveModel {
            id: ActiveValue::Set(id),
            ..Default::default()
        };
        message.delete(db).await
    }
}
