# Vehicle Make Data API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Listing vehicle make data

```sh
curl -d '{}' -H 'Content-Type: application/json' http://localhost:8080/getvehiclemakedata

http POST :8080/getvehiclemakedata
```

The response should be a 200 OK with the following JSON body:

```json
{
    "message_data": "ALFA ROMEO|ANY|ASHOK|AUDI|BACKHOE|BAJAJ|BEDFORD|BEIBEN|BEIFANG|BHACHU|BMW|BOBCAT|BOMAG|BULLDOZER|BUS|CADILLAC|CAM|CANTER|CASE|CAT|CHEVROLET|CHRYSLER|CITROEN|CMC|CRANE|DAEWOO|DAF|DAIHATSU|DODGE|DOLL|DOZER|DUMPER|EICHER|EXCAVATOR|FAW|FERRARI|FIAT|FORD|FOTON|GEELEY|GRADER|GREATWALL|HAMM|HANS KENYA|HINO|HITACHI|HONDA|HOWO|HUMMER|HYUNDAI|ISUZU|IVECO|JAC|JAGUAR|JCB|JEEP|JMC|JOHN-DEERE|KEHAR|KIA|KLUGER|KOMATSU|LANCER|LANDROVER|LEEBOY|LEXUS|LEYLAND|LEYLANDDAF|LIEBHERR|LOADER|LORRY|M/CYCLE|MACK|MAHINDRA|MAN|MARUTI|MASSEY|MAZDA|MERCEDES|MINI|MITSUBISHI|MIXER|MORRIS|NEWHOLLAND|NIS_DIE|NISSAN|OCEAN|OPEL|PACER|PEUGEOT|PORSCHE|PRIMEMOVER|PUCH|RANDON|RENAULT|ROLLER|ROLLS|ROVER|SAAB|SAILOR|SCANIA|SDLG|SHACMAN|SHOVEL|SINO|SKODA|SONALIKA|SSANG YONG|SUBARU|SUZUKI|TADANO|TANKER|TATA|TEREX|TIGER|TIGGO|TIPPER|TOYOTA|TRACTOR|TRAILER|TRUCK|TUKTUK|TVS|UD|VAUXHALL|VOLKSWAGEN|VOLVO|WUZHENG|XINKAI|YAMAHA|YARI|",
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
