# Final Project Report

## Video Demo

Video demo is in the `chat_app_demo.mp4` file under this GitHub repository.

## Team Information

- **Team Member 1**: Ming Yang (1006262223) - <mingy.yang@mail.utoronto.ca>
- **Team Member 2**: Xinran Ji (1006093843) - <lisa.ji@mail.utoronto.ca>
- **Team Member 3**: Yukun Zhou (1010122494) - <yukun.zhou@mail.utoronto.ca>

---

## Motivation

There are many chat applications available, each with different features. However, privacy and security requirements often complicate the process of online chatting. Typically, users must request permission to add others as friends, using either a QR code or an account number, which can make it difficult to start a group chat quickly. Instant online communication is essential in many situations, but it does not always require the same level of privacy and security. For instance, in a classroom activity or event, a teacher or organizer might need to quickly set up a public chat group for information collection or course discussions. If they could create a temporary chat group, inviting everyone involved to join simply by entering a group number or access code (without requiring permission), the process would be much faster and more convenient.

This need for quick, public and unrestricted access is why we decided to develop a light-weight, real-time, instant online chat solution. Our application allows anyone to create a chat group instantly by generating a temporary join code, enabling participants to join quickly by simpy entering the join code without permissions or resitrictions and leave the group freely.

---

## Objectives

*What are the goals and intended outcomes of this project?*  

- **Objective 1: Develop a user-friendly application for online chatting**  
  Ensure that the login and registration processes are simple and intuitive. Design the chat interface to be easy to use and navigate, providing an accessible experience for all users.

- **Objective 2: Build a simple and efficient chat group management system**  
  Allow users to create, join, and leave chat groups effortlessly. Eliminate unnecessary restrictions and permissions, enabling groups to be created and joined quickly.

- **Objective 3: Ensure the application is secure and reliable**  
  Require each user to have a unique username and password. Implement a join code system for group participation to maintain privacy and prevent unnecessary complications.

- **Objective 4: Ensure real-time online chatting**  
  Deliver chat messages instantly without delay to maintain a seamless and fluent chatting experience.

- **Objective 5: Provide reliable backup functionality**  
  Store all chat history and group information in the server database. Ensure that even if a user logs out, the chat history and group details are restored upon logging back in.

---

## Features

*List the main features of your project and provide a brief explanation of each.*  

### Client

