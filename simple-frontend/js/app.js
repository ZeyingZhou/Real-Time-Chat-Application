document.addEventListener('DOMContentLoaded', () => {
    const apiBase = 'http://127.0.0.1:8000/api';
    let userId = null;
    let username = '';
    let currentRoomId = null;
    let ws = null;

    document.getElementById('show-signin').addEventListener('click', () => {
        document.getElementById('signup-form').classList.add('hidden');
        document.getElementById('signin-form').classList.remove('hidden');
    });

    document.getElementById('show-signup').addEventListener('click', () => {
        document.getElementById('signin-form').classList.add('hidden');
        document.getElementById('signup-form').classList.remove('hidden');
    });

    document.getElementById('form-signup').addEventListener('submit', async (e) => {
        e.preventDefault();
        const signupUsername = document.getElementById('signup-username').value;
        const signupPassword = document.getElementById('signup-password').value;
    
        try {
            const response = await fetch(`${apiBase}/auth/signup`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ username: signupUsername, password: signupPassword })
            });
    
            if (response.ok) {
                const data = await response.json(); 
                alert('Sign up successfully');
                document.getElementById('signup-form').classList.add('hidden');
                document.getElementById('signin-form').classList.remove('hidden');
            } else {
                
                const errorText = await response.text();
                alert(`Sign up failed: ${errorText}`);
            }
        } catch (error) {
            console.error('Wrong message:', error);
            alert('Sign up failed');
        }
    });

    document.getElementById('form-signin').addEventListener('submit', async (e) => {
        e.preventDefault();
        const signinUsername = document.getElementById('signin-username').value;
        const signinPassword = document.getElementById('signin-password').value;

        try {
            const response = await fetch(`${apiBase}/auth/signin`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ username: signinUsername, password: signinPassword })
            });

            const data = await response.json();
            if (response.ok) {
                userId = data.user_id;
                username = signinUsername;
                document.getElementById('signin-form').classList.add('hidden');
                document.getElementById('main-section').classList.remove('hidden');
                document.getElementById('user-name').textContent = username;
                loadChatRooms();
                loadJoinedRooms();
            } else {
                alert(`Log in failed: ${data.message || response.statusText}`);
            }
        } catch (error) {
            console.error('Log in failed:', error);
            alert('Log in failed');
        }
    });

    document.getElementById('logout-button').addEventListener('click', () => {
        userId = null;
        username = '';
        currentRoomId = null;
        if (ws) {
            ws.close();
        }
        document.getElementById('main-section').classList.add('hidden');
        document.getElementById('signup-form').classList.remove('hidden');
    });

    async function loadChatRooms() {
        try {
            const response = await fetch(`${apiBase}/chat_rooms`);
            if (response.ok) {
                const chatRooms = await response.json();
                const select = document.getElementById('chat-rooms-list');
                select.innerHTML = '<option value="">Select Chatroom</option>';
                chatRooms.forEach(room => {
                    const option = document.createElement('option');
                    option.value = room.id;
                    option.textContent = room.name;
                    select.appendChild(option);
                });
            } else {
                console.error('get chatroom name failed');
            }
        } catch (error) {
            console.error('get chatroom name failed:', error);
        }
    }

    document.getElementById('form-create-room').addEventListener('submit', async (e) => {
        e.preventDefault();
        const roomName = document.getElementById('new-room-name').value;

        try {
            const response = await fetch(`${apiBase}/chat_rooms`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ name: roomName, user_id: userId })
            });

            const data = await response.json();
            if (response.ok) {
                alert('Create successfully');
                loadChatRooms();
                loadJoinedRooms();
            } else {
                alert(`Create failed: ${data.message || response.statusText}`);
            }
        } catch (error) {
            console.error('Create failed:', error);
            alert('Create failed');
        }
    });

    document.getElementById('join-room-button').addEventListener('click', async () => {
        const roomId = document.getElementById('chat-rooms-list').value;
        if (!roomId) {
            alert('Select chatroom');
            return;
        }

        try {
            const response = await fetch(`${apiBase}/chat_rooms/join`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ room_id: parseInt(roomId), user_id: userId })
            });

            if (response.ok) {
                alert('Join successfully');
                loadJoinedRooms();
            } else {
                const data = await response.json();
                alert(`Join failed: ${data.message || response.statusText}`);
            }
        } catch (error) {
            console.error('Join failed:', error);
            alert('Join failed');
        }
    });

    async function loadJoinedRooms() {
        try {
            const response = await fetch(`${apiBase}/users/${userId}`);
            if (response.ok) {
                const data = await response.json();
                const rooms = data.data.chat_rooms;
                const list = document.getElementById('joined-rooms-list');
                list.innerHTML = '';
                rooms.forEach(room => {
                    const li = document.createElement('li');
                    li.textContent = room.name;
                    const joinBtn = document.createElement('button');
                    joinBtn.textContent = 'Join';
                    joinBtn.style.marginLeft = '10px';
                    joinBtn.addEventListener('click', () => joinRoom(room.id, room.name));
                    li.appendChild(joinBtn);
                    list.appendChild(li);
                });
            } else {
                alert('Unable to load joined chat rooms ');
            }
        } catch (error) {
            console.error('Unable to load joined chat rooms:', error);
            alert('Unable to load joined chat rooms');
        }
    }

    function joinRoom(roomId, roomName) {
        if (currentRoomId) {
            alert('have already joined');
            return;
        }

        currentRoomId = roomId;
        document.getElementById('current-room-name').textContent = roomName;
        document.getElementById('chat-section-container').classList.remove('hidden');
        connectWebSocket(roomId);
    }

    document.getElementById('leave-room-button').addEventListener('click', () => {
        if (ws) {
            ws.close();
        }
        currentRoomId = null;
        document.getElementById('chat-section-container').classList.add('hidden');
        document.getElementById('chat-section').innerHTML = '';
    });

    function connectWebSocket(roomId) {
        ws = new WebSocket(`ws://127.0.0.1:8000/ws/${roomId}/${userId}`);

        ws.onopen = () => {
            console.log('WebSocket connected');
        };

        ws.onmessage = (event) => {
            const chatSection = document.getElementById('chat-section');
            const msg = document.createElement('div');
            msg.textContent = event.data;
            chatSection.appendChild(msg);
            chatSection.scrollTop = chatSection.scrollHeight;
        };

        ws.onclose = () => {
            console.log('WebSocket closed');
        };

        ws.onerror = (error) => {
            console.error('WebSocket wrong:', error);
        };
    }

    document.getElementById('chat-form').addEventListener('submit', (e) => {
        e.preventDefault();
        const messageInput = document.getElementById('chat-message');
        const message = messageInput.value.trim();
        if (message && ws) {
            ws.send(message);
            messageInput.value = '';
        }
    });
});
