use anyhow::anyhow;
use chrono::NaiveDate;
use log::error;
use rusqlite::{named_params, Connection /*, Transaction*/};
use std::sync::Mutex;

use crate::killmail::Key;
use crate::killmail::Killmail;

pub fn create_connection(url: &str) -> anyhow::Result<Connection> {
    let conn = Connection::open(url)?;
    let _ = conn.execute_batch("
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

    return Ok(conn);
}

pub fn insert(conn: &Mutex<Connection>, killmail: Killmail) -> anyhow::Result<()> {
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

    match conn.try_lock() {
        Ok(conn) => {
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
        Err(what) => {
            error!("Can't lock connection: {what}");
            Err(anyhow!(format!("Can't lock connection: {what}")))
        }
    }
}

pub fn select_ids_by_date(conn: &Mutex<Connection>, date: &NaiveDate) -> anyhow::Result<Vec<Key>> {
    let left = date.format("%Y-%m-%d").to_string();
    let right = date.succ().format("%Y-%m-%d").to_string();
    let sql = format!(
        "SELECT killmail_id FROM killmails WHERE killmail_time BETWEEN '{left}' AND '{right}';"
    );

    match conn.try_lock() {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql)?;
            let mut ids = Vec::new();
            for id in stmt.query_map([], |row| row.get(0))? {
                ids.push(id?);
            }
            Ok(ids)
        }
        Err(what) => {
            error!("Can't lock connection: {what}");
            Err(anyhow!(format!("Can't lock connection: {what}")))
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct RawHistory {
    pub killmail_id: i32,
    pub character_id: i32,
    pub corporation_id: i32,
    pub alliance_id: i32,
    pub ship_type_id: i32,
    pub damage: i32,
    pub is_victim: bool,
}

pub fn character_history(conn: &Mutex<Connection>, id: i32) -> anyhow::Result<Vec<RawHistory>> {
    let sql = format!(
            "SELECT K.killmail_id, character_id, corporation_id, alliance_id, ship_type_id, damage, is_victim, solar_system_id
             FROM participants P JOIN killmails K ON K.killmail_id = P.killmail_id
             WHERE character_id = :id AND killmail_time and killmail_time > date('now','-2 month');"
        );

    match conn.try_lock() {
        Ok(conn) => {
            let mut stmt = conn.prepare(&sql)?;
            let iter = stmt.query_map(&[(":id", &id)], |row| {
                Ok(RawHistory {
                    killmail_id: row.get(0)?,
                    character_id: row.get(1)?,
                    corporation_id: row.get(2)?,
                    alliance_id: row.get(3)?,
                    ship_type_id: row.get(4)?,
                    damage: row.get(5)?,
                    is_victim: row.get(6)?,
                })
            })?;
            Ok(iter.map(|res| res.unwrap()).collect())
        }
        Err(what) => {
            error!("Can't lock connection: {what}");
            Err(anyhow!(format!("Can't lock connection: {what}")))
        }
    }
}
