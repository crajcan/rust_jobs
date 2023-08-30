SELECT c.id, c.name, c.space
FROM companies c
WHERE NOT EXISTS
    (SELECT 1
     FROM jobs j
     WHERE j.company_id = c.id
         AND j.filled = false)
ORDER BY c.id ASC; 