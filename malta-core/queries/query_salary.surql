SELECT VALUE new_salary FROM (
    SELECT * FROM salary_tracking
        WHERE employee = $employee
            && change_date <= time::group($date, 'day')
            && work_type = $work ORDER change_date DESC LIMIT 1
);