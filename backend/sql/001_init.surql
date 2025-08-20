DEFINE NAMESPACE IF NOT EXISTS virtual_tribe;
USE NS virtual_tribe;

DEFINE DATABASE IF NOT EXISTS geek_store;
USE DB geek_store;

-- Create schemafull creator table;
DEFINE TABLE  IF NOT EXISTS creator SCHEMAFULL;
DEFINE FIELD username ON TABLE creator TYPE string;
DEFINE FIELD email ON TABLE creator TYPE string VALUE string::lowercase($value) ASSERT string::is::email($value);
DEFINE FIELD avatar ON TABLE creator TYPE option<string>;
DEFINE FIELD description ON TABLE creator TYPE option<string>;
DEFINE FIELD password ON TABLE creator TYPE string;
DEFINE FIELD joined_at ON TABLE creator TYPE datetime DEFAULT time::now();
DEFINE FIELD verified ON TABLE creator TYPE bool DEFAULT false PERMISSIONS FOR update WHERE $auth.role="admin";
DEFINE FIELD socials ON TABLE creator FLEXIBLE TYPE option<object>;
DEFINE FIELD role ON TABLE creator TYPE string;
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
DEFINE FIELD role ON TABLE studio_creator TYPE string;

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


-- region:      -- Genre table
DEFINE TABLE IF NOT EXISTS genre SCHEMAFULL;
DEFINE FIELD name ON TABLE genre TYPE string ASSERT string::len($value) <= 30;
DEFINE FIELD description ON TABLE genre TYPE string ASSERT string::len($value) <= 1000;
-- INSERT INTO genre(name, description) VALUES ("Fantasy", "A genre of speculative fiction which involves themes of the supernatural, magic, and imaginary worlds and creatures.");
-- INSERT INTO genre(name, description) VALUES ("Sci-Fi", "A genre that explores the future through the lens of science and technology");
-- INSERT INTO genre(name, description) VALUES ("Superheroe", "A subgenre of speculative fiction that features costumed crime fighters with extraordinary powers.");
-- endregion:   -- Genre table

DEFINE TABLE IF NOT EXISTS content SCHEMAFULL;
DEFINE FIELD title ON TABLE content TYPE string;
DEFINE FIELD rating ON TABLE content TYPE float DEFAULT 0.0f;
DEFINE FIELD studio ON TABLE content TYPE record<studio>;
DEFINE FIELD description ON TABLE content TYPE string ;
DEFINE FIELD audiences ON TABLE content TYPE string;
DEFINE FIELD recom_price ON TABLE content TYPE float;
DEFINE FIELD genre ON TABLE content TYPE array<record<genre>;
DEFINE FIELD cover ON TABLE content TYPE option<string>;


-- region:      -- Comic table
DEFINE TABLE IF NOT EXISTS comic SCHEMAFULL;
DEFINE FIELD volumes ON TABLE comic TYPE number DEFAULT 0;
DEFINE FIELD chapters ON TABLE comic TYPE number DEFAULT 0;
DEFINE FIELD writer ON TABLE comic TYPE array<string>;
DEFINE FIELD artist ON TABLE comic TYPE array<string>;
DEFINE FIELD creator ON TABLE comic TYPE array<record<creator>>;
DEFINE FIELD created_at ON TABLE comic TYPE datetime DEFAULT time::now();
DEFINE FIELD cover ON TABLE comic TYPE option<string>;
DEFINE FIELD content ON TABLE comic TYPE record<content>;

-- Volume
DEFINE TABLE IF NOT EXISTS volume SCHEMAFULL;
DEFINE FIELD vol_no ON TABLE volume TYPE number; -- Make it so that it is automatically calculated using array length +1
DEFINE FIELD no_of_chapters ON TABLE volume TYPE number DEFAULT 0;
DEFINE FIELD runlength ON TABLE volume TYPE option<object>;
DEFINE FIELD synopsis ON TABLE volume TYPE string ASSERT string::len($value) <= 1000 ;
DEFINE FIELD comic ON TABLE volume TYPE record<comic>;
DEFINE FIELD cover ON TABLE volume TYPE option<string>;

DEFINE FUNCTION fn::calc_vol_no(){
    RETURN (SELECT count() FROM volume)[0].count;
}

DEFINE EVENT set_vol_no ON volume WHEN $event = "CREATE" THEN (
    UPDATE $this SET vol_no = fn::calc_vol_no();
)
-- Chapter
DEFINE TABLE IF NOT EXISTS chapter SCHEMAFULL;
DEFINE FIELD relative_chapter ON TABLE chapter TYPE number;
DEFINE FIELD absolute_chapter ON TABLE chapter TYPE number;
DEFINE FIELD pages ON TABLE chapter TYPE number;
DEFINE FIELD synopsis ON TABLE chapter TYPE string ASSERT string::len($value) <= 700 ;
DEFINE FIELD title ON TABLE chapter TYPE string ASSERT string::len($value) <=  100 ;
DEFINE FIELD file ON TABLE chapter TYPE option<string>;
DEFINE FIELD cover ON TABLE chapter TYPE option<string>;
DEFINE FIELD volume ON TABLE chapter TYPE record<volume>;
DEFINE FIELD created_at ON TABLE chapter TYPE datetime DEFAULT time::now();

-- endregion:   -- Comic table


-- region:      --- Video
DEFINE TABLE IF NOT EXISTS video SCHEMAFULL;
DEFINE FIELD seasons ON TABLE video TYPE number DEFAULT 0;
DEFINE FIELD episodes ON TABLE video TYPE number DEFAULT 0;
DEFINE FIELD writer ON TABLE video TYPE array<string>;
DEFINE FIELD creator ON TABLE video TYPE array<record<creator>>;
DEFINE FIELD average_run_length ON TABLE video TYPE object;
DEFINE FIELD created_at ON TABLE video TYPE datetime DEFAULT time::now();
DEFINE FIELD cover ON TABLE video TYPE option<string>;
DEFINE FIELD trailer ON TABLE video TYPE option<string>;
DEFINE FIELD video_type ON TABLE video TYPE string; -- Animation OR Live Action
DEFINE FIELD content ON TABLE video TYPE record<content>;

-- Season
DEFINE TABLE IF NOT EXISTS season SCHEMAFULL;
DEFINE FIELD no_of_episodes ON TABLE season TYPE number DEFAULT 0;
DEFINE FIELD episodes_available ON TABLE season TYPE number DEFAULT 0 ;
DEFINE FIELD season_no ON TABLE season TYPE number DEFAULT 0;
DEFINE FIELD runlength ON TABLE season TYPE option<object>;
DEFINE FIELD synopsis ON TABLE season TYPE string ASSERT string::len($value) <= 1000 ;
DEFINE FIELD video ON TABLE season TYPE record<video>;
DEFINE FIELD cover ON TABLE season TYPE option<string>;

--Episode
DEFINE TABLE IF NOT EXISTS episode SCHEMAFULL;
DEFINE FIELD relative_episode ON TABLE episode TYPE number;
DEFINE FIELD absolute_episode ON TABLE episode TYPE number;
DEFINE FIELD runlength ON TABLE episode TYPE object;
DEFINE FIELD skiplength ON TABLE episode TYPE object;
DEFINE FIELD synopsis ON TABLE episode TYPE string ASSERT string::len($value) <= 350 ;
DEFINE FIELD file ON TABLE episode TYPE option<string>;
DEFINE FIELD cover ON TABLE episode TYPE option<string>;
DEFINE FIELD season ON TABLE episode TYPE record<season>;