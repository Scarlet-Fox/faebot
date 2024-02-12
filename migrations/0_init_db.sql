CREATE TABLE IF NOT EXISTS characters (
    id                  INTEGER PRIMARY KEY NOT NULL,
    guild_id            TEXT                NOT NULL,
    owner_id            TEXT                NOT NULL,
    name                TEXT                NOT NULL,
    description         TEXT                NOT NULL,
    refresh             INTEGER             NOT NULL DEFAULT 0,
    high_concept        TEXT                NOT NULL,
    trouble             TEXT                NOT NULL,
    aspect_three        TEXT                NOT NULL,
    aspect_four         TEXT                NOT NULL,
    aspect_five         TEXT                NOT NULL,
    extras              TEXT                NOT NULL,
    stunts              TEXT                NOT NULL,
    consequence_one     TEXT                NOT NULL,
    consequence_two     TEXT                NOT NULL,
    consequence_three   TEXT                NOT NULL,
    consequence_four    TEXT                NOT NULL,
    physical_capacity   INTEGER             NOT NULL,
    mental_capacity     INTEGER             NOT NULL
);

CREATE TABLE IF NOT EXISTS character_skills (
    id                  INTEGER PRIMARY KEY NOT NULL,
    character_id        INTEGER             NOT NULL,
    level               INTEGER             NOT NULL,
    name                TEXT                NOT NULL
);

