# Documentation

## API

- `/signup`
    - POST
    - Body:
      ```json
      {
        "username": "string",
        "password": "string"
      }
      ```
    - Response:
      ```json
      {
        "id": "integer",
        "username": "string"
      }
      ```
    - Error responses:
      - 400 Bad Request: Username already taken
      - 500 Internal Server Error: Database errors
    - Test:
      - curl -w "\n" "http://localhost:8080/signup" --json '{"username":"user1", "password":"123456"}'


- `/login`
    - POST
    - Body:
      ```json
      {
        "username": "string",
        "password": "string"
      }
      ```
    - Response:
      ```json
      {
        "id": "integer",
        "username": "string"
      }
      ```
    - Error responses:
      - 400 Bad Request: Username or password incorrect
      - 500 Internal Server Error: Database errors
    - Test:
      - curl -w "\n" "http://localhost:8080/login" --json '{"username":"user1", "password":"123456"}'
