# Employees Data API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Listing employees data

```sh
curl -d '{}' -H 'Content-Type: application/json' http://localhost:8080/getallemployeesdata

http POST :8080/getallemployeesdata
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful",
    "employees_data": []
}
```
