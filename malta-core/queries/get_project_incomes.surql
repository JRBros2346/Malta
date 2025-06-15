SELECT id[1] AS source, id[0] AS on_date, amount FROM income WHERE type::is::record(id[1], 'project') ORDER BY id[0] DESC
;