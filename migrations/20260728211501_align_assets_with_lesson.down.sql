UPDATE assets
SET unit_value = 10.0
WHERE name = 'Bitcoin';

UPDATE assets
SET name = 'Ethereum',
    unit_value = 20.0
WHERE name = 'Dólar';

UPDATE assets
SET name = 'Solana',
    unit_value = 30.0
WHERE name = 'Real';
