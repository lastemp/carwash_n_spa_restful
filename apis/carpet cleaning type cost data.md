# Carpet cleaning type cost Data API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Listing carpet cleaning type cost data

```sh
curl -d '{}' -H 'Content-Type: application/json' http://localhost:8080/getcarpetcleaningtypecostdata

http POST :8080/getcarpetcleaningtypecostdata
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful",
    "cost_data": [
        {
            "cleaning_size_name": "5by8",
            "amount": 600
        },
        {
            "cleaning_size_name": "6by9",
            "amount": 700
        },
        {
            "cleaning_size_name": "7by10",
            "amount": 800
        },
        {
            "cleaning_size_name": "8by11",
            "amount": 900
        }
    ]
}
```
