use anyhow::anyhow;
use chrono::NaiveDate;
use rusqlite::{named_params, Connection /*, Transaction*/};

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

pub fn select_ids_by_date(conn: &Connection, date: &NaiveDate) -> anyhow::Result<Vec<Key>> {
    let left = date.format("%Y-%m-%d").to_string();
    let right = date.succ().format("%Y-%m-%d").to_string();
    let sql = format!(
        "SELECT killmail_id FROM killmails WHERE killmail_time BETWEEN '{left}' AND '{right}';"
    );

    let mut ids = Vec::new();
    let mut stmt = conn.prepare(&sql)?;
    for id in stmt.query_map([], |row| row.get(0))? {
        ids.push(id?);
    }
    Ok(ids)
}
