CREATE TABLE IF NOT EXISTS user (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS project (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    environments TEXT NOT NULL,
    description TEXT NOT NULL,
    name TEXT UNIQUE NOT NULL,
    visibility TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS project_admin (
    project_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES project(id),
    FOREIGN KEY (user_id) REFERENCES user(id)
);

CREATE TABLE IF NOT EXISTS project_collaborator (
    project_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES project(id),
    FOREIGN KEY (user_id) REFERENCES user(id)
);

CREATE TABLE IF NOT EXISTS curl_group (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    curls TEXT NOT NULL,
    description TEXT NOT NULL,
    labels TEXT NOT NULL,
    name TEXT NOT NULL,
    project_id INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES project(id)
);
