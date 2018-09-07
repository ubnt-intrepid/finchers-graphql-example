CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f'
)
