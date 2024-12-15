# Final Project Report

## Team Information

- **Team Member 1**: Ming Yang (1006262223) - Preferred Email
- **Team Member 2**: Xinran Ji (1006093843) - <lisa.ji@mail.utoronto.ca>
- **Team Member 3**: Yukun Zhou (1010122494) - Preferred Email

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
6. **Online Chat**: (Short description)
7. **Leave a group**: The ChatAppMsg::DeleteGroup handler manages the deletion of a group both locally and on the server. Locally, it identifies the group by its group_id, removes it from the list of groups, deletes its join code from the HashMap, and adjusts the selected_group index if necessary. A JSON payload containing the group_id is then created and sent to the server via an HTTP POST request to the /api/group/leave endpoint, with an authorization token included in the headers for authentication. The server's response is handled asynchronously: on success, the ChatAppMsg::DeleteGroupResponse updates the app state, clears any error messages, and logs the deletion. If there’s a network error or server issue, an error message is set in the app state and logged accordingly.

8. **Logout**:The logout functionality effectively terminates the user's session. On the main page, when the "Logout" button is clicked, the ChatAppMsg::Logout message is triggered. This message handler ensures a secure logout by performing several actions: it sets self.logged_in to false to mark the user as logged out, updates self.current_page to Page::LoginPage to navigate to the login screen, and clears any existing error messages by setting self.error_message to None. Additionally, it sends a ChatAppMsg::DisconnectWebSocket message to close any active WebSocket connections, fully disconnecting the user from the server. The handler returns true, indicating that the application's state has changed and prompting the UI to re-render.

### Server

- **Feature 1**: (Short description)  
- **Feature 2**: (Short description)  
- **Feature 3**: (Short description)  

---

## User’s (or Developer’s) Guide

*Provide instructions for using the main features of your project, whether it is a standalone application or a crate.*  

### Client

1. **Registration**: Click the "Register" button in the login page to navigate to the registration page. Enter a username and password in the input fields, then click "Register" to proceed. If the username is already taken, an error message will be displayed. If successfully creating account, it jumps to the login page. Click "Back to Login" to directly return to the login page.

2. **Login**: Access the server login page by entering URL in the browser. Provide your username and password in the respective input fields, then click "Login" to proceed. If the credentials are invalid, an error message will be displayed. Upon successful authentication, you will be redirected to the home page.
3. **Main Page**: Enter the main page when successfully logged in. Clicking "New Group" will navigate to the new group page. Clicking "Logout" will log out the current user and navigate to the login page. Clicking on a group will display the chat history of the group or start a new chat in chat window.
4. **Create a new group**: Click "New Group" on the main page to nevigate to the new group page. Enter a group name and click "Create" to proceed. The group will be created and added to the user's list of groups.
5. **Join a group**: Click "Join a Group" on the main page to nevigate to the new group page. Enter a group name and click "Join" to proceed. The group will be joined and added to the user's list of groups.
6. **Online Chat**: (Explain how to use it step-by-step)
7. **Leave a group**: Click cross button on the group card in the main page to leave the group.
8. **Logout**: Click "Logout" on the main page to log out the current user and navigate to the login page.

### Server

1. **Using Feature 1**: (Explain how to use it step-by-step)  
2. **Using Feature 2**: (Explain how to use it step-by-step)  
3. **Using Feature 3**: (Explain how to use it step-by-step)  

---

## Reproducibility Guide

*Detail the exact commands needed to set up the runtime environment and build the project.*  

### Client

1. **Environment Setup**  

    ```bash
    # Example commands for setting up the environment
    sudo apt update && sudo apt install <required-packages>
    ```

2. **Building the Project**  

    ```bash
    # Example build commands
    cargo build --release
    ```

3. **Running the Project**  

    ```bash
    # Example run commands
    ./target/release/<project-binary-name>
    ```

### Server

---

## Contributions by Team Members

*Break down the contributions made by each team member.*  

- **Team Member 1**: (Description of contributions)  
- **Team Member 2**: (Description of contributions)  
- **Team Member 3**: (Description of contributions)  

---

## Lessons Learned and Concluding Remarks

*Summarize the lessons the team learned during the project, challenges faced, and key takeaways.*  
(Add your reflections and concluding thoughts here.)

---

## Screenshots and Images

*Include relevant screenshots or images to visually support your report. Ensure that the images are stored in the repository and linked correctly.*

![Example Screenshot](images/example-screenshot.png)

---

**Note**: This project was developed as part of [Course Name/Code]. All instructions above were tested on Ubuntu Linux and macOS Sonoma.
