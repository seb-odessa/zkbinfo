
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Killmail {
    pub killmail_id: i32,
    pub killmail_time: String,
    pub solar_system_id: i32,
    pub victim: Victim,
    pub attackers: Vec<Attackers>,
    pub zkb: Option<Zkb>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Attackers {
    pub alliance_id: Option<i32>,
    pub character_id: Option<i32>,
    pub corporation_id: Option<i32>,
    pub damage_done: i32,
    pub ship_type_id: Option<i32>,
    pub weapon_type_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Victim {
    pub alliance_id: Option<i32>,
    pub character_id: Option<i32>,
    pub corporation_id: Option<i32>,
    pub damage_taken: i32,
    pub ship_type_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Zkb {
    pub hash: String,
}

#[cfg(test)]
mod tests_killmail {
    use super::*;

    #[test]
    fn test_killmail_deserialize() {
        let json = r#"
        {
        "attackers": [
            {
                "alliance_id": 99010832,
                "character_id": 2116032618,
                "corporation_id": 98676166,
                "damage_done": 8076,
                "final_blow": true,
                "security_status": -2.1,
                "ship_type_id": 17728,
                "weapon_type_id": 2446
            }
        ],
        "killmail_id": 97318112,
        "killmail_time": "2021-12-12T15:46:42Z",
        "solar_system_id": 30001438,
        "victim": {
            "alliance_id": 933731581,
            "character_id": 308241937,
            "corporation_id": 98052179,
            "damage_taken": 115352,
            "items": [
                {
                    "flag": 28,
                    "item_type_id": 24515,
                    "quantity_destroyed": 25,
                    "singleton": 0
                }
            ],
            "position": {
                "x": -249633174755.42352,
                "y": 191130500380.3102,
                "z": 192467434893.65863
            },
            "ship_type_id": 47466
        },
        "zkb": {
            "locationID": 30001438,
            "hash": "9377f28e34eabc18162e57e7e85f7a15c9339604",
            "fittedValue": 1327809.86,
            "droppedValue": 160905.63,
            "destroyedValue": 1241817.19,
            "totalValue": 1402722.82,
            "points": 1,
            "npc": false,
            "solo": true,
            "awox": false,
            "esi": "https:\/\/esi.evetech.net\/latest\/killmails\/97318112\/9377f28e34eabc18162e57e7e85f7a15c9339604\/",
            "url": "https:\/\/zkillboard.com\/kill\/97318112\/"
        }
        }"#;

        let maybe_killmail = serde_json::from_str::<Killmail>(json);
        assert!(maybe_killmail.is_ok());
        let killmail = maybe_killmail.unwrap();

        assert_eq!(killmail.killmail_id, 97318112);
        assert_eq!(killmail.attackers.len(), 1);
        assert_eq!(killmail.attackers[0].character_id, Some(2116032618));
        assert_eq!(killmail.victim.character_id, Some(308241937));
        assert!(killmail.zkb.is_some());
        assert_eq!(
            killmail.zkb.unwrap().hash,
            String::from("9377f28e34eabc18162e57e7e85f7a15c9339604")
        );
    }
}
