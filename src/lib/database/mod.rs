use anyhow::anyhow;
use chrono::NaiveDate;

use r2d2;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{named_params, Connection};

use crate::killmail::Killmail;

pub type SqlitePool = r2d2::Pool<SqliteConnectionManager>;

pub fn create_pool(url: &str) -> anyhow::Result<SqlitePool> {
    let manager = SqliteConnectionManager::file(url);
    let pool = r2d2::Pool::new(manager).unwrap();
    let conn = pool.get().unwrap();
    conn.execute_batch("
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS killmails(
            killmail_id INTEGER NOT NULL PRIMARY KEY,
            killmail_time TEXT NOT NULL,
            solar_system_id INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS killmail_time_idx ON killmails(killmail_time);

        CREATE TABLE IF NOT EXISTS participants(
            killmail_id INTEGER NOT NULL,
            character_id INTEGER,
            corporation_id INTEGER,
            alliance_id INTEGER,
            ship_type_id INTEGER,
            damage INTEGER NOT NULL,
            is_victim INTEGER NOT NULL,
            UNIQUE(killmail_id, character_id, is_victim),
            FOREIGN KEY(killmail_id) REFERENCES killmails(killmail_id)
        );
        CREATE INDEX IF NOT EXISTS participant_idx ON participants(character_id, corporation_id, alliance_id);
    ").map_err(|e| anyhow!(e))?;

    return Ok(pool);
}

pub fn insert(conn: &Connection, killmail: Killmail) -> anyhow::Result<()> {
    const INSERT_KILLMAIL: &str = r"INSERT OR IGNORE INTO killmails VALUES (
        :killmail_id,
        :killmail_time,
        :solar_system_id)";

    const INSERT_PARTICIPANT: &str = r"INSERT OR IGNORE INTO participants VALUES (
        :killmail_id,
        :character_id,
        :corporation_id,
        :alliance_id,
        :ship_type_id,
        :damage,
        :is_victim)";

    let mut insert_killmail_stmt = conn.prepare(INSERT_KILLMAIL)?;
    let mut insert_participant_stmt = conn.prepare(INSERT_PARTICIPANT)?;

    insert_killmail_stmt.execute(named_params! {
        ":killmail_id": killmail.killmail_id,
        ":killmail_time": killmail.killmail_time,
        ":solar_system_id": killmail.solar_system_id
    })?;

    let victim = killmail.victim;
    insert_participant_stmt.execute(named_params! {
        ":killmail_id": killmail.killmail_id,
        ":character_id": victim.character_id,
        ":corporation_id": victim.corporation_id,
        ":alliance_id": victim.alliance_id,
        ":ship_type_id": victim.ship_type_id,
        ":damage": victim.damage_taken,
        ":is_victim": 1
    })?;

    for attacker in killmail.attackers {
        insert_participant_stmt.execute(named_params! {
            ":killmail_id": killmail.killmail_id,
            ":character_id": attacker.character_id,
            ":corporation_id": attacker.corporation_id,
            ":alliance_id": attacker.alliance_id,
            ":ship_type_id": attacker.ship_type_id,
            ":damage": attacker.damage_done,
            ":is_victim": 0
        })?;
    }

    Ok(())
}

pub fn select_ids_by_date(conn: &Connection, date: &NaiveDate) -> anyhow::Result<Vec<i32>> {
    let left = date.format("%Y-%m-%d").to_string();
    let right = date.succ().format("%Y-%m-%d").to_string();
    let sql = format!(
        "SELECT killmail_id FROM killmails WHERE killmail_time BETWEEN '{left}' AND '{right}';"
    );

    let mut stmt = conn.prepare(&sql)?;
    let mut ids = Vec::new();
    for id in stmt.query_map([], |row| row.get(0))? {
        ids.push(id?);
    }
    Ok(ids)
}

#[derive(Debug)]
pub struct RawHistory {
    pub killmail_id: i32,
    pub character_id: Option<i32>,
    pub corporation_id: Option<i32>,
    pub alliance_id: Option<i32>,
    pub ship_type_id: Option<i32>,
    pub damage: i32,
    pub is_victim: bool,
    pub solar_system_id: i32,
}

pub fn character_history(conn: &Connection, id: i32) -> anyhow::Result<Vec<RawHistory>> {
    let sql = format!(
            "SELECT K.killmail_id, character_id, corporation_id, alliance_id, ship_type_id, damage, is_victim, solar_system_id
             FROM participants P JOIN killmails K ON K.killmail_id = P.killmail_id
             WHERE character_id = {id} AND killmail_time and killmail_time > date('now','-30 days');"
        );

    let mut stmt = conn.prepare(&sql)?;
    let iter = stmt.query_map([], |row| {
        Ok(RawHistory {
            killmail_id: row.get(0)?,
            character_id: row.get(1)?,
            corporation_id: row.get(2)?,
            alliance_id: row.get(3)?,
            ship_type_id: row.get(4)?,
            damage: row.get(5)?,
            is_victim: row.get(6)?,
            solar_system_id: row.get(7)?,
        })
    })?;
    Ok(iter.map(|res| res.unwrap()).collect())
}

#[derive(Debug, PartialEq)]
pub enum RelationType {
    FriendsChar,
    EnemiesChar,
    FriendsCorp,
    EnemiesCorp,
    FriendsAlli,
    EnemiesAlli,
}
impl RelationType {
    fn get_field(relation: &RelationType) -> &'static str {
        match relation {
            RelationType::FriendsChar => "character_id",
            RelationType::EnemiesChar => "character_id",
            RelationType::FriendsCorp => "corporation_id",
            RelationType::EnemiesCorp => "corporation_id",
            RelationType::FriendsAlli => "alliance_id",
            RelationType::EnemiesAlli => "alliance_id",
        }
    }
    fn get_victim_value(relation: &RelationType) -> i16 {
        match relation {
            RelationType::FriendsChar => 0,
            RelationType::EnemiesChar => 1,
            RelationType::FriendsCorp => 0,
            RelationType::EnemiesCorp => 1,
            RelationType::FriendsAlli => 0,
            RelationType::EnemiesAlli => 1,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RelationSubject {
    Character,
    Corporation,
    Alliance,
}
impl RelationSubject {
    fn get_field(relation: &Self) -> &'static str {
        match relation {
            RelationSubject::Character => "character_id",
            RelationSubject::Corporation => "corporation_id",
            RelationSubject::Alliance => "alliance_id",
        }
    }
}

pub type RawRelation = (i32, usize);

pub fn relations(
    conn: &Connection,
    id: i32,
    sbj: RelationSubject,
    rel: RelationType,
) -> anyhow::Result<Vec<RawRelation>> {
    let object_field = RelationSubject::get_field(&sbj);
    let relation_field = RelationType::get_field(&rel);
    let victum_value = RelationType::get_victim_value(&rel);
    let sql = format!(
        "WITH RECURSIVE character_killmails(id) AS (
           SELECT K.killmail_id
	       FROM participants P JOIN killmails K ON K.killmail_id = P.killmail_id
	       WHERE {object_field} = {id} AND is_victim = {victum_value} AND killmail_time > date('now','-30 days')
        )
        SELECT {relation_field} AS id, count(id) AS times
        FROM character_killmails JOIN participants ON id = killmail_id
        WHERE {object_field} <> {id}
        GROUP BY 1;");
    let mut stmt = conn.prepare(&sql)?;
    let iter = stmt.query_map([], |row| Ok((row.get(0).unwrap_or_default(), row.get(1)?)))?;
    Ok(iter
        .map(|res| res.unwrap())
        .filter(|(id, _)| *id != 0)
        .collect())
}
