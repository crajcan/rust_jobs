SELECT
    job.id,
    company.name,
    company.space,
    job.title,
    job.filled,
    job.strictly_rust,
    job.url
FROM
    jobs job
JOIN
    companies company
ON job.company_id = company.id
WHERE job.filled = false;
