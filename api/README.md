#### Deployment

Deployed via Github to render.com app "rust_jobs_api"

#### Curl the oapi docs from production: 

curl -X POST https://rust-jobs-api.onrender.com/companies/oapi

#### Curl the companies from production: 

curl -X POST  https://rust-jobs-api.onrender.com/companies/query \
  --header 'Content-Type: application/json' --data-raw '{"$limit":3}'

#### Curl the taxi cab example

```
curl -X POST http://localhost:8080/trips/query --header 'Content-Type: application/json' --data '{"$limit":1, "$filter": {"pickup_location": 211}}'
```

#### Curl the oapi docs

```
curl -X POST http://localhost:8080/trips/oapi
```

#### Run dozer with options

```
dozer run --config-path ./dozer-config.yaml
```

#### To build, in dozer repo:

```
RUSTFLAGS="-C link-args=-fstack-protector-all -lssp" cargo build --target x86_64-unknown-linux-gnu --release
```

