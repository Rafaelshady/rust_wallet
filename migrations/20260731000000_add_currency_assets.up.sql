INSERT INTO assets (name, unit_value)
VALUES
    ('Dólar', 5.5),
    ('Real', 1.0)
ON CONFLICT (name) DO NOTHING;
