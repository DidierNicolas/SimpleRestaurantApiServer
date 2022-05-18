DROP TABLE IF EXISTS items CASCADE;
CREATE TABLE IF NOT EXISTS items
(
    id INT NOT NULL,
    tid INT NOT NULL,
    name VARCHAR(255),
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    cook_time INT,
    PRIMARY KEY (id, tid)
);

