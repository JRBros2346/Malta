SELECT id[1] AS source, id[0] AS on_date, amount FROM income WHERE type::is::string(id[1]) ORDER BY id[0] DESC
;