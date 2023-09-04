# Carpet type size Data API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Listing carpet type size data

```sh
curl -d '{}' -H 'Content-Type: application/json' http://localhost:8080/getcarpettypesizedata

http POST :8080/getcarpettypesizedata
```

The response should be a 200 OK with the following JSON body:

```json
{
    "message_data": "CARPET SIZE|5 by 8|6 by 9|7 by 10|8 by 11|",
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
