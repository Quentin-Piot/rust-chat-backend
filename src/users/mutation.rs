use sea_orm::{ActiveModelTrait, ActiveValue, DbConn, DbErr};

use crate::entities::user;
use crate::entities::user::Model;

pub struct UserMutation;

impl UserMutation {
    pub async fn create_user(db: &DbConn, form_data: Model) -> Result<user::ActiveModel, DbErr> {
        let user = user::ActiveModel {
            email: ActiveValue::Set(form_data.email),
            password: ActiveValue::Set(form_data.password),
            ..Default::default()
        };

        user.save(db).await
    }
    pub async fn update_user(db: &DbConn, form_data: Model) -> Result<Model, DbErr> {
        let user = user::ActiveModel {
            id: ActiveValue::Set(form_data.id),
            email: ActiveValue::Set(form_data.email),
            password: ActiveValue::Set(form_data.password),
            ..Default::default()
        };

        user.update(db).await
    }
}
