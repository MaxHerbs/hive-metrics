CREATE TABLE users (
    id UUID PRIMARY KEY,
    email TEXT UNIQUE NOT NULL
);

CREATE TABLE devices (
    id UUID PRIMARY KEY,
    device_id TEXT UNIQUE NOT NULL,
    location TEXT,
    name TEXT
);

CREATE TABLE user_devices (
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    device_id UUID REFERENCES devices(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, device_id)
);
