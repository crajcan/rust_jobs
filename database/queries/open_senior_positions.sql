SELECT j.id, c.name, c.space, j.title, j.filled, j.strictly_rust, j.url FROM jobs j join companies c ON j.company_id = c.id WHERE j.filled = false AND title ILIKE 'Sr%';
