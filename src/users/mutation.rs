use sea_orm::{ActiveModelTrait, ActiveValue, DbConn, DbErr, DeleteResult};

use crate::entities::user;
use crate::entities::user::Model;
use crate::users::dto::{CreateUser, UpdateUser};

pub struct UserMutation;

impl UserMutation {
    pub async fn create_user(
        db: &DbConn,
        form_data: CreateUser,
    ) -> Result<user::ActiveModel, DbErr> {
        let user = user::ActiveModel {
            email: ActiveValue::Set(form_data.email),
            password: ActiveValue::Set(form_data.password),
            firstname: ActiveValue::Set(form_data.firstname),
            lastname: ActiveValue::Set(form_data.lastname),
            ..Default::default()
        };

        user.save(db).await
    }
    pub async fn update_user(db: &DbConn, id: i32, form_data: UpdateUser) -> Result<Model, DbErr> {
        let mut user = user::ActiveModel {
            id: ActiveValue::Set(id),
            ..Default::default()
        };

        if form_data.email.is_some() {
            user.email = ActiveValue::Set(form_data.email.unwrap());
        }
        if form_data.password.is_some() {
            user.password = ActiveValue::Set(form_data.password.unwrap());
        }
        if form_data.firstname.is_some() {
            user.firstname = ActiveValue::Set(form_data.firstname.unwrap());
        }
        if form_data.lastname.is_some() {
            user.email = ActiveValue::Set(form_data.lastname.unwrap());
        }

        user.update(db).await
    }
    pub async fn delete_user(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let user = user::ActiveModel {
            id: ActiveValue::Set(id),
            ..Default::default()
        };
        user.delete(db).await
    }
}
