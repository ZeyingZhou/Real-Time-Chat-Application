# Real-Time Chat Final Report
Name: Zeying Zhou Student#: 1005172732  zeying.zhou@mail.utoronto.ca

Name: Danyang Zhang Student#: 1005087022 danyang.zhang@mail.utoronto.ca

Name: Renchao Wu Student#: 1009750672 renchao.wu@mail.utoronto.ca

## Video Demo

## Motivation

Real-time chat applications have evolved into essential tools for modern communication, underpinning interactions in social, professional, and support environments worldwide. Starting from basic messaging systems, the field has witnessed the rise of sophisticated platforms like IRC (Internet Relay Chat) and the progression to highly interactive, feature-rich applications such as Slack, Discord, and Microsoft Teams. These advancements have redefined how people collaborate and communicate in real time, with applications that deliver not only messages but also a sense of connection, immediacy, and productivity.

The initial versions of chat applications were constrained by limited technology and infrastructure, offering only the most basic functionality. Over time, however, demand surged for real-time data transfer, stable performance, and high scalability. Today’s communication needs require chat applications that are extremely efficient, reliable, and responsive even under high user loads. Users expect instant delivery of messages, seamless presence updates, and a smooth user experience without lag. This high standard poses a challenge to developers, especially as chat applications must now balance scalability with security, maintain high performance across multiple users and devices, and address the increasing complexity of concurrent message exchanges.

Many of the current chat platforms are built using languages like JavaScript, Python, or Java, each of which has established itself as a reliable choice for building web applications. However, these languages, while powerful, have inherent limitations when it comes to maximizing performance and security, particularly at the system level. They often struggle to manage memory as efficiently, and ensuring both speed and stability under high load can require significant workarounds. This is where Rust, a systems programming language known for its safety and efficiency, presents a compelling alternative.

Rust offers unique advantages that align closely with the demands of real-time applications, particularly for projects focused on performance, security, and concurrency. Known for its memory safety guarantees and low-level control, Rust provides the tools to build applications that run efficiently and handle complex, high-concurrency situations with ease. This is particularly critical for a chat application that will be managing large volumes of messages, users, and dynamic data in real time.

We are motivated to develop this real-time chat application because it combines technical challenges with the potential for impactful results. Building a high-performance, real-time communication platform from the ground up allows us to apply and expand our skills in systems programming, concurrency, and secure coding practices. Unlike a static or transactional application, a real-time chat application requires that each component—from backend server handling to message routing and presence detection—works seamlessly together to deliver a cohesive user experience.

Additionally, this project appeals to us because it’s not only technically rewarding but also deeply practical. Chat applications are widely used, and users have high expectations for speed, security, and reliability. By working on this project, we’re not only building an application but also learning how to create software that can meet the high standards of modern users. Real-time messaging is inherently complex, and delivering a smooth, reliable experience for multiple users requires solving complex challenges in memory management, error handling, and data consistency.

In conclusion, we are excited to build this real-time chat application in Rust because it represents the perfect blend of technical challenge and user impact. With Rust’s strengths, we are confident that we can deliver an application that not only performs well under heavy loads but also offers a smooth and secure experience for users. This project is our commitment to creating a reliable, high-performance chat solution that demonstrates the potential of Rust in the realm of real-time communication.

## Objectives
The objective of this project is to create a scalable real-time chat application using Rust, providing a straightforward platform where users can engage in instant messaging within dedicated chat rooms. Though real-time applications aren’t entirely new, examples implemented with Rust, particularly using WebSocket with Actix Web, are still relatively few. By focusing on this specific setup, our project contributes a useful example for other Rust developers interested in basic real-time communication and provides a practical application of WebSocket within Rust’s ecosystem.

As an exploratory project, this chat application highlights Rust’s potential to handle simple, efficient real-time messaging and introduces a structured approach to managing concurrent users and chat rooms using asynchronous programming. The Rust ecosystem has solid support for backend frameworks and system-level programming but fewer examples geared towards web-based real-time communication. Our project aims to add to the collective knowledge base, showing how Actix Web can be applied for interactive, concurrent applications.

