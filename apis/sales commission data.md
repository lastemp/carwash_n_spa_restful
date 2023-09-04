# Sales commission Data API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Listing sales commission data

```sh
curl -d '{}' -H 'Content-Type: application/json' http://localhost:8080/getallsalescommissiondata

http POST :8080/getallsalescommissiondata
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful",
    "sales_commission_data": []
}
```

## Search sales commission data

```sh
curl -d '{"search_by":{"employee_full_names": true}, "search_data": "john"}' -H 'Content-Type: application/json' http://localhost:8080/getsearchsalescommissiondata

http POST :8080/getsearchsalescommissiondata "search_by"={"employee_full_names": true} "search_data"="john"
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful",
    "sales_commission_data": []
}
```