1. **Registeration**:The registration page in this Yew-based web application allows users to create a unique account by providing a username and password, which are essential for identifying users in a group chat. When the "Register" button is clicked, the application retrieves the input values via the DOM API (web_sys), then sends a ChatAppMsg::Register message containing the username and password. The registration logic first checks if the fields are empty and verifies if the username already exists in the server's database through a JSON request-response cycle. If the username is available, it is added to the database. Any errors encountered during registration are reflected in the app's error_message, which is displayed on the page. The "Back to Login" button allows users to navigate to the login page by sending a ChatAppMsg::NavigateTo message. All changes in the app's state trigger re-rendering of the UI, with the render_registration_page function in view.rs responsible for rendering the visual updates.
2. **Login**: The login page allows users to authenticate by entering their username and password, with an option to navigate to the registration page if they do not yet have an account. The render_login_page function in view.rs defines the visual structure, including input fields for credentials, a "Login" button to initiate authentication, and a "Register" button to navigate to the registration page. When "Login" is clicked, the app uses the DOM API (web_sys) to retrieve the input values and sends a ChatAppMsg::Login message for processing. This message triggers the login logic, which validates the inputs and communicates with the server via a JSON request to verify the username and password. The server responds with a JSON object containing the user's ID, username, and authentication token. If authentication fails, the app updates the error_message in its state, which is displayed on the page in red. All updates and interactions dynamically re-render the page for a seamless user experience.
3. **Main Page**:The main page of the chat application offers a user-friendly interface for managing and participating in group-based chats. At the top, a header displays the current user's name along with buttons for creating a new group or logging out, both of which are tied to event callbacks (ChatAppMsg::NavigateTo and ChatAppMsg::Logout). A sidebar lists the user’s groups, each displayed as a button that triggers the ChatAppMsg::SelectGroup message when clicked. Groups also have a delete button that calls the ChatAppMsg::DeleteGroup message to remove the group. The central area displays the chat window for the selected group, showing the group name, online members, chat messages, and a message input field for real-time communication using ChatAppMsg::SendMessage. If no group is selected, the interface prompts the user to choose one.
4. **Create a new group**: The render_new_group_page function defines the user interface for creating or joining a group in the Yew-based chat application. The page includes a navigation bar with the title "New Group Page" and a "Back to Main" button to return to the main page. To create a new group, users enter a group name in an input field and click the "Create Group" button. This action retrieves the input using the DOM API (web_sys) and sends it via the ChatAppMsg::CreateGroup message for further processing. The ChatAppMsg::CreateGroup handler in main.rs constructs a JSON body containing the group name and sends it to the server in an HTTP POST request, along with the user's authentication token to verify the request is from a logged-in user. The server responds with a JSON object containing the group ID, group name, and join code. The application updates its state with the new group details and redirects the user to the main page, the newly created group will be added to the group list in main page.
5. **Join a group**: The render_new_group_page function defines the user interface for creating or joining a group in the Yew-based chat application. The page includes a navigation bar with the title "New Group Page" and a "Back to Main" button to return to the main page. To join a group, users enter a group name in an input field and click the "Join Group" button. This action retrieves the input using the DOM API (web_sys) and sends it via the ChatAppMsg::JoinGroup message for further processing. The ChatAppMsg::JoinGroup handler in main.rs constructs a JSON body containing the join code and sends it to the server in an HTTP POST request, along with the user's authentication token to verify the request is from a logged-in user. The server responds with a JSON object containing the group ID, group name, and join code. The application updates its state with the new group details and redirects the user to the main page, the group will be added to the user's list of groups.
6. **Online Chat**: The online chat feature uses WebSocket connections to enable real-time messaging between users in a group. When a user selects a group, the application establishes a WebSocket connection to the server using the gloo::net::websocket library. Messages are sent and received asynchronously through this connection. The chat window displays the group name, a list of online members that updates in real-time as members join or leave, and the chat history. Users can type messages in an input field and send them by pressing Enter or clicking the "Send" button, which triggers ChatAppMsg::SendMessage. The message is then sent through the WebSocket connection (stored in ws_write) to the server, which broadcasts it to all online group members. The application maintains the connection by sending periodic ping messages via ws_ping_interval, and regularly checks group status through group_status_interval. All chat history is persisted in the server database and loaded when users rejoin a group.
7. **Leave a group**: The ChatAppMsg::DeleteGroup handler manages the deletion of a group both locally and on the server. Locally, it identifies the group by its group_id, removes it from the list of groups, deletes its join code from the HashMap, and adjusts the selected_group index if necessary. A JSON payload containing the group_id is then created and sent to the server via an HTTP POST request to the /api/group/leave endpoint, with an authorization token included in the headers for authentication. The server's response is handled asynchronously: on success, the ChatAppMsg::DeleteGroupResponse updates the app state, clears any error messages, and logs the deletion. If there’s a network error or server issue, an error message is set in the app state and logged accordingly.

8. **Logout**:The logout functionality effectively terminates the user's session. On the main page, when the "Logout" button is clicked, the ChatAppMsg::Logout message is triggered. This message handler ensures a secure logout by performing several actions: it sets self.logged_in to false to mark the user as logged out, updates self.current_page to Page::LoginPage to navigate to the login screen, and clears any existing error messages by setting self.error_message to None. Additionally, it sends a ChatAppMsg::DisconnectWebSocket message to close any active WebSocket connections, fully disconnecting the user from the server. The handler returns true, indicating that the application's state has changed and prompting the UI to re-render.

### Server

#### Techniques Used in Server

1. **Web Framework**: We used Rust's `actix-web` framework to build the server, for its high performance, simplicity and variety of features like actors, middleware, etc.

2. **Database**: We chose `SQLite` as the database, for its lightweight and self-contained features, which can be easily embedded into the rust project and setup by `cargo run`.

3. **Authentication**: We used JWT (JSON Web Tokens) for authentication, which provides a stateless authentication mechanism. We used `jsonwebtoken` crate to handle the JWT operations, and `actix-web-httpauth` crate to build the authentication function as a middleware. The `user_id` value will be extracted in middleware and stored for requests handler's later use.

