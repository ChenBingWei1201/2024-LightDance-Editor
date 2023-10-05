use dotenv::var;
use sqlx::MySqlPool;
use std::time::Instant;

#[derive(Debug, sqlx::FromRow)]
struct LED {
    color_id: i32,
    alpha: i32,
}

#[tokio::test]
pub async fn setup_benchmark() -> Result<(), sqlx::Error> {
    let host = var("MYSQL_TEST_URL").expect("MYSQL_TEST_URL is not set");
    let pool = MySqlPool::connect(host.as_str())
        .await
        .expect("Failed to create mysql pool");

    let start = Instant::now();
    for i in 1..=1000 {
        let mut leds_json_array: Vec<(i32, i32)> = Vec::new();
        let mut leds: Vec<(i32, i32, i32, i32)> = Vec::new();
        for j in 1..=20 {
            leds_json_array.push((0, j));
            leds.push((i, j, 0, j));
        }
        let leds_json = serde_json::to_string(&leds_json_array).unwrap();

        let _ = sqlx::query(
            r#"
                INSERT INTO `LEDEffect` (`name`, `part_name`, `repeat`, `frames`)
                VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(i.to_string())
        .bind("test")
        .bind(1)
        .bind(leds_json)
        .execute(&pool)
        .await?;

        let _ = sqlx::query(
            r#"
                INSERT INTO `LEDEffectTest` (`name`, `part_name`, `repeat`)
                VALUES (?, ?, ?)
            "#,
        )
        .bind(i.to_string())
        .bind("test")
        .bind(1)
        .execute(&pool)
        .await?;

        for (effect_id, position, color_id, alpha) in leds {
            let _ = sqlx::query(
                r#"
                    INSERT INTO `LED` (`effect_id`, `position`, `color_id`, `alpha`)
                    VALUES (?, ?, ?, ?)
                "#,
            )
            .bind(effect_id)
            .bind(position)
            .bind(color_id)
            .bind(alpha)
            .execute(&pool)
            .await?;
        }
    }
    println!("Inserting 100 effects took {:?}", start.elapsed());

    Ok(())
}

// #[sqlx::test(migrations = "tests/migrations")]
// pub async fn run(pool: sqlx::MySqlPool) -> sqlx::Result<()> {
#[tokio::test]
pub async fn run_benchmark() -> Result<(), sqlx::Error> {
    let host = var("MYSQL_TEST_URL").expect("MYSQL_TEST_URL is not set");
    let pool = MySqlPool::connect(host.as_str())
        .await
        .expect("Failed to create mysql pool");

    let start = Instant::now();
    let led_data = sqlx::query_as::<_, LED>(
        r#"
            SELECT color_id, alpha FROM LED
            WHERE effect_id = 1
            ORDER BY position ASC;
        "#,
    )
    .fetch_all(&pool)
    .await?;
    println!("Fetched {:?} rows", led_data.len());
    println!("Fetching LEDs by table took {:?}", start.elapsed());

    let start = Instant::now();
    let mut led_data_from_json = Vec::<LED>::new();
    let frames = sqlx::query_scalar::<_, sqlx::types::JsonValue>(
        r#"
            SELECT frames FROM LEDEffect WHERE id = 1 LIMIT 1;
        "#,
    )
    .fetch_one(&pool)
    .await?;
    if let Some(frames) = frames.as_array() {
        for frame in frames {
            if let Some(leds) = frame.as_array() {
                led_data_from_json.push(LED {
                    color_id: leds[0].as_i64().unwrap() as i32,
                    alpha: leds[1].as_i64().unwrap() as i32,
                })
            }
        }
    }
    println!("Fetched {:?} rows", led_data_from_json.len());
    println!("Fetching LEDs by JSON took {:?}", start.elapsed());

    // println!("Effect data: {:?}", led_data_from_json);

    Ok(())
}
