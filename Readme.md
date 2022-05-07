### Properties HTTP/1.1
#### L7 Protocol, sent over TCP, message based

### Architecture
#### Server:
- TCP Listener: listens on port 80, r/w to client
- HTTP Parser: parses HTTP requests
- HTTP Handler: routing and dispatching requests, handling responses to client
- Single request at a time, single thread.