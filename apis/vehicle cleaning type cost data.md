# Vehicle cleaning type cost Data API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Listing vehicle cleaning type cost data

```sh
curl -d '{}' -H 'Content-Type: application/json' http://localhost:8080/getvehiclecleaningtypecostdata

http POST :8080/getvehiclecleaningtypecostdata
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful",
    "cost_data": [
        {
            "cleaning_type_name": "interior",
            "amount": 200
        },
        {
            "cleaning_type_name": "exterior",
            "amount": 300
        },
        {
            "cleaning_type_name": "engine",
            "amount": 150
        },
        {
            "cleaning_type_name": "undercarriage",
            "amount": 210
        }
    ]
}
```
