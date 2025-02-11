use std::env;
use crate::core::types::Trade;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn get_db_pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(env::var("DATABASE_URL").expect("DATABASE_URL not found").as_str())
        .await
        .expect("Failed to connect to database")
}

pub async fn save_trade(trade: &Trade) {
    let pool = get_db_pool().await;
    sqlx::query!(
        "INSERT INTO trades (id, timestamp, amount, price, side, symbol, status, fee, liquidated, liquidation_price)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        trade.id as i64,
        trade.timestamp.to_rfc3339(),
        trade.amount,
        trade.price,
        format!("{:?}", trade.side),
        trade.symbol,
        trade.status,
        trade.fee,
        trade.liquidated,
        trade.liquidation_price
    )
        .execute(&pool)
        .await
        .expect("Failed to insert trade into database");
}

pub async fn load_trade_history() -> Vec<Trade> {
    let pool = get_db_pool().await;
    let rows = sqlx::query!(
        "SELECT id, timestamp, amount, price, side, symbol, status, fee, liquidated, liquidation_price FROM trades"
    )
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch trades");

    rows.iter()
        .map(|row| Trade {
            id: row.id as u64,
            timestamp: chrono::DateTime::parse_from_rfc3339(&row.timestamp)
                .unwrap()
                .with_timezone(&chrono::Utc),
            amount: row.amount,
            price: row.price,
            side: match row.side.as_str() {
                "Buy" => crate::core::types::OrderSide::Buy,
                "Sell" => crate::core::types::OrderSide::Sell,
                _ => crate::core::types::OrderSide::Hold,
            },
            symbol: row.symbol.clone(),
            status: row.status.clone(),
            fee: row.fee.clone(),
            liquidated: row.liquidated,
            liquidation_price: row.liquidation_price,
            leverage: None,
            exchange_data: crate::core::types::ExchangeData {
                name: Some("Database".to_string()),
                exchange_id: "db".to_string(),
                exchange_url: None,
                exchange_logo: None,
                exchange_name: Some("Database".to_string()),
                exchange_timezone: Some("UTC".to_string()),
                exchange_timezone_name: Some("UTC".to_string()),
            },
        })
        .collect()
}
