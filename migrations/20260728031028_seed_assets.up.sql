INSERT INTO assets (name, unit_value)
VALUES
    ('Bitcoin', 10.0),
    ('Ethereum', 20.0),
    ('Solana', 30.0)
ON CONFLICT (name) DO NOTHING;
