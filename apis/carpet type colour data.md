# Carpet type colour Data API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Listing carpet type colour data

```sh
curl -d '{}' -H 'Content-Type: application/json' http://localhost:8080/getcarpettypecolourdata

http POST :8080/getcarpettypecolourdata
```

The response should be a 200 OK with the following JSON body:

```json
{
    "message_data": "CARPET COLOUR|WHITE|BLACK|RED|BLUE|YELLOW|ORANGE|PURPLE|GREEN|MIXTURE",
    "status_code": 0,
    "status_description": "Successful"
}
```
