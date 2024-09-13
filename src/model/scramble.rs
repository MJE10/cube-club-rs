use crate::model::puzzle::Puzzle;
use anyhow::anyhow;
use rocket::futures::TryStreamExt;
use rocket::tokio::sync::Mutex;
use sqlx::SqliteConnection;
use std::collections::HashMap;
use std::ops::Deref;
use std::process::Stdio;
use std::sync::Arc;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Scramble {
    pub id: i64,
    pub scramble: String,
    pub puzzle: Puzzle,
}

#[derive(Clone)]
pub struct ScrambleManager {
    pub scrambles: Arc<Mutex<HashMap<Puzzle, Vec<String>>>>,
    pub is_resetting: Arc<Mutex<HashMap<Puzzle, bool>>>,
}

impl ScrambleManager {
    pub fn new() -> Self {
        let mut scrambles = HashMap::new();
        let mut resetting = HashMap::new();
        for puzzle in [Puzzle::Three, Puzzle::Two] {
            scrambles.insert(puzzle, vec![]);
            resetting.insert(puzzle, false);
        }
        let s = Self {
            scrambles: Arc::new(Mutex::new(scrambles)),
            is_resetting: Arc::new(Mutex::new(resetting)),
        };
        s.async_refill();
        s
    }

    pub async fn get_scramble(&self, puzzle: Puzzle) -> anyhow::Result<String> {
        self.async_refill();
        let mut scrambles = self.scrambles.lock().await;
        let v = scrambles
            .get_mut(&puzzle)
            .ok_or(anyhow!("puzzle not supported for scrambling"))?;
        if v.is_empty() {
            Self::generate_puzzle_count(puzzle, 1)
                .await
                .map(|v| v[0].clone())
        } else {
            Ok(v.remove(0))
        }
    }

    fn async_refill(&self) {
        let s2 = self.clone();
        tokio::spawn(async move {
            s2.refill().await;
        });
    }

    async fn refill(&self) {
        let mut puzzles_to_generate = vec![];
        {
            let scrambles = self.scrambles.lock().await;
            let mut resetting = self.is_resetting.lock().await;
            for (puzzle, scrambles) in scrambles.deref() {
                if scrambles.len() < 10 {
                    let should_spawn = match resetting.get(puzzle).unwrap() {
                        true => false,
                        false => {
                            resetting.insert(*puzzle, true);
                            true
                        }
                    };
                    if should_spawn {
                        puzzles_to_generate.push(*puzzle);
                    }
                }
            }
        }
        for puzzle in puzzles_to_generate {
            let s2 = self.clone();
            tokio::spawn(async move {
                if let Ok(mut scrambles) = Self::generate_puzzle_count(puzzle, 10).await {
                    s2.scrambles
                        .lock()
                        .await
                        .get_mut(&puzzle)
                        .unwrap()
                        .append(&mut scrambles);
                }
                s2.is_resetting.lock().await.insert(puzzle, false);
            });
        }
    }

    async fn generate_puzzle_count(puzzle: Puzzle, count: usize) -> anyhow::Result<Vec<String>> {
        let output = tokio::process::Command::new("tnoodle.bat")
            .args([
                "scramble",
                "-c",
                &count.to_string(),
                "-p",
                puzzle.tnoodle_name(),
            ])
            .stdout(Stdio::piped())
            .spawn()?
            .wait_with_output()
            .await?;
        Ok(String::from_utf8_lossy(&output.stdout)
            .split('\n')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect())
    }
}

#[allow(dead_code)]
impl Scramble {
    pub async fn get(db: &mut SqliteConnection, id: i64) -> anyhow::Result<Self> {
        let r = sqlx::query!("SELECT id, scramble, puzzle FROM scramble WHERE id = ?", id)
            .fetch_one(db)
            .await?;
        Ok(Self {
            id,
            scramble: r.scramble,
            puzzle: Puzzle::from_id(r.puzzle).ok_or(anyhow!("invalid puzzle"))?,
        })
    }

    pub async fn generate(
        db: &mut SqliteConnection,
        scrambles: &ScrambleManager,
        puzzle: Puzzle,
    ) -> anyhow::Result<Self> {
        let scramble = scrambles.get_scramble(puzzle).await?;
        let scramble2 = scramble.clone();
        let puzzle_id = puzzle.id();
        let id = sqlx::query!(
            "INSERT INTO scramble (scramble, puzzle, generated_at) VALUES (?, ?, unixepoch()) RETURNING id",
            scramble2,
            puzzle_id,
        )
        .fetch(db)
        .try_collect::<Vec<_>>()
        .await?
        .first()
        .ok_or(anyhow!("insert failed"))?
        .id;
        Ok(Self {
            id,
            scramble,
            puzzle,
        })
    }
}
