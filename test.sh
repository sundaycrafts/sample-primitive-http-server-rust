#!/usr/bin/env sh

# Ensure the cargo run process is terminated when the script exits
cleanup() {
  kill "$cargo_pid" 2>/dev/null
}

# Register the cleanup function to be called on script exit
trap cleanup EXIT

tmp=$(mktemp "${TMPDIR}log-XXXXXX")

# 1. Run the Rust project in the background
cargo run > "$tmp" 2>&1 &
# Save the process ID of the cargo run command
cargo_pid=$!

# Wait for the server to start
sleep 2

# 2. Send a request to the localhost port 8080 (avoid exit status)
(curl http://localhost:8080 --request POST --data '{ "name": "John" }')

# 3. Check if the cargo run process has not exited with a non-zero exit code
kill "$cargo_pid"
exit_code=$?

# 4. If it has exited with a non-zero exit code, print an error message and exit with 1
if [ $exit_code -ne 0 ]; then
  echo "Error: cargo run process exited with a non-zero exit code: $exit_code"
  exit 1
else
  echo "test succeeded"
  cat "$tmp"
  exit 0
fi

# 5. The cargo run process will be terminated by the cleanup function, regardless of the success or failure of step 4
