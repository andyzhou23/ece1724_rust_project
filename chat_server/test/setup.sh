#!/bin/bash
# Kill any existing server process
pkill -f "../target/debug/chat_server"
sleep 1
rm -f server.db
cargo run
