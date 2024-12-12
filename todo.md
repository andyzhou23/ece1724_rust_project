# TODO

## Client

### To learn

- database: sqlite, pool, query
- websocket: actix_web_actors::ws, streamHandler, Actor
- jwt: jsonwebtoken (do at last)

### To implement

- user
  - signup: username+password -> id or error
  - login: username+password -> token or error
  - logout: mainly on GUI

- group
  - create: group_name -> group_info(id, name, code)
  - join: group_code -> group_info(id, name, code)
  - leave: group_id -> ()
  - list: user_id -> [group_info(id, name, code, members)] // get group infos for display

- message
  - send: send message
  - receive: receive message and display
  - history: (group_id, latest_msg_id) -> [message(id, group_id, sender_id, content, created_at)] // get message history for display