Through this project, we hope to provide an easy-to-understand reference for developers who want to explore basic real-time communication in Rust. This contribution, while modest in scope, is a step toward diversifying the applications Rust is known for, demonstrating its suitability for lightweight, real-time interactions.
## Features
1. User Authentication

* Users can register and log in with a username and password.
* Passwords are hashed and stored securely in the database.

2. Chat Room Management

* Users can create or join chat rooms using unique access codes.
* Each chat room is isolated, with real-time message delivery and presence tracking.

3. Real-Time Messaging

* WebSocket integration enables instant two-way communication between the client and server.
* Messages are delivered in real time with minimal latency.

4. Presence Detection
* Tracks user activity within each chat room.
* Displays online/offline status for all participants.

5. Simple Front-End
* A lightweight frontend for sending and receiving messages, managing chat rooms, and sign in and sign up.


## Developer's Guide
## Reproducibility Guide
First, you need to download or clone the project into your own folder.
```
git clone https://github.com/ZeyingZhou/Real-Time-Chat-Application.git
```
After download or clone the repository, use terminal to perform following commands to start the project
### Frontend
Go to the frontend project folder in terminal
```
cd Real-Time-Chat-Application/real-time-chat-app-frontend/
```
Add the WebAssembly (WASM) target for Yew frontend
```
rustup target add wasm32-unknown-unknown
```
Install Trunk
```
# Install Trunk for building and serving the Yew frontend
# note that this might take a while to install because it compiles everything from scratch
# Trunk also provides prebuilt binaries for a number of major package managers
# See https://trunkrs.dev/#install for further details
cargo install --locked trunk
```
Run trunk build to build everything
```
trunk build
```
Start the frontend project
```
# Start the frontend project
trunk serve
```


### Backend
1. Environment Setup
* Operating System: Linux/Windows/MacOS.
* Rust version: 1.70+ (use rustup show to verify).
* SQLite3: Ensure it’s installed and accessible via terminal.
if it's not install run following commands based on your system
* Ubuntu/Debian:
```
sudo apt update
sudo apt install libsqlite3-dev sqlite3
```
* macOS
```
brew install sqlite
```
After clone the repository run following command to navigate to the backend folder
```
cd Real-Time-Chat-Application/real-time-chat-app-backend/
```
Just simply start the backend by cargo run!
```
cargo run
```


## Contributions by Each Team Member  

| Team Member     | Contributions                                                                                   |
|-----------------|-------------------------------------------------------------------------------------------------|
| **Zeying Zhou** | Frontend development sign in, sign up, user home page and relate api; Backend development sign in, sign up, create and join room api; Database set up; Documentation for the project  
| **Danyang Zhang** | Backend development chat room api; Presence Detection ; manual testing; feature integration          |
| **Renchao Wu**  | Backend development chat room api;           |
    
## Lessons learned and concluding remarks:
This project successfully implemented an efficient real-time chat application using Rust and Actix Web, achieving WebSocket-based communication, user status management, and chatroom message broadcasting functionalities. Throughout the development process, we not only deepened our understanding of system-level programming in Rust but also honed our skills in asynchronous programming, database management, and high-concurrency system design.

While the core features have been implemented, we recognize that there remains significant room for improvement and further development. Performance optimization will be a key area of focus to ensure the application can handle even larger user loads with minimal latency. Additionally, building a more user-friendly front-end interface and introducing new features such as message history storage and private messaging will enhance the application's functionality and usability.

One of the most valuable lessons from this project was understanding how to balance system performance, reliability, and scalability in a real-world application. By combining Rust’s strengths in memory safety and concurrency management with Actix’s powerful Actor model, we were able to construct a robust and scalable architecture that can efficiently handle simultaneous user interactions.

Moreover, the challenges we faced—such as implementing smooth WebSocket connections, managing user presence dynamically, and ensuring seamless interaction with the database—pushed us to refine our problem-solving skills and adopt best practices in modern software development. This hands-on experience has given us a clearer perspective on building real-time applications that meet high-performance standards.
    
    
