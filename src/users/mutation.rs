use sea_orm::{ActiveModelTrait, ActiveValue, DbConn, DbErr};

use crate::entities::user;

pub struct UserMutation;

impl UserMutation {
    pub async fn create_user(
        db: &DbConn,
        form_data: user::Model,
    ) -> Result<user::ActiveModel, DbErr> {
        let user = user::ActiveModel {
            email: ActiveValue::Set(form_data.email),
            password: ActiveValue::Set(form_data.password),
            ..Default::default()
        };

        user.save(db).await
    }
}
