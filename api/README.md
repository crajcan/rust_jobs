
#### Curl the taxi cab example

```
curl -X POST http://localhost:8080/trips/query --header 'Content-Type: application/json' --data '{"$limit":1, "$filter": {"pickup_location": 211}}'
```

#### Curl the oapi docs

```
curl -X POST http://localhost:8080/trips/oapi
```
