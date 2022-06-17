
SELECT
	character_id,
	count(character_id),
	(SELECT count(character_id) FROM participants P WHERE is_victim = 0 AND P.character_id = O.character_id) AS wins,
	(SELECT count(character_id) FROM participants P WHERE is_victim = 1 AND P.character_id = O.character_id) AS losses
FROM participants O
GROUP BY 1
ORDER BY 2 DESC


SELECT K.killmail_id, K.killmail_time
FROM participants P JOIN killmails K ON K.killmail_id = P.killmail_id
WHERE
	character_id = 2114209292

SELECT strftime('%H', K.killmail_time), count(K.killmail_id)
FROM participants P JOIN killmails K ON K.killmail_id = P.killmail_id
WHERE character_id = 2114209292
GROUP BY 1
ORDER BY 1 DESC
