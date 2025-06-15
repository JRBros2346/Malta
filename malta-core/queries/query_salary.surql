SELECT VALUE new_salary FROM (SELECT * FROM salary_log WHERE id[0] = $employee AND id[1] <= time::group($date, 'day') ORDER BY id[1] DESC
 LIMIT 1);