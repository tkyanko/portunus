-- Ensure SQLite foreign key support is on, since it defaults to off
PRAGMA foreign_keys = ON;

-- Create the core tables and indexes
CREATE TABLE products (
    id INTEGER UNIQUE NOT NULL PRIMARY KEY,
    title VARCHAR UNIQUE NOT NULL,
    store VARCHAR NOT NULL DEFAULT 'Steam'
);
CREATE INDEX idx_key_titles ON products(title);

CREATE TABLE users (
    id INTEGER UNIQUE NOT NULL PRIMARY KEY,
    discord_id VARCHAR UNIQUE NOT NULL,
    last_taken_time DATETIME NOT NULL DEFAULT '1970-01-01T00:00:00',
    keys_given INTEGER NOT NULL DEFAULT 0,
    keys_taken INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX idx_discord_id ON users(discord_id);

CREATE TABLE keys (
    id INTEGER UNIQUE NOT NULL PRIMARY KEY,
    product_id INTEGER NOT NULL,
    product_key VARCHAR NOT NULL,
    time_added DATETIME NOT NULL,
    user_who_added INTEGER NOT NULL,
    time_claimed DATETIME,
    user_who_claimed INTEGER,
    FOREIGN KEY (product_id) REFERENCES products(id),
    FOREIGN KEY (user_who_added) REFERENCES users(id),
    FOREIGN KEY (user_who_claimed) REFERENCES users(id)
);
CREATE INDEX idx_keys_time_added ON keys(time_added);
CREATE INDEX idx_user_who_claimed ON keys(user_who_claimed);
