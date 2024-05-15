DEFINE NAMESPACE IF NOT EXISTS virtual_tribe;
USE NS virtual_tribe;

DEFINE DATABASE IF NOT EXISTS geek_store;
USE DB geek_store;

-- Create schemafull creator table;
DEFINE TABLE IF NOT EXISTS creator SCHEMAFULL;
DEFINE FIELD username ON TABLE creator TYPE string;
DEFINE FIELD email ON TABLE creator TYPE string VALUE string::lowercase($value) ASSERT string::is::email($value);
DEFINE FIELD avatar ON TABLE creator TYPE option<string>;
DEFINE FIELD description ON TABLE creator TYPE option<string>;
DEFINE FIELD password ON TABLE creator TYPE string;
DEFINE FIELD joined_at ON TABLE creator TYPE datetime DEFAULT time::now();
DEFINE FIELD verified ON TABLE creator TYPE bool DEFAULT false PERMISSIONS FOR update WHERE $auth.role="admin";
DEFINE FIELD socials ON TABLE creator FLEXIBLE TYPE option<object>;
DEFINE FIELD role ON TABLE creator TYPE array<string>;
DEFINE FIELD refresh_token ON TABLE creator TYPE option<string>;
DEFINE FIELD login_attempts ON TABLE creator TYPE number DEFAULT 0;
-- End of creator table;

-- Create schemafull studio table;
DEFINE TABLE IF NOT EXISTS studio SCHEMAFULL;
DEFINE FIELD name ON TABLE studio TYPE string;
DEFINE FIELD owner ON TABLE studio TYPE record<creator>;
DEFINE FIELD email ON TABLE studio TYPE string VALUE string::lowercase($value) ASSERT string::is::email($value);
DEFINE FIELD description ON TABLE studio TYPE option<string>;

-- DEFINE FUNCTION fn::no_ips($studio: record){
--     TODO: Write function to calculate the number of ip's the studio has
-- }
-- End of studio table;

-- Create schemafull studio_creator table;
DEFINE TABLE IF NOT EXISTS studio_creator SCHEMAFULL;
DEFINE FIELD studio ON TABLE studio_creator TYPE record<studio>;
DEFINE FIELD creator ON TABLE studio_creator TYPE record<creator>;
DEFINE FIELD role ON TABLE studio TYPE string;

-- Create schemafull creator table;
DEFINE TABLE IF NOT EXISTS user SCHEMAFULL;
DEFINE FIELD username ON TABLE user TYPE string;
DEFINE FIELD email ON TABLE user TYPE string VALUE string::lowercase($value) ASSERT string::is::email($value);
DEFINE FIELD avatar ON TABLE user TYPE option<string>;
DEFINE FIELD password ON TABLE user TYPE string;
DEFINE FIELD joined_at ON TABLE user TYPE datetime DEFAULT time::now();
DEFINE FIELD verified ON TABLE user TYPE bool DEFAULT false PERMISSIONS FOR update WHERE $auth.role="admin";
DEFINE FIELD refresh_token ON TABLE user TYPE option<string>;
DEFINE FIELD login_attempts ON TABLE user TYPE number DEFAULT 0;
-- End of creator table;