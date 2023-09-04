# Sales Data API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Adding sales data

```sh
curl -d '{"batch_no": "1","sales_data": [{"customer_sales_data": {"cust_name": "Risper Muite","mobile_no": "0712876340","sales_amount": "560","paid_amount": "560","payment_mode": "cash"},"vehicle_sales_data": {"vehicle_make": "Toyota","vehicle_model": "Corolla","vehicle_regno": "kag 283j","sales_amount": "560","payment_mode": "cash","interior_cleaning": true,"exterior_cleaning": true,"engine_cleaning": true,"undercarriage_cleaning": true,"employee_id": 1,"employee_full_names": "John Doe"},"carpet_sales_data": {"carpet_size": "5 by 8","carpet_colour": "blue","sales_amount": "930","payment_mode": "mpesa","employee_id": 1,"employee_full_names": "John Doe"}}]}' -H 'Content-Type: application/json' http://localhost:8080/addsalesdata

http POST :8080/addsalesdata "batch_no": "1" "sales_data": [{"customer_sales_data": {"cust_name": "Risper Muite","mobile_no": "0712876340","sales_amount": "560","paid_amount": "560","payment_mode": "cash"},"vehicle_sales_data": {"vehicle_make": "Toyota","vehicle_model": "Corolla","vehicle_regno": "kag 283j","sales_amount": "560","payment_mode": "cash","interior_cleaning": true,"exterior_cleaning": true,"engine_cleaning": true,"undercarriage_cleaning": true,"employee_id": 1,"employee_full_names": "John Doe"},"carpet_sales_data": {"carpet_size": "5 by 8","carpet_colour": "blue","sales_amount": "930","payment_mode": "mpesa","employee_id": 1,"employee_full_names": "John Doe"}}]}
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful"
}
```

## Listing sales data

```sh
curl -d '{}' -H 'Content-Type: application/json' http://localhost:8080/getallsalesdata

http :8080/getallsalesdata
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful",
    "sales_batch_data": []
}
```

## Search sales data

```sh
curl -d '{"search_by":{"customer_name": true}, "search_data": "john"}' -H 'Content-Type: application/json' http://localhost:8080/getsearchsalesdata

http POST :8080/getsearchsalesdata "search_by"={"customer_name": true} "search_data"="john"
```

The response should be a 200 OK with the following JSON body:

```json
{
    "status_code": 0,
    "status_description": "Successful",
    "sales_batch_data": []
}
```