4. **WebSocket**: We used `actix-web-actors::ws` crate to handle the WebSocket connections, which is based on the `actix::actor` model. The idea of this model is to assign an "Actor" for each websocket connection, which maintains a context of the connection and handles communications between the client and server. This design provides scalability and high performance for handling multiple connections concurrently.

5. **Message Broadcasting**: Similarly, we used `actix::actor` model to implement the `ChatServer` actor, where WebSocket connection actors register to and handles the message broadcasting logic.

#### APIs Provided

There are 9 APIs provided by the server in 3 categories: `Authentication`, `Group Management` and `Chat`, functions can be infered from the API paths. Meanwhile, the prefix `/api` implies those APIs are protected by JWT authentication, and the `user_id` is extracted from the `JWT` token in request headers. The specific API descriptions and test commands can be found in `doc.md`.

- **Authentication**

1. **`/signup`**: User provides a json body with `username` and `password`. Server will first check the database for existing `username`, if not taken, insert a new user and return the generated `user id` and `username`. If taken, a hint message will be returned to client with status code 400. We know that hashing the `password` is a common way to protect it, which is easy to implement with certain crate. We chose not to implement it for the simplicity of inspection and testing.

2. **`/login`**: User provides a json body with `username` and `password`. Server will validate them, if correct, generate a `JWT` token and return it along with `user id` and `username`. If incorrect, a hint message will be returned to client with status code 401 (Unauthorized).

3. **Info about Authentication**: The `JWT` token is consisted of 3 parts: `header`, `payload` and `signature`, where `header` stores the encryption informations, `payload` stores the information need for a stateless authentication (user_id, expire time, etc.) and `signature` is used to verify the token is valid. The signature is generated by encrypting the `header` and `payload` with a `secret key`, which is stored only in the server. The `secret key` should be randomly regenerated if you want to disable all tokens, in our case, that is when server restarts. However, we commented this feature and used a fixed secret key for reproductivity and simplicity of inspection and testing. In following sections, the request headers should contain the `JWT` token client acquired from `/login` for authentication. e.g. `Authorization: Bearer YOUR_ACCESS_TOKEN`. If the authentication fails, the server will return a error message with status code 401 (Unauthorized).

- **Group Management**

1. **`/api/group/create`**: This API is used to create a new group by providing a `group_name`. Server will use `base36` to encode the group id with unix timestamp, and generate a unique `group_code` with proper length and easy to be memorized. Then insert the group information (name, code) into database, and add the creator as the first member of the group. We allow groups with same names, since the `group_code` is unique.

2. **`/api/group/list`**: This API is used to list all groups that the user is in, and return the group information including `group_id`, `group_name`, `group_code`, `created_at` and `members` (a list of user id and username). This is API can be used for various purposes. e.g. for client's redering. No request body is needed because the `user_id` is extracted in authentication middleware.

3. **`/api/group/join`**: This API is used to join a group by providing a `group_code`. Server will first check if the code (group) exists and if the user is already in the group, return a error message with status code 400. If valid, add the user to the group and return the group information (`group_id`, `group_name`, `group_code`).

4. **`/api/group/leave`**: This API is used to leave a group by providing a `group_id`. Server will check if the user is in the group, and if so, remove the user from the group. If not, return a error message with status code 400.

5. **`/api/group/status`**: This API is used to acquire the status of a group by providing a `group_id`, for now, it only returns the usernames of online group members.

- **Chat**

1. **/api/history**: This API is used to acquire the history messages of designated groups, by providing each target group's `group_id` and the `latest_msg_id` of the last message received. In our design, each message has a `id` colum in database schema `id INTEGER PRIMARY KEY AUTOINCREMENT`. To eliminate the inconsistency caused by network delay, we order the messages by the time they are inserted into database, which is by the increasing `id`. In this case, server will return the messages information in this group with `id` larger than `latest_msg_id` to the client.

2. **/api/ws/connect**: This API is used to establish a WebSocket connection with the server. The only request information needed is the `user_id` extracted from the `JWT` token.

#### Messaging Implementation

We used WebSocket to implement the real-time chatting feature, which is based on the `actix::actor` model as we mentioned in the "Techniques Used in Server" section. In our implementation, each connection is managed by a `ConnectionActor` actor, which registers to the `ChatServer` actor for message broadcasting. The `ChatServer` actor is responsible for managing all active connections and broadcasting messages to the appropriate clients. The communications between actors are through messages, which are defined in `messages.rs`. The chatting logics are implemented in `src/chat` folder.

