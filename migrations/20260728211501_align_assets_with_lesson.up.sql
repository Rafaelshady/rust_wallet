UPDATE assets
SET unit_value = 10.0
WHERE name = 'Bitcoin';

UPDATE assets
SET name = 'Dólar',
    unit_value = 5.5
WHERE name = 'Ethereum';

UPDATE assets
SET name = 'Real',
    unit_value = 1.0
WHERE name = 'Solana';
