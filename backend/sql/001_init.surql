DEFINE NAMESPACE IF NOT EXISTS virtual_tribe;
USE NS virtual_tribe;

DEFINE DATABASE IF NOT EXISTS geek_store;
USE DB geek_store;

-- Create schemafull creator table;
DEFINE TABLE  IF NOT EXISTS creator SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS username ON TABLE creator TYPE string;
DEFINE FIELD IF NOT EXISTS email ON TABLE creator TYPE string VALUE string::lowercase($value) ASSERT string::is::email($value);
DEFINE FIELD IF NOT EXISTS avatar ON TABLE creator TYPE option<string>;
DEFINE FIELD IF NOT EXISTS description ON TABLE creator TYPE option<string>;
DEFINE FIELD IF NOT EXISTS password ON TABLE creator TYPE string;
DEFINE FIELD IF NOT EXISTS joined_at ON TABLE creator TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS verified ON TABLE creator TYPE bool DEFAULT false PERMISSIONS FOR update WHERE $auth.role="admin";
DEFINE FIELD IF NOT EXISTS socials ON TABLE creator FLEXIBLE TYPE option<object>;
DEFINE FIELD IF NOT EXISTS role ON TABLE creator TYPE string;
DEFINE FIELD IF NOT EXISTS refresh_token ON TABLE creator TYPE option<string>;
DEFINE FIELD IF NOT EXISTS login_attempts ON TABLE creator TYPE number DEFAULT 0;
-- End of creator table;

-- Create schemafull studio table;
DEFINE TABLE IF NOT EXISTS studio SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS name ON TABLE studio TYPE string;
DEFINE FIELD IF NOT EXISTS owner ON TABLE studio TYPE record<creator>;
DEFINE FIELD IF NOT EXISTS email ON TABLE studio TYPE string VALUE string::lowercase($value) ASSERT string::is::email($value);
DEFINE FIELD IF NOT EXISTS description ON TABLE studio TYPE option<string>;

-- DEFINE FUNCTION fn::no_ips($studio: record){
--     TODO: Write function to calculate the number of ip's the studio has
-- }
-- End of studio table;

-- Create schemafull studio_creator table;
DEFINE TABLE IF NOT EXISTS studio_creator SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS studio ON TABLE studio_creator TYPE record<studio>;
DEFINE FIELD IF NOT EXISTS creator ON TABLE studio_creator TYPE record<creator>;
DEFINE FIELD IF NOT EXISTS role ON TABLE studio_creator TYPE string;

-- Create schemafull creator table;
DEFINE TABLE IF NOT EXISTS user SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS username ON TABLE user TYPE string;
DEFINE FIELD IF NOT EXISTS email ON TABLE user TYPE string VALUE string::lowercase($value) ASSERT string::is::email($value);
DEFINE FIELD IF NOT EXISTS avatar ON TABLE user TYPE option<string>;
DEFINE FIELD IF NOT EXISTS password ON TABLE user TYPE string;
DEFINE FIELD IF NOT EXISTS joined_at ON TABLE user TYPE datetime DEFAULT time::now();
DEFINE FIELD IF NOT EXISTS verified ON TABLE user TYPE bool DEFAULT false PERMISSIONS FOR update WHERE $auth.role="admin";
DEFINE FIELD IF NOT EXISTS refresh_token ON TABLE user TYPE option<string>;
DEFINE FIELD IF NOT EXISTS login_attempts ON TABLE user TYPE number DEFAULT 0;
-- End of creator table;