##### **`ConnectionActor`**

This actor is implemented in `connection_actor.rs`, which is responsible for managing the WebSocket connection.

When a client connects, a `ConnectionActor` is created, it will initialize the context of the connection (user_id, last_active_at, etc.) and send a `AddSession` message to the `ChatServer` actor to register the connection. Same happens when a client goes offline, a `RemoveSession` message is sent to the `ChatServer` actor to remove the connection.

Once the connection is established, the `ConnectionActor` will start listening to the WebSocket messages. When a message is received, the actor will first check if it's a json formatted message to broadcast, a plain text message for test purposes or a command to control the connection (e.g. {disconnect}). For the json formatted message, the actor will first deserialize it into a `ChatMessage` struct, then send it to the `ChatServer` actor for broadcasting.

`ConnectionActor` also need to handle the `BroadcastMessage` from `ChatServer` actor, which is another user's message to be broadcasted. After receive the message, the actor will serialize the message into a json formatted string and send it to the client through the WebSocket connection.

The `ConnectionActor` only need to focus on the WebSocket connection, and convey the messages between WebSocket and `ChatServer` actor. All the message broadcasting logic is implemented in `ChatServer` actor.

Also, once the connection is established, the `ConnectionActor` will start a interval task to check if the connection is still active by heartbeat history, however, we deprecated this feature as it's not nessasary for our final design.

##### **`ChatServer`**

This actor is responsible for managing all active connections and broadcasting messages to the appropriate clients.

After receiving a `AddSession` message, the `ChatServer` actor will add the connection actor's information to the `sessions` HashMap. When a `RemoveSession` message is received, the `ChatServer` actor will remove it. This HashMap maintains a list of all active connections, which is used for broadcasting messages, and handling group status requests from the client.

When receiving a `ClientMessage` message, the `ChatServer` actor will first check if the user is a member of the group, if not, return a error message. Then, it will insert the message into database and broadcast it to all online members in the same group by sending a `BroadcastMessage` message to each matched `ConnectionActor`.

When receiving a `CheckUserStatus` message, the `ChatServer` actor will check if the user is online, if so, return the username, otherwise, return a error message. This message is used during handling the group status requests from the client.

#### Tests

To test the server's function during development, we used several tools:

1. **`curl`**: This is the most basic tool for testing HTTP requests, we included the test commands in `doc.md`. You need to `/login` first to get the `JWT` token, then use the token in the request headers.

2. **`wscat`**: This is a tool for testing WebSocket connections and messaging, also included in `doc.md`. If you send a json formatted message, the server will broadcast it to all online members; if you send a plain text message, the server will write back the message with a prefix `"RawText: "`; if you send `"{disconnect}"`, the server will close the connection.

3. **Python script**: We also have some python scripts to test the server's function, which is in the `/test` folder. The `api_test.py` implemented the test functions, and the `chat_test.py` uses those functions to set up a testing environment with 6 users (user_1 to user_6) and 3 groups, group_1 (user 123), group_2 (user 456), group_3 (all users). The script will also print the `wscat` connection commands including the `JWT` token for each user.

4. **Sqlite Extension of VScode**: We used this extension to inspect the database.

5. **Note**: The data is stored in `server.db`, please delete it if you want to reset the database.

## User’s Guide

*Provide instructions for using the main features of your project, whether it is a standalone application or a crate.*  

1. **Registration**: Click the "Register" button in the login page to navigate to the registration page. Enter a username and password in the input fields, then click "Register" to proceed. If the username is already taken, an error message will be displayed. If successfully creating account, it jumps to the login page. Click "Back to Login" to directly return to the login page.

2. **Login**: Access the server login page by entering URL in the browser. Provide your username and password in the respective input fields, then click "Login" to proceed. If the credentials are invalid, an error message will be displayed. Upon successful authentication, you will be redirected to the home page.
3. **Main Page**: Enter the main page when successfully logged in. Clicking "New Group" will navigate to the new group page. Clicking "Logout" will log out the current user and navigate to the login page. Clicking on a group will display the chat history of the group or start a new chat in chat window.
4. **Create a new group**: Click "New Group" on the main page to nevigate to the new group page. Enter a group name and click "Create" to proceed. The group will be created and added to the user's list of groups with a join code.
5. **Join a group**: Click "Join a Group" on the main page to nevigate to the new group page. Enter a join code and click "Join" to proceed. The group will be joined and added to the user's list of groups.
6. **Online Chat**: After selecting a group in the main page, the chat window displays the group name, online members list, and chat history. Type messages in the input field at the bottom and press Enter or click "Send" to send messages. Messages appear instantly in the chat window for all online group members. The chat history is automatically saved and loaded when rejoining the group. Online members are shown in real-time as they join or leave the group.
7. **Leave a group**: Click cross button on the group card in the main page to leave the group.
8. **Logout**: Click "Logout" on the main page to log out the current user and navigate to the login page.

