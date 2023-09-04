# Vehicle Model Data API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Listing vehicle model data

```sh
curl -d '{"vehicle_make":"audi"}' -H 'Content-Type: application/json' http://localhost:8080/getvehiclemodeldata

http POST :8080/getvehiclemodeldata "vehicle_make"="audi"
```

The response should be a 200 OK with the following JSON body:

```json
{
    "message_data": "AUDI|AUDI-A2|AUDI-A3|AUDI-A3 SE AUTO|AUDI-A4 AVANT S|AUDI-A4 AVANT W|AUDI-A4 FSI S-L|AUDI-A4 TURBO Q|AUDI-A6 SE AUTO|AUDI-A6 STDI|AUDI-A6 TURBO|AUDI-A6 TURBO S|AUDI-A8 SE TDI|AUDI-AUDI 500|AUDI-Q5|AUDI-Q5 3.2L AU|AUDI-Q7|",
    "status_code": 0,
    "status_description": "Successful"
}
```
