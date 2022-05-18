# SimpleRestaurantApiServer
<h2>Context</h2>
This application was created to resolve this problem : https://github.com/paidy/interview/blob/master/SimpleRestaurantApi.md

<p>This project contains the server application.</p>

You can find the client application [here](https://github.com/DidierNicolas/SimpleRestaurantApiClient)
<h2>Build & Launch the Application</h2>
<p>To build and run this application, apply theses instructions in the project folder</p>
<code>cargo build --bin simple_api_restaurant </code><br>
<code>cargo run --bin simple_api_restaurant</code>

## Prerequisites
- Postgresql Database
  <br><br>For this project I am using PostGreSql 14 make sure to get the last version [Here](https://www.postgresql.org/download/)
- Server configuration
  ````
    server :
      ip = "127.0.0.1"
      port = "8080"

    Database
      prefix = "postgresql://postgres"
      password = "postgres"
      ip = "localhost"
      port = "5432"
      db_name = "postgres"
  ````
  The db schema will be delete and created each time you lunch the application, if you want to keep the data, delete the drop line [here](https://github.com/DidierNicolas/SimpleRestaurantApiServer/blob/master/sql/db.sql)
  
## Curl testing
- Get all items <br>
 <code>curl -X GET -H 'content-type: application/json;' http://localhost:8080/items </code>
- Get items from a table <br>
 <code>curl -X GET -H 'content-type: application/json;' http://localhost:8080/tables/tid </code>
- Get a specific item from a specific table <br>
 <code> curl -X GET -H 'content-type: application/json;' http://localhost:8080/tables/tid/iid </code>
- Create new item(s)
 <code> curl -X GET -H 'content-type: application/json;' http://localhost:8080/items {json_string} </code>
   - JSON Example 
     ```json
     {
          "tid": 1,
          "items": [
          {
              "name": "Steak",
              "cook_time": 5
          },
          {
              "name": "Burger",
              "cook_time": 10
          }]
      }
     ```
- Delete item from a specific table <br>
 <code>curl -X DELETE  -H 'Content-Type: application/json;' http://localhost:8080/tables/tid/iid</code>
- Update item from a specific table <br>
 <code>curl -X PUT  -H 'Content-Type: application/json;' http://localhost:8080/tables/tid/iid -d {json_string} </code>
   - JSON example
     ```json
      {
          "name": "Apple pie",
          "cook_time": 12
      }
     ```
 ## Client Side
 You can also use the [Client application](https://github.com/DidierNicolas/SimpleRestaurantApiClient) wich will randomly make request
