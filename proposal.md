# Proposal

## Team Members

| Name       | Student ID |
| ---------- | ---------- |
| Ming Yang  | 1006262223 |
| Xinran Ji  | 1006093843 |
| Yukun Zhou | 1010122494 |

## Motivation

  There are many chat applications available, each with different features. However, privacy and security requirements often complicate the process of online chatting. Typically, users must request permission to add others as friends, using either a QR code or an account number, which can make it difficult to start a group chat quickly. In many situations, however, instant online communication is essential, while the importance of privacy and security are relatively low.
  
  For example, in a classroom activity or event, a teacher or organizer might need to quickly set up a public chat group for information collection or course discussions. If they could create a temporary chat group, inviting everyone involved to join simply by entering a group number or access code (without requiring permission), the process would be much faster and more convenient.
  
  This need for quick, public and unrestricted access is why we decided to develop a light-weight, real-time, instant online chat solution. Our application will allow anyone to create a chat group instantly by generating a temporary join code, enabling participants to join quickly and leave freely, without unnecessary restrictions or permissions required to start chatting.

## Objective & Key Features

### User Authentication

- **Sign Up**:  
  Create a user with username and password, server will make sure the username is unique.
- **Sign In**:  
  Input username and password to authenticate, if success, user will receive a token to authenticate the following actions. Client will automatically send heartbeat messages to server for online notice and retrieve the messages during absence.
- **Sign Out**:  
  Inform server the client is down, stop sending and cache new messages

### Chat Group Management

- **Creation**:
  Set a simple join code to share with other people, with a limited valid time. System will inform you whether the code is available (not currently in use). If successful, the group will be created, a unique group id will be automatically generated by the server and assigned as the only identifier of this group.
- **Administration**:  
  The group will be deleted if the group owner quit the group, all group members will be informed and not be able to send messages in this group anymore.
- **Joining**:
  - By a short join code within the limited time after the group was created, which reflects the feature of "instant grouping".
  - By the unique group identifier after the join code expires.
- **Quitting**:
  - For group member, quitting a group will inform the server to stop forwarding new messages and disable user's message sending.
  - For group owner, quitting means the deletion of this group, the server will perform the quitting action for every members and delete the records and free other resources of this group.
  - The deletion of chat history on client's side is separated from quitting the group, which can be later done by users themselves. The actions on chat history at server's side won't be reflected on client's side.

### Real-time Messaging

- **Message Structure**:
  - Token
  - Time stamp
  - Group id
  - Sender's user id
  - Content
- **Server's Behavior**:
  - Validate the message by checking the token, group id and sender id.
  - Locate the group, append the message information to group chat history.
  - Broadcast this message to every online users, set the "updated time" record of those users to current time.
  - For current offline users, they will receive the missed messages (range determined by their "updated time" record) upon sign in.
- **Client's Behavior**:
  - Cache the existing messages to save bandwidth.
  - Display and update the messages properly.

### Presence Detection

- Server will maintain a status record of every user, including the timestamp of their last "heartbeat".
- While the user is logged in, the client will recurrently send a "heartbeat" message to server, that indicates it's alive.
- The client will also explicitly inform the server of its status at signing in and out.
- The user's status is calculated upon request, determined by whether the time gap between last heartbeat and current time reaches the threshold.

### Graphic User Interface

- **Welcome Page**: Allow user to sign up or sign in.
- **Main Page**:
  - Group list, shows the groups this user joined.
  - Chat history, shows the current group's chat history, default the group on top.
  - Group status, including group id, group members and their status.
  - Group setting, quit button for every one, and a group name setting for group owner.
  - Sign out button.
- **New Group Page**:
  - Create group  
  By enter a join id, the group will be created if the join id is valid; otherwise user will be informed to try another join id. The join id will be expired after a certain time limit.
  - Join group  
  Users can join the group by a short join code within the time limit. After that, user can still use the unique group id (usually longer) to join the group.

## Tentative Plan

Generally, we can split the implementation into four main layers, to share the workload and enabling parallel development:

- Client Logic: Ming Yang
- Graphic User Interface: Xinran Ji
- API: Ming Yang, Xinran Ji
- Backend: Yukun Zhou
- Testing: Everyone cross-test each others part

Front-end and back-end development will be pushed forward simultaneously during the following phases.

### Phase 1: Basic Setup and User Authentication

- **Duration**: 1 week
- **Goals**: Implement the frontend/backend infrastructure and user authentication flow.
- **Tasks**:
  - Set up client and server infrastructure.
  - Implement sign up, sign in, and sign out features.
  - Implement token authentication flow to ensure smooth login/logout and api calling.

### Phase 2: Chat Group Management

- **Duration**: 1 week
- **Goals**: Create, join, and manage chat groups.
- **Tasks**:
  - Group creation feature with short term join code.
  - Develop the administration logic for owner.
  - Group joining by short code function, and by unique ID after join code expired.
  - Implement quitting feature for group members and owner, including resource cleanup.

### Phase 3: Real-time Messaging

- **Duration**: 1 week
- **Goals**: Real-time message exchange and absence message caching.
- **Tasks**:
  - Define the message structure to include information required.
  - User authorization and massage validation.
  - Real-time message broadcast to online users.
  - Build "absence message caching" function for offline users.

### Phase 4: Presence Detection

- **Duration**: 1 week
- **Goals**: Maintain and show user presence status.
- **Tasks**:
  - Implement presence detection with "heartbeat" messages.
  - Calculate user's status upon request by comparing the heartbeat time gap and threshold.
  - Use presence detection results in messaging and group logic.
  - Display on GUI.

### Phase 5: Testing and Refinement

- **Duration**: 1 week
- **Goals**: Ensure all features work seamlessly and are user-friendly.
- **Tasks**:
  - Conduct functional and usability testing across all features.
  - Optimize backend server performance.
  - GUI improvements.
