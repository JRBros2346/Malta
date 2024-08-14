DEFINE TABLE project SCHEMAFULL;
DEFINE FIELD name ON project TYPE string;
DEFINE FIELD estimate ON project TYPE option<decimal>;

DEFINE TABLE employee SCHEMAFULL;
DEFINE FIELD name ON employee TYPE string;

DEFINE TABLE salary_tracking SCHEMAFULL;
DEFINE FIELD employee ON salary_tracking TYPE record<employee>;
DEFINE FIELD change_date ON salary_tracking TYPE datetime
    VALUE time::group($value, 'day');
DEFINE FIELD work_type ON salary_tracking TYPE int
    ASSERT $value IN [1, 2, 4];
DEFINE FIELD new_salary ON salary_tracking TYPE decimal;

DEFINE TABLE worked_in TYPE RELATION IN employee OUT project SCHEMAFULL;
DEFINE FIELD on_date ON worked_in TYPE datetime
    VALUE time::group($value, 'day');
DEFINE FIELD work_type ON worked_in TYPE int
    ASSERT $value IN [1, 2, 4];

DEFINE TABLE payment SCHEMAFULL;
DEFINE FIELD employee ON payment TYPE record<employee>;
DEFINE FIELD on_date ON payment TYPE datetime
    VALUE time::group($value, 'day');
DEFINE FIELD amount ON payment TYPE decimal;

DEFINE TABLE tool SCHEMAFULL;
DEFINE FIELD name ON tool TYPE string;
DEFINE FIELD adjectives ON tool TYPE set<string>;

DEFINE TABLE cost_tracking SCHEMAFULL;
DEFINE FIELD tool ON cost_tracking TYPE record<tool>;
DEFINE FIELD change_date ON cost_tracking TYPE datetime
    VALUE time::group($value, 'day');
DEFINE FIELD new_cost ON cost_tracking TYPE decimal;

DEFINE TABLE taken_by TYPE RELATION IN tool OUT employee SCHEMAFULL;
DEFINE FIELD from_time ON taken_by TYPE datetime;
DEFINE FIELD duration ON taken_by TYPE duration;
DEFINE FIELD to_project ON taken_by TYPE option<record<project>>;

DEFINE TABLE expense SCHEMAFULL;
DEFINE FIELD on_date ON expense TYPE datetime;
DEFINE FIELD project ON expense TYPE option<record<project>>;
DEFINE FIELD reason ON expense TYPE string;
DEFINE FIELD amount ON expense TYPE decimal;

-- DEFINE FUNCTION fn::get_salary($employee: record<employee>, $date: datetime, $work: int) {
--     LET $out = (SELECT VALUE new_salary FROM (SELECT * FROM salary_tracking WHERE employee = $employee && change_date <= time::group($date, 'day') && work_type = $work ORDER change_date DESC LIMIT 1));
--     RETURN IF $out=[] THEN
--         0dec
--     ELSE
--         array::first($out)
--     END;
-- };
-- DEFINE FUNCTION fn::get_details($tool: record<tool>, $time: datetime) {
--     LET $cost = array::first(SELECT VALUE new_cost FROM (SELECT * FROM cost_tracking WHERE tool = $tool && change_date <= time::group($time, 'day') ORDER change_date DESC LIMIT 1));
--     LET $loc = array::first(SELECT * FROM taken_by WHERE in = $tool && from_time<=$time ORDER from_time DESC LIMIT 1);
--     RETURN {
--         cost: (IF $cost=NONE THEN 0dec ELSE $cost END),
--         loc: (IF $loc=NONE THEN NONE ELSE IF $time-$loc.from_time < $loc.duration THEN $loc.out ELSE $loc.to_project END)
--     };
-- };