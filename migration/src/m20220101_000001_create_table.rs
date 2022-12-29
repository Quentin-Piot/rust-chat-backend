use crate::ColumnSpec::Default;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Firstname).string().not_null())
                    .col(ColumnDef::new(User::Lastname).string().not_null())
                    .to_owned(),
            )
            .await
            .expect("Error creating user table");
        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Group::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Group::isDuo).boolean().not_null())
                    .col(ColumnDef::new(Group::CreatedBy).string().not_null())
                    .to_owned(),
            )
            .await
            .expect("Error creating group table");

        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Group::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Group::isDuo).boolean().not_null())
                    .col(ColumnDef::new(Group::CreatedBy).string().not_null())
                    .to_owned(),
            )
            .await
            .expect("Error creating group table");

        manager
            .create_table(
                Table::create()
                    .table(GroupUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GroupUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(GroupUser::Group).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-group_user-group")
                            .from(GroupUser::Table, GroupUser::Group)
                            .to(Group::Table, Group::Id),
                    )
                    .col(ColumnDef::new(Message::User).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-group_user-user")
                            .from(GroupUser::Table, GroupUser::User)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
            .expect("Error creating group user table");
        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Message::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Message::Content).string().not_null())
                    .col(ColumnDef::new(Message::Group).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-message-group")
                            .from(Message::Table, Message::Group)
                            .to(Group::Table, Group::Id),
                    )
                    .col(ColumnDef::new(Message::User).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-message-user")
                            .from(Message::Table, Message::User)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    Id,
    Email,
    Password,
    Firstname,
    Lastname,
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Group {
    Table,
    Id,
    isDuo,
    CreatedBy,
}

#[derive(Iden)]
enum Message {
    Table,
    Id,
    Content,
    Group,
    User,
}

#[derive(Iden)]
enum GroupUser {
    Table,
    Id,
    User,
    Group,
}
