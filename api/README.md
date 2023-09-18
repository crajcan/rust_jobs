## Production Deployment (fly.io)

Deployed via Dockerfile + fly CLI to fly.io "rust-jobs-api"

### Curl the companies from production:

curl -X POST  https://rust-jobs-api.fly.dev/companies/query \
  --header 'Content-Type: application/json' --data-raw '{"$limit":3}'

### Admin url

```
https://fly.io/apps/rust-jobs-api
```

## Production Deployment (render.com [on hold])

Deployed via Github to render.com app "rust_jobs_api"

### Curl the oapi docs from production:

curl -X POST https://rust-jobs-api.onrender.com/companies/oapi

### Curl the companies from production:

curl -X POST  https://rust-jobs-api.onrender.com/companies/query \
  --header 'Content-Type: application/json' --data-raw '{"$limit":3}'

### Cross compiling dozer for x86_64 from m1 macbook

```
RUSTFLAGS="-C link-args=-fstack-protector-all -lssp" cargo build --target x86_64-unknown-linux-gnu --release
```


## Development

### Curl the oapi docs

```
curl -X POST http://localhost:8080/trips/oapi
```
### Curl the companies
curl -X POST  localhost:8080/companies/query \
  --header 'Content-Type: application/json' --data-raw '{"$limit":3}'

### Run dozer with options

```
dozer run --config-path ./dozer-config.yaml
```

