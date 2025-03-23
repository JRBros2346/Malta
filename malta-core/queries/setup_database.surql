DEFINE TABLE IF NOT EXISTS project SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS name ON project TYPE string;
DEFINE FIELD IF NOT EXISTS estimate ON project TYPE option<decimal>;

DEFINE TABLE IF NOT EXISTS income SCHEMAFULL;
-- DEFINE FIELD IF NOT EXISTS on_date ON income TYPE datetime
--     VALUE time::group($value, 'day');
-- DEFINE FIELD IF NOT EXISTS reason ON income TYPE record<project> | string;
DEFINE FIELD IF NOT EXISTS source ON income TYPE decimal;

DEFINE TABLE IF NOT EXISTS expense SCHEMAFULL;
-- DEFINE FIELD IF NOT EXISTS on_date ON expense TYPE datetime;
-- DEFINE FIELD IF NOT EXISTS project ON expense TYPE option<record<project>>;
DEFINE FIELD IF NOT EXISTS reason ON expense TYPE string;
DEFINE FIELD IF NOT EXISTS amount ON expense TYPE decimal;

DEFINE TABLE IF NOT EXISTS employee SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS name ON employee TYPE string;

DEFINE TABLE IF NOT EXISTS salary_tracking SCHEMAFULL;
-- DEFINE FIELD IF NOT EXISTS employee ON salary_tracking TYPE record<employee>;
-- DEFINE FIELD IF NOT EXISTS change_date ON salary_tracking TYPE datetime
--     VALUE time::group($value, 'day');
-- DEFINE FIELD IF NOT EXISTS work_type ON salary_tracking TYPE int
--     ASSERT $value IN [1, 2, 4];
DEFINE FIELD IF NOT EXISTS new_salary ON salary_tracking TYPE decimal;

DEFINE TABLE IF NOT EXISTS worked_in TYPE RELATION IN employee OUT project SCHEMAFULL;
-- DEFINE FIELD IF NOT EXISTS on_date ON worked_in TYPE datetime
    -- VALUE time::group($value, 'day');
DEFINE FIELD IF NOT EXISTS work_type ON worked_in TYPE int
    ASSERT $value IN [1, 2, 4];

DEFINE TABLE IF NOT EXISTS payment SCHEMAFULL;
-- DEFINE FIELD IF NOT EXISTS employee ON payment TYPE record<employee>;
-- DEFINE FIELD IF NOT EXISTS on_date ON payment TYPE datetime
--     VALUE time::group($value, 'day');
DEFINE FIELD IF NOT EXISTS amount ON payment TYPE decimal;

DEFINE TABLE IF NOT EXISTS tool SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS name ON tool TYPE string;
DEFINE FIELD IF NOT EXISTS adjectives ON tool TYPE set<string>;

DEFINE TABLE IF NOT EXISTS cost_tracking SCHEMAFULL;
-- DEFINE FIELD IF NOT EXISTS tool ON cost_tracking TYPE record<tool>;
-- DEFINE FIELD IF NOT EXISTS change_date ON cost_tracking TYPE datetime
    -- VALUE time::group($value, 'day');
DEFINE FIELD IF NOT EXISTS new_cost ON cost_tracking TYPE decimal;

DEFINE TABLE IF NOT EXISTS taken_by TYPE RELATION IN tool OUT employee SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS from_time ON taken_by TYPE datetime;
DEFINE FIELD IF NOT EXISTS duration ON taken_by TYPE duration;
DEFINE FIELD IF NOT EXISTS to_project ON taken_by TYPE option<record<project>>;

