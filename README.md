# plan10
From virtual space

## Current Stack Idea
* Postgresql
    * https://www.postgresql.org/
* WebAssembly
    * https://webassembly.github.io/spec/core/intro/index.html
* Zstd Compression
    * https://facebook.github.io/zstd/

## Project Scope
The goal of this project is to test the waters for a boilerplate metaverse that others may extend to construct whatever kind of shared creator/consumer platform they desire.

### Servers
* Establish an address whereby clients can connect and send commands
* Handle incoming commands from clients as well as communicate command results back

### Clients
* Connect to servers
* Send commands to servers
  * Send messages to other clients
  * Broadcast messages to all clients in the server
  * Upload files to the server db for other uses to discover
  * Send files to other clients
  * Request contents from server db
* Access local db

### Local Database
* Store binary files with a header for metadata
  * The type of file should be in the metadata
* Read files
* Edit files
* Delete files
* Move files?
* Export files
