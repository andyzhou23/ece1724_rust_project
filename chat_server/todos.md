# Works to do

## Server

- Database
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

- Temporal record (in memory not on disk)
  - Connections
    - id: auto increment u64
