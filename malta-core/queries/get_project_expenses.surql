SELECT id[0] AS on_date, reason, amount FROM expense WHERE id[1] = $project ORDER BY id[0] DESC
;