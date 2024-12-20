# Works to do

## Server

- Database **done**
  - Messages
    - id: auto increment u64
    - group_id: u64
    - user_id: u64
    - content: string
    - created_at: timestamp
  - Groups
    - id: auto increment u64
    - name: string
    - code: string
    - created_at: timestamp
  - Users
    - id: auto increment u64
    - name: string
    - password: string
    - created_at: timestamp
  - GroupMembers
    - id: auto increment u64
    - group_id: foreign key to Groups.id
    - user_id: foreign key to Users.id
    - created_at: timestamp

- Chat Server Context **done**
  - actor messages (all from ws actor to session manager, handler impl in session manager)
    - BroadcastMessage
      - user_id: u64
      - group_id: u64
      - content: string
      - created_at: timestamp
    - AddSession
      - user_id: u64
      - addr: Addr<WebSocketActor>
    - RemoveSession
      - user_id: u64
  - SessionManager
    - struct SessionInfo
      - user_id: u64
      - username: string
      - addr: Addr<WebSocketActor>
      - connected_at: timestamp
    - sessions: HashMap<user_id, SessionInfo>
    - fn new() -> Self
    - fn add_session(user_id: u64, session: SessionInfo)
    - fn remove_session(user_id: u64)
    - impl Actor for SessionManager
    - impl Handlers for SessionManager
      - <BroadcastMessage>
      - <AddSession>
      - <RemoveSession>
  - WsActor
    - struct WsActor
      - user_id: u64
      - session_manager: Addr<SessionManager>
      - last_active_at: timestamp
    - fn new(user_id: u64, session_manager: Addr<SessionManager>) -> Self
    - fn start_heartbeat(self)
    - impl Actor for WsActor // register at SessionManager
      - fn started(self) -> send(SessionManager::AddSession(self.user_id, self))
      - fn stopped(self) -> send(SessionManager::RemoveSession(self.user_id))
    - impl StreamHandler // callback when receive message
      - fn handler(self, msg: Message, stream: &mut StreamSession)
        - match: text
          - send(BroadcastMessage(self.user_id, msg.to_string()))
        - match: Ping
          - udpate last_active_at
          - response Pong
        - match: else (including close)
          - close connection
  - WsActor Creator
    - GET /ws
    - ws::start(WebSocketActor::new(user_id, session_manager), &req, stream)

## Http Functions

- /login **done**
- /signup **done**
- /ws/connect/{user_id} **done**
- /group/create **done**
  - name: string
  - user_id: u64
- /group/join **done**
  - user_id: u64
  - group_code: string
- /group/leave **done**
  - user_id: u64
  - group_id: u64
- /group/list/{user_id} **done**
- /history
