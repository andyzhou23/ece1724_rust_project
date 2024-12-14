# Documentation

## API

### public api

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
    - curl -w "\n" "http://localhost:8081/signup" --json '{"username":"user_1", "password":"123456"}'

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
        "username": "string",
        "access_token": "string"
      }
      ```

  - Error responses:
    - 401 Unauthorized: Username or password incorrect
    - 500 Internal Server Error: Database errors
  - Test:
    - curl -w "\n" "http://localhost:8081/login" --json '{"username":"user_1", "password":"123456"}'

### jwt protected api (prefix: /api)

- JWT Auth
  - Header: "Authorization: Bearer YOUR_ACCESS_TOKEN"
  - Test:
    - curl -w "\n" -H "Authorization: Bearer YOUR_ACCESS_TOKEN" http://localhost:8081/api/

- `/api/ws/connect`
  - GET
  - Path:
    - user_id: integer
  - Response:
    - WebSocket connection
  - Error responses:
    - 400 Bad Request: User not found
  - Test:
    - wscat -c ws://localhost:8081/api/ws/connect -H "Authorization: Bearer YOUR_ACCESS_TOKEN"

- `/api/group/create`
  - POST
  - Body:

    ```json
    {
      "name": "string"
    }
    ```

  - Response:

    ```json
    {
      "group_id": "integer",
      "group_name": "string",
      "group_code": "string"
    }
    ```

  - Error responses:
    - 500 Internal Server Error: Database errors
  - Test:
    - curl -w "\n" -H "Authorization: Bearer YOUR_ACCESS_TOKEN" "http://localhost:8081/api/group/create" --json '{"name":"group1"}'

- `/api/group/list`
  - GET
  - Response:

    ```json
    [
      {
        "id": "integer",
        "name": "string",
        "code": "string", 
        "created_at": "integer",
        "members": [
          {
            "id": "integer",
            "name": "string"
          }
        ]
      }
    ]
    ```

  - Error responses:
    - 500 Internal Server Error: Database errors
  - Test:
    - curl -w "\n" -H "Authorization: Bearer YOUR_ACCESS_TOKEN" "http://localhost:8081/api/group/list" -X GET

- `/api/group/join`
  - POST
  - Request:

    ```json
    {
      "group_code": "string"
    }
    ```

  - Response:

    ```json
    {
      "group_id": "integer", 
      "group_name": "string",
      "group_code": "string"
    }
    ```

  - Error responses:
    - 400 Bad Request: Invalid group code or user already in group
    - 500 Internal Server Error: Database errors
  - Test:
    - curl -w "\n" -H "Authorization: Bearer YOUR_ACCESS_TOKEN" "http://localhost:8081/api/group/join" --json '{"group_code":"FILL_THIS_PART_IN_TEST"}'

- `/api/group/leave`
  - POST
  - Request:

    ```json
    {
      "group_id": "integer"
    }
    ```

  - Response:

    ```json
    {
      "message": "string"
    }
    ```

  - Error responses:
    - 400 Bad Request: User is not a member of group
    - 500 Internal Server Error: Database errors
  - Test:
    - curl -w "\n" -H "Authorization: Bearer YOUR_ACCESS_TOKEN" "http://localhost:8081/api/group/leave" --json '{"group_id":1}'

- `/api/history`
  - GET
  - Request:

    ```json
    {
      "user_id": "integer",
      "entries": [
        {
          "group_id": "integer",
          "latest_msg_id": "integer"
        }
      ]
    }
    ```

  - Response:

    ```json
    {
      "group_id": [
        {
          "msg_id": "integer",
          "group_id": "integer",
          "sender_id": "integer",
          "content": "string",
          "created_at": "integer"
        }
      ]
    }
    ```

  - Error responses:
    - 500 Internal Server Error: Database errors
    - If user is not a member of group, its key will be missing in the response
  - Test:
    - curl -w "\n" -H "Authorization: Bearer YOUR_ACCESS_TOKEN" "http://localhost:8081/api/history" --json '{"entries":[{"group_id":1, "latest_msg_id":5}]}' -X GET

## WebSocket Payload Format

- Text:
  - ClientMessageJson (client -> server) // {"group_id":1,"content":"hello world"}
    - group_id: integer
    - content: string
  - BroadcastMessage (server -> client)
    - msg_id: integer
    - sender_id: integer
    - group_id: integer
    - content: string
    - created_at: integer (Unix timestamp in seconds)
    - sender_name: string
  - if not json, server will build one with group_id=0
- Ping:
  - Anything
  - server will respond the same message
- Close:
  - Anything
