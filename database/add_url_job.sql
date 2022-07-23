ALTER TABLE jobs ADD COLUMN url TEXT;

UPDATE jobs SET url = 'https://oxide.computer/careers/sw-control-plane'
WHERE ID = 1;

UPDATE jobs SET url = 'https://oxide.computer/careers/sw-internal-systems'
WHERE ID = 2;

ALTER TABLE jobs ALTER COLUMN url SET NOT NULL;