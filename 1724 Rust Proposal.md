# Real-Time Chat Application Proposal
Name: Zeying Zhou Student#: 1005172732
Name: Danyang Zhang Student#: 1005087022
Name: Renchao Wu Student#: 1009750672

## Technologies
- [Actix Web](https://actix.rs/): Backend framework for building asynchronous web applications in Rust.
- [Yew](https://yew.rs/): Frontend framework for building web applications in Rust.
- [WebSocket](https://docs.rs/websocket/latest/websocket/): Communication protocol for real-time, bidirectional messaging.
- [GitHub](https://github.com/): Version control and collaboration platform.
- [Postman](https://www.postman.com/): Tool for API testing and debugging.


## Motivation

Real-time chat applications have evolved into essential tools for modern communication, underpinning interactions in social, professional, and support environments worldwide. Starting from basic messaging systems, the field has witnessed the rise of sophisticated platforms like IRC (Internet Relay Chat) and the progression to highly interactive, feature-rich applications such as Slack, Discord, and Microsoft Teams. These advancements have redefined how people collaborate and communicate in real time, with applications that deliver not only messages but also a sense of connection, immediacy, and productivity.

The initial versions of chat applications were constrained by limited technology and infrastructure, offering only the most basic functionality. Over time, however, demand surged for real-time data transfer, stable performance, and high scalability. Today’s communication needs require chat applications that are extremely efficient, reliable, and responsive even under high user loads. Users expect instant delivery of messages, seamless presence updates, and a smooth user experience without lag. This high standard poses a challenge to developers, especially as chat applications must now balance scalability with security, maintain high performance across multiple users and devices, and address the increasing complexity of concurrent message exchanges.

Many of the current chat platforms are built using languages like JavaScript, Python, or Java, each of which has established itself as a reliable choice for building web applications. However, these languages, while powerful, have inherent limitations when it comes to maximizing performance and security, particularly at the system level. They often struggle to manage memory as efficiently, and ensuring both speed and stability under high load can require significant workarounds. This is where Rust, a systems programming language known for its safety and efficiency, presents a compelling alternative.

Rust offers unique advantages that align closely with the demands of real-time applications, particularly for projects focused on performance, security, and concurrency. Known for its memory safety guarantees and low-level control, Rust provides the tools to build applications that run efficiently and handle complex, high-concurrency situations with ease. This is particularly critical for a chat application that will be managing large volumes of messages, users, and dynamic data in real time.

We are motivated to develop this real-time chat application because it combines technical challenges with the potential for impactful results. Building a high-performance, real-time communication platform from the ground up allows us to apply and expand our skills in systems programming, concurrency, and secure coding practices. Unlike a static or transactional application, a real-time chat application requires that each component—from backend server handling to message routing and presence detection—works seamlessly together to deliver a cohesive user experience.

Additionally, this project appeals to us because it’s not only technically rewarding but also deeply practical. Chat applications are widely used, and users have high expectations for speed, security, and reliability. By working on this project, we’re not only building an application but also learning how to create software that can meet the high standards of modern users. Real-time messaging is inherently complex, and delivering a smooth, reliable experience for multiple users requires solving complex challenges in memory management, error handling, and data consistency.

In conclusion, we are excited to build this real-time chat application in Rust because it represents the perfect blend of technical challenge and user impact. With Rust’s strengths, we are confident that we can deliver an application that not only performs well under heavy loads but also offers a smooth and secure experience for users. This project is our commitment to creating a reliable, high-performance chat solution that demonstrates the potential of Rust in the realm of real-time communication.

## Objective and key features
The objective of this project is to create a scalable real-time chat application using Rust, providing a straightforward platform where users can engage in instant messaging within dedicated chat rooms. Though real-time applications aren’t entirely new, examples implemented with Rust, particularly using WebSocket with Actix Web, are still relatively few. By focusing on this specific setup, our project contributes a useful example for other Rust developers interested in basic real-time communication and provides a practical application of WebSocket within Rust’s ecosystem.

As an exploratory project, this chat application highlights Rust’s potential to handle simple, efficient real-time messaging and introduces a structured approach to managing concurrent users and chat rooms using asynchronous programming. The Rust ecosystem has solid support for backend frameworks and system-level programming but fewer examples geared towards web-based real-time communication. Our project aims to add to the collective knowledge base, showing how Actix Web can be applied for interactive, concurrent applications.

Through this project, we hope to provide an easy-to-understand reference for developers who want to explore basic real-time communication in Rust. This contribution, while modest in scope, is a step toward diversifying the applications Rust is known for, demonstrating its suitability for lightweight, real-time interactions.

### Key Features
#### 1.	User Authentication
A reliable authentication system is the backbone of any chat application, ensuring that users have secure access to the platform. We plan to implement a basic authentication feature where users can register and log in with a username and password, allowing us to maintain secure, individualized access to the chat rooms. This feature not only enhances security but also supports user-based functionalities like tracking active users in each chat room.
#### 2.	Chat Room Management
Users will have the ability to create and join chat rooms, which are spaces for group conversations centered around specific topics or interests. Each room will have a unique access code, making it easy for users to join existing rooms or create new ones if none match their current needs. The chat room management feature is central to the user experience, providing a flexible and dynamic environment for real-time communication.
#### 3.	Real-time Messaging Using WebSocket
At the core of our application is the real-time messaging feature, enabling instant message delivery with minimal delay. WebSocket will facilitate a two-way communication channel between the server and the client, allowing messages to be delivered and received in real time without constant polling. This approach enhances both the performance and responsiveness of the application, ensuring a seamless chat experience even with multiple users in a single room.
#### 4.	Presence Detection
Presence detection is a valuable feature that enhances user engagement by displaying the online/offline status of users within each chat room. By tracking and updating each user’s connection status, the application provides a more dynamic chat environment where users can easily identify who is active in a chat room. This feature is implemented in tandem with user authentication, enabling efficient status updates that contribute to a community-like atmosphere.
#### 5.	Command-line Interface (CLI) as a Front-End
For this project, we are initially implementing a simple command-line interface (CLI) to facilitate user interactions. The CLI will support essential functions, including logging in, creating and joining chat rooms, and sending and receiving messages. This approach allows us to concentrate on the backend functionalities and optimize performance while still providing a straightforward interface for user interactions. As an optional extension, we may later incorporate a Yew-based front-end to provide a more visual experience.

## Tentative plan
Our team will design and develop a scalable real-time chat application focusing on performance and efficient communication between frontend and backend using **Yew** and **Actix Web**. **GitHub** will be used as our primary development and collaboration tool for version control, task management, and code reviews.

### 1. Frontend Development with Yew  
**Frontend Lead**: Zeying Zhou
- Create a user interface in Yew, including:
  - **Sign-In and Sign-Up pages** for user authentication.
  - Chat room creation, navigation, message input, and online presence display.
- Integrate WebSocket communication to sync with backend in real time.

### 2. Backend Development with Actix Web  
**Backend Lead**: Renchao Wu

- Set up a SQL database to store user, chat room, message information.
- Implement user authentication (Sign-In, Sign-Up) and chat room management (creation, joining) with Actix Web.
- Set up WebSocket handling in Actix Web for real-time messaging.
- Build presence detection for online/offline status, optimizing server-side handling for performance.

#### Database Schema
Our relational database will manage user data, chat rooms, messages, and online presence. Here’s an overview of the schema.


| Table             | Columns                                | Description                                                                                  |
|-------------------|---------------------------------------|----------------------------------------------------------------------------------------------|
| **Users**         | `id` (PK), `username`, `password_hash`, `status`, `last_seen` | Stores user information, hashed passwords, online status, and last seen time.               |
| **Chat Rooms**    | `id` (PK), `name`, `created_at`       | Manages unique chat rooms, storing room names and creation timestamps.                       |
| **User-Chat Room Membership** | `user_id` (FK), `room_id` (FK) | Links users to the chat rooms they join (many-to-many relationship).                        |
| **Messages**      | `id` (PK), `room_id` (FK), `user_id` (FK), `content`, `timestamp` | Stores chat messages, linking each message to a room and user, with a timestamp.   

**Notes**:
- **PK** (Primary Key): A unique identifier for each record in a table, ensuring each entry is distinct and easily referenced.
- **FK** (Foreign Key): A field that links to the primary key in another table, establishing relationships between tables.

### 3. Testing and Optimization  
**Quality Assurance**: Danyang Zhang

- Use Postman to test the APIs
- Conduct manual testing of user authentication, WebSocket connections, and real-time messaging functionality.

### 4. Documentation and Coordination  
**Project Manager**: Zeying Zhou

- Manage team tasks, maintain documentation, and oversee the final code handoff.
- Use GitHub for version control, issue tracking, and code reviews to ensure smooth collaboration and code integration among team members.

### 5. Task Distribution
To streamline development, we will break down the project workload into specific tickets and assign responsibilities as follows:

| Ticket                               | Description                                               | Assigned To    |
|--------------------------------------|-----------------------------------------------------------|----------------|
| **Database Setup**                   | Design and set up database tables for users, messages, and rooms | Zeying Zhou   |
| **User Authentication**              | Implement Sign-In and Sign-Up functionality                | Zeying Zhou    |
| **Chat Room Management**             | Develop APIs for creating and joining chat rooms          | Danyang Zhang    |
| **WebSocket Integration (Backend)**  | Set up WebSocket handling for real-time messaging         | Renchao Wu    |
| **Presence Detection**               | Implement online/offline status tracking                  | Renchao Wu    |
| **Sign-In/Sign-Up UI**               | Design frontend UI for user authentication                | Zeying Zhou   |
| **Chat Room UI**                     | Design UI for chat room creation and navigation           | Danyang Zhang   |
| **Messaging Interface**              | Implement UI for real-time message input and display      | Danyang Zhang   |
| **WebSocket Integration (Frontend)** | Connect frontend WebSocket to backend for real-time sync  | Renchao Wu   |
| **Manual Testing**                   | Test authentication, WebSocket connections, and messaging | Zeying Zhou |

### Timeline
The tasks outlined above are expected to be completed in the next **four weeks**, focusing on core functionality. An additional **two weeks** will be allocated for improving the user interface and conducting thorough testing to ensure stability and usability.


---

