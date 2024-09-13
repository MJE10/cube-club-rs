use crate::model::user::User;
use sqlx::SqliteConnection;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Clone)]
pub struct ErrorRecord {
    pub code: u64,
    pub user: Option<User>,
    pub message: String,
    pub stack_trace: String,
}

impl ErrorRecord {
    pub async fn insert(&self, db: &mut SqliteConnection) -> anyhow::Result<()> {
        let code = self.code as i64;
        let query = if let Some(user) = &self.user {
            sqlx::query!("INSERT INTO error (time, code, user, message, stack_trace) VALUES (datetime(), ?, ?, ?, ?)",
                code, user.id, self.message, self.stack_trace)
        } else {
            sqlx::query!(
                "INSERT INTO error (time, code, message, stack_trace) VALUES (datetime(), ?, ?, ?)",
                code,
                self.message,
                self.stack_trace
            )
        };
        Ok(query.execute(db).await.map(|_| ())?)
    }

    pub async fn try_create(
        db: &mut SqliteConnection,
        user: Option<User>,
        error: anyhow::Error,
    ) -> anyhow::Result<()> {
        let message = format!("{error}");
        let stack_trace = format!("{error:?}");

        error!("Recorded: {message}");

        let hash = Self::hash(&error);
        ErrorRecord {
            code: hash,
            user,
            message,
            stack_trace,
        }
        .insert(db)
        .await
    }

    pub fn hash(error: &anyhow::Error) -> u64 {
        let message = error.to_string();
        let mut hasher = DefaultHasher::new();
        message.hash(&mut hasher);
        hasher.finish() % 100000
    }

    pub async fn blind_try_create(
        db: &mut SqliteConnection,
        user: Option<User>,
        error: anyhow::Error,
    ) {
        let _ = ErrorRecord::try_create(db, user, error).await;
    }
}
