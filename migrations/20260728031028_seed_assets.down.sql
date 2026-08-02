DELETE FROM assets AS a
WHERE a.name IN ('Bitcoin', 'Ethereum', 'Solana')
  AND NOT EXISTS (
      SELECT 1
      FROM owned_assets AS o
      WHERE o.asset_id = a.id
  );
