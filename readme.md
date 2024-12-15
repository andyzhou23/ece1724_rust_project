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

3. **Create a new group**: (Short description)
4. **Join a group**: (Short description)
5. **Chat**: (Short description)
6. **Leave a group**: (Short description)
7. **Logout**: (Short description)

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
3. **Create a new group**: (Explain how to use it step-by-step)
4. **Join a group**: (Explain how to use it step-by-step)
5. **Online Chat**: (Explain how to use it step-by-step)
6. **Leave a group**: (Explain how to use it step-by-step)
7. **Logout**: (Explain how to use it step-by-step)

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