---

## Reproducibility Guide

*Detail the exact commands needed to set up the runtime environment and build the project.*  

### Client

1. **Environment Setup**  

    ```bash
    # Update package lists and install necessary tools
    sudo apt update && sudo apt install curl nodejs npm
    # Install Rust and Cargo
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    # Add WebAssembly target for Rust
    rustup target add wasm32-unknown-unknown
    # Install Trunk for building the Yew application
    cargo install trunk
    ```

    - **If you encounter errors related to `openssl-sys` or `pkg-config` while building a Rust project on Ubuntu or macOS, follow these steps to resolve the issue:**:

    ```bash
    sudo apt update
    sudo apt install pkg-config
    ```

2. **Building the Project**  

    ```bash
    # Build the project using Trunk
    trunk build --release
    ```

3. **Running the Project**  

    ```bash
    # Serve the project using Trunk
    trunk serve --release
    ```

### Server

1. **Environment Setup**  
    Please make sure the rust environment is set up correctly, and the port 8081 is not occupied by other applications.
    Delete the `server.db` file if you want to reset the database.

2. **Running the Project**  

    ```bash
    # Run the server using Cargo
    cargo run --release
    ```

---

## Contributions by Team Members

*Break down the contributions made by each team member.*  

- **Ming Yang (1006262223)**: Responsible for implementing the Client Logic, designing the user interface, and managing the process of sending HTTP requests to the server to send and receive data. Tasks include developing core client-side functionality, ensuring smooth communication with the back-end through efficient data exchange, and contributing to the creation of a clear and functional user interface. Additional responsibilities involve participating in cross-testing to validate functionality and ensure seamless integration across all project components.  

- **Xinran Ji (1006093843)**:
Responsible for the design and implementation of the client-side user interface, with a primary focus on the registration and login pages. Conducted a comprehensive analysis of various approaches for GUI development and selected Yew as the framework due to its compatibility and efficiency. Evaluated the feasibility of using SQLite for data handling but ultimately decided against it due to its incompatibility with Yew. Played a key role in debugging and testing HTTP requests, ensuring accurate error handling and message display. Developed JSON parsing and HTTP request/response handling for core group functionalities, including creating, joining, and leaving groups. Additionally, contributed to the project report by authoring sections on motivation, objectives, user account and group management features, and the reproducibility guide.

- **Yukun Zhou (1010122494)**:
Responsible for all server-side logic design, implementation, documentation and testing. Including database schema, APIs, WebSocket, authentication, message broadcasting, etc. Also recorded the demo video.

---

## Lessons Learned and Concluding Remarks

*Summarize the lessons the team learned during the project, challenges faced, and key takeaways.*  

This project successfully demonstrates the development of a Yew and Actix based web application for real-time communication.

By incorporating essential features such as user registration, authentication, group creation, and group participation, it highlights the seamless integration of client-side interactivity with server-side communication through WebSocket and HTTP protocols. Our team gained valuable insights into modern web application development using Rust and Yew.

One of the key lessons was the importance of structuring a reactive front-end application to efficiently manage state and handle asynchronous communication. For the back-end, it is important to design the APIs in a neat way, which is clearly defined and compatible with the futures improvements.Working with WebSockets provided hands-on experience in implementing real-time features, allows users to send and receive messages instantly without delay, this is a new technique we learned during this project.

We also learned the significance of balancing security and functionality, particularly when implementing features like user authentication, token-based session management, and error feedback. The use of Rust’s strict type system and borrow checker encouraged us to adopt safer coding practices and design more robust systems, especially for tasks like state management and server communication.

Collaboration within the team taught us the value of effective communication, version control with Git, and breaking down tasks into manageable modules.

---
