use chrono::{NaiveDate, ParseError as ChronoParseError};
use mysql::prelude::*;
use mysql::*;
use std::env;
use thiserror::Error;

pub struct Database {
    pool: Pool,
    some_config: i32,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub first_day_of_week: Option<i32>,
    pub weekly_target_minutes: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct WorkEntries {
    id: Option<u32>,
    date: NaiveDate,
    minutes_worked: i32,
    notes: String,
    created_at: NaiveDate,
    updated_at: NaiveDate,
    year_col: NaiveDate,
    month_col: NaiveDate,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct WorkEntriesQuery {
    pub id: u32,
    pub date: String,
    pub minutes_worked: i32,
    pub notes: String,
    pub created_at: String,
    pub updated_at: String,
    pub year_col: String,
    pub month_col: String,
}

#[derive(Debug, Clone)]
pub struct WorkWeeks {
    id: Option<u32>,
    year: NaiveDate,
    week_number: u32,
    total_minutes: u32,
    updated_at: NaiveDate,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] mysql::Error),

    #[error("Parse error: {0}")]
    ParseError(#[from] ChronoParseError),
}

impl WorkEntries {
    pub fn from_query_result(query_row: WorkEntriesQuery) -> Result<Self, AppError> {
        Ok(WorkEntries {
            id: Some(query_row.id),
            date: NaiveDate::parse_from_str(&query_row.date, "%Y-%m-%d")?,
            minutes_worked: query_row.minutes_worked,
            notes: query_row.notes,
            created_at: NaiveDate::parse_from_str(&query_row.created_at, "%Y-%m-%d")?,
            updated_at: NaiveDate::parse_from_str(&query_row.updated_at, "%Y-%m-%d")?,
            year_col: NaiveDate::parse_from_str(&query_row.year_col, "%Y-%m-%d")?,
            month_col: NaiveDate::parse_from_str(&query_row.month_col, "%Y-%m-%d")?,
        })
    }
}

impl Database {
    pub fn new() -> Result<Self> {
        dotenv::dotenv().expect(".env file not found");
        // Connection params
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");

        let opts = Opts::from_url(&database_url).expect("Unable to parse opts");
        let pool = Pool::new(opts).expect("unable to create pool");

        Ok(Database {
            pool,
            some_config: 42,
        })
    }

    pub fn find_by_id(&self, id: u32) -> Result<Option<WorkEntries>, AppError> {
        let mut conn = self.pool.get_conn().expect("Unable to connect to pool");

        let result_row: Option<WorkEntriesQuery> = conn.exec_first(
            "SELECT id, DATE_FORMAT(date, '%Y-%m-%d'), minutes_worked, notes, DATE_FORMAT(created_at, '%Y-%m-%d'), DATE_FORMAT(updated_at, '%Y-%m-%d'), DATE_FORMAT(year_col, '%Y-%m-%d'), DATE_FORMAT(month_col, '%Y-%m-%d') FROM work_entries WHERE id = ?",
            (id,)
        )?;

        let parsed_result = result_row.map(WorkEntries::from_query_result).transpose()?;
        Ok(parsed_result)
    }
}
