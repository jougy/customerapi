# Customer API
## summary
This is an example of a simple API built using the Rust programming language and the Axum framework. The API provides endpoints to create, list, and retrieve customer information using HTTP requests. The customer information is stored in an in-memory hashmap.

### The API uses the struct Customer to represent customer information, represented by the following JSON:

- id: customer's unique identifier (`usize`)
- first_name: customer's first name (`String`)
- last_name: customer's last name (`String`)
- email: customer's email address (`String`)
- associated_ethereum_addresses: list of Ethereum addresses associated with the client (`Vec<String>`)

## Functions
### To create a new customer:
```
curl --header "Content-Type: application/json" \
     --request POST \
     --data '{
        "first_name": "Vitalik",
        "last_name": "Buterin",
        "email": "vitalik@ethereum.org",
        "associated_ethereum_addresses": [
            "0xab5801a7d398351b8be11c439e05c5b3259aec9b",
            "0xDAFEA492D9c6733ae3d56b7Ed1ADB60692c98Bc5"
        ]
     }' \
     localhost:3000/customers
```
### To retrieve a customer by ID:

```
curl http://localhost:3000/customer/{ID} 
```
### To retrieve a list of all customers:
```
curl http://localhost:3000/customers
```
