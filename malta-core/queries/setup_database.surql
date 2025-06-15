DEFINE TABLE IF NOT EXISTS project SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS name ON project TYPE string;
DEFINE FIELD IF NOT EXISTS estimate ON project TYPE option<decimal>;

DEFINE TABLE IF NOT EXISTS income SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS id ON income TYPE [datetime, record<project> | string];
DEFINE FIELD IF NOT EXISTS source ON income TYPE decimal;

DEFINE TABLE IF NOT EXISTS expense SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS id ON expense TYPE [datetime, option<record<project>>];
DEFINE FIELD IF NOT EXISTS reason ON expense TYPE string;
DEFINE FIELD IF NOT EXISTS amount ON expense TYPE decimal;

DEFINE TABLE IF NOT EXISTS employee SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS name ON employee TYPE string;

DEFINE TABLE IF NOT EXISTS salary_log SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS id ON salary_log TYPE [record<employee>, datetime];
DEFINE FIELD IF NOT EXISTS new_salary ON salary_log TYPE decimal;

DEFINE TABLE IF NOT EXISTS work_log /*TYPE RELATION IN employee OUT project*/ SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS id ON work_log TYPE [record<employee>, 1 | 2 | 3 | 4, record<project>, datetime];

DEFINE TABLE IF NOT EXISTS pay_log SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS id ON pay_log TYPE [record<employee>, datetime];
DEFINE FIELD IF NOT EXISTS amount ON pay_log TYPE decimal;

DEFINE TABLE IF NOT EXISTS tool SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS name ON tool TYPE string;
DEFINE FIELD IF NOT EXISTS adjectives ON tool TYPE set<string>;

DEFINE TABLE IF NOT EXISTS cost_log SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS id ON cost_log TYPE [record<tool>, datetime];
DEFINE FIELD IF NOT EXISTS new_cost ON cost_log TYPE decimal;

DEFINE TABLE IF NOT EXISTS took_log TYPE RELATION IN tool OUT employee SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS from_time ON took_log TYPE datetime;
DEFINE FIELD IF NOT EXISTS duration ON took_log TYPE duration;
DEFINE FIELD IF NOT EXISTS to_project ON took_log TYPE option<record<project>>;
