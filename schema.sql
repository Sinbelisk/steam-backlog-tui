-- Table for representing games
CREATE TABLE game (
    appid INTEGER PRIMARY KEY,
    description TEXT,
    avg_completion_time INTEGER,
    steam_url VARCHAR(255),
    meta_score INTEGER,
    open_score INTEGER,
    steam_score VARCHAR(100)
);

-- Tags embedded on the steam store pages for a game
CREATE TABLE tag (
    tag_id INTEGER PRIMARY KEY,
    name VARCHAR(100)
);

-- N:N relation GAME - TAG
CREATE TABLE game_tag (
    appid INTEGER,
    tag_id INTEGER,

    FOREIGN KEY (appid) REFERENCES game(appid),
    FOREIGN KEY (tag_id) REFERENCES tag(tag_id),
    PRIMARY KEY (appid, tag_id)
);

-- Game library: user 0 is the main user
CREATE TABLE library (
    steam_id INTEGER PRIMARY KEY,
    username VARCHAR(32),
    game_count INTEGER
);

CREATE TABLE library_game (
    library_appid INTEGER PRIMARY KEY AUTO_INCREMENT,
    steam_id INTEGER,
    appid INTEGER,
    hours_played INTEGER DEFAULT 0,
    note TEXT DEFAULT '',
    status TEXT,

    FOREIGN KEY (steam_id) REFERENCES library(steam_id),
    FOREIGN KEY (appid) REFERENCES game(appid)
);

CREATE TABLE backlog_entry (
    entry_id INTEGER PRIMARY KEY AUTO_INCREMENT,
    appid INTEGER,

    FOREIGN KEY (appid) REFERENCES game(appid)
);
