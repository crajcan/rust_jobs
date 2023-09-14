### Connect to fly postgres instance: 

```
flyctl postgress connect -a rust-jobs
```

### Clearing walsenders from database: 

```
rust_jobs=> SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE pid <> pg_backend_pid() AND datname = 'rust_jobs';
```
