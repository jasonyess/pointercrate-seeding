use pointercrate_core::permission::Permission;
use pointercrate_user::auth::{legacy::Registration, AuthenticatedUser, PasswordOrBrowser};

use crate::gen::name::generate_member_name;

use super::Pointercrate;

impl Pointercrate {
    pub async fn register_user(
        &mut self,
        permission: Option<Permission>,
    ) -> AuthenticatedUser<PasswordOrBrowser> {
        let mut connection = self.connect().await;

        let user = AuthenticatedUser::register(
            Registration {
                name: generate_member_name(&mut self.rng),
                password: "1234567890".to_string(),
            },
            &mut *connection,
        )
        .await
        .unwrap();

        if permission.is_some_and(|p| p.bit() > 0) {
            sqlx::query(
                r#"UPDATE members SET permissions = $2::INTEGER::BIT(16) WHERE member_id = $1"#,
            )
            .bind(user.user().id)
            .bind(permission.unwrap().bit() as i16)
            .execute(&mut *connection)
            .await
            .unwrap();
        }

        log::info!("Registered user with ID {}", &user.user().id);

        user
    }
}
