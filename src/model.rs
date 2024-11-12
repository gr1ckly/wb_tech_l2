use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::sync::{Arc, RwLock};
use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, Pool, Postgres, Row};

const SUCCESS_CREATE_EVENT_MESSAGE: &str = "The event was created successfully!";
const SUCCESS_UPDATE_EVENT_MESSAGE: &str = "The event was updated successfully!";
const UNFORTUNATE_UPDATE_EVENT_MESSAGE: &str = "The event wasn't updated!";
const SUCCESS_DELETE_EVENT_MESSAGE: &str = "The event was deleted successfully!";
const UNFORTUNATE_DELETE_EVENT_MESSAGE: &str = "The event wasn't deleted!";

#[derive(FromRow, Clone, Serialize, Deserialize)]
pub struct Event{
    #[serde(default="default_id")]
    id: i64,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    name: String,
    description: String,
}

fn default_id() -> i64{
    0
}

impl Event{
    pub fn build(id: i64, start_time: DateTime<Utc>, end_time: DateTime<Utc>, name: String, description: String) -> Result<Self, String>{
        if name.len() > 255{
            return Err(String::from("The name must be no more than 255 characters long!"))
        }
        if description.len() > 1000{
            return Err(String::from("The description must be no more than 10000 characters long!"))
        }
        Ok(Event{
            id: id,
            start_time: start_time,
            end_time: end_time,
            name: name,
            description: description
        })
    }
    pub fn set_id(&mut self, new_id: i64){
        self.id = new_id;
    }

    pub fn get_id(&self) -> &i64{
        &self.id
    }

    pub fn get_start_time(&self) -> &DateTime<Utc>{
        &self.start_time
    }

    pub fn get_end_time(&self) -> &DateTime<Utc>{
        &self.end_time
    }

    pub fn get_name(&self) -> &String{
        &self.name
    }

    pub fn get_description(&self) -> &String{
        &self.description
    }
}

pub struct Model{
    cache: Arc<RwLock<HashMap<i64, Event>>>,
    pool: Pool<Postgres>
}

impl Model {
    pub async fn init() -> Result<Self, Box<dyn Error>>{
        let db_url = env::var("DATABASE_URL").expect("Couldn't get DATABASE_URL");
        let pool = sqlx::PgPool::connect(&db_url).await.expect("Couldn't connect to database");
        let events_vec = query_as::<_, Event>("SELECT * FROM EVENTS;").fetch_all(&pool).await?;
        let mut events_map = HashMap::new();
        for event in events_vec {
            events_map.insert(event.id, event);
        }
        Ok(
            Model{
                cache: Arc::new(RwLock::new(events_map)),
                pool: pool
            }
        )
    }

    pub async fn update_event(&self, event: Event) -> Result<&str, Box<dyn Error + '_>>{
        let row = query("UPDATE events SET start_time=$1, end_time=$2, name=$3, description=$4 WHERE id=$5").bind(event.get_start_time()).bind(event.get_end_time()).bind(event.get_name()).bind(event.get_description()).bind(event.id).execute(&self.pool).await?;
        if row.rows_affected() != 0 {
            let mut events_map = self.cache.write()?;
            events_map.insert(event.get_id().clone(), event);
            return Ok(SUCCESS_UPDATE_EVENT_MESSAGE)
        }
        Ok(UNFORTUNATE_UPDATE_EVENT_MESSAGE)
    }

    pub async fn add_event(&self, mut event: Event) -> Result<&str, Box<dyn Error + '_>> {
        let mut row = query("INSERT INTO events(start_time, end_time, name, description) VALUES ($1, $2, $3, $4) RETURNING id;").bind(event.get_start_time()).bind(event.get_end_time()).bind(event.get_name()).bind(event.get_description()).fetch_one(&self.pool).await?;
        event.set_id(row.try_get("id")?);
        let mut event_map = self.cache.write()?;
        event_map.insert(event.get_id().clone(), event);
        Ok(SUCCESS_CREATE_EVENT_MESSAGE)
    }

    pub async fn delete_event(&self, id: &i64) -> Result<&str, Box<dyn Error + '_>>{
        let mut row = query("DELETE FROM events WHERE id = $1").bind(id).execute(&self.pool).await?;
        if row.rows_affected() != 0 {
            let mut events_map = self.cache.write()?;
            if let Some(value) = events_map.remove(id){
                return Ok(SUCCESS_DELETE_EVENT_MESSAGE)
            }
        }
        Ok(UNFORTUNATE_DELETE_EVENT_MESSAGE)
    }

    pub async fn get_events_for_day(&self, day:DateTime<Utc>) -> Result<Vec<Event>, Box<dyn Error + '_>>{
        let mut suitable_events = Vec::new();
        let day = day.with_timezone(&Utc);
        for event in self.cache.read()?.values(){
            let curr_event = event.start_time.with_timezone(&Utc);
            let same_year = curr_event.year() == day.year();
            let same_month = curr_event.month() == day.month();
            let same_day = curr_event.day() == day.day();
            if same_year && same_month && same_day{
                suitable_events.push(event.clone());
            }
        }
        Ok(suitable_events)
    }

    pub async fn get_events_for_week(&self, curr_time:DateTime<Utc>) -> Result<Vec<Event>, Box<dyn Error + '_>>{
        let mut suitable_events = Vec::new();
        let curr_time = curr_time.with_timezone(&Utc);
        for event in self.cache.read()?.values(){
            let curr_event = event.start_time.with_timezone(&Utc);
            let same_year = curr_event.year() == curr_time.year();
            let same_month = curr_event.month() == curr_time.month();
            let same_day = curr_event.iso_week() == curr_time.iso_week();
            if same_year && same_month && same_day{
                suitable_events.push(event.clone());
            }
        }
        Ok(suitable_events)
    }

    pub async fn get_events_for_month(&self, curr_time:DateTime<Utc>) -> Result<Vec<Event>, Box<dyn Error + '_>>{
        let mut suitable_events = Vec::new();
        let curr_time = curr_time.with_timezone(&Utc);
        for event in self.cache.read()?.values(){
            let curr_event = event.start_time.with_timezone(&Utc);
            let same_year = curr_event.year() == curr_time.year();
            let same_month = curr_event.month() == curr_time.month();
            if same_year && same_month{
                suitable_events.push(event.clone());
            }
        }
        Ok(suitable_events)
    }
}