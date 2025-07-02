INSERT INTO users (id, email)
VALUES (
    'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa',
    'test-user@example.com'
)
ON CONFLICT (email) DO NOTHING;

INSERT INTO devices (id, device_id, location, name)
VALUES (
    'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb',
    'test',
    'some-room',
    'Test Device'
)
ON CONFLICT (device_id) DO NOTHING;

INSERT INTO user_devices (user_id, device_id)
VALUES (
    'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa',
    'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb'
)
ON CONFLICT DO NOTHING;
