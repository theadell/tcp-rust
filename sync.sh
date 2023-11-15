#!/bin/bash

SOURCE_DIR="."
DEST_DIR="root@node-1:dev/trust"
EXCLUDE='exclude.txt'
DELAY=2  # Delay in seconds

# Function to perform rsync
do_rsync() {
    rsync -avz --exclude-from "$EXCLUDE" "$SOURCE_DIR" "$DEST_DIR"
}

# Function to batch file change events
sync_with_delay() {
    while true; do
        # Read all available events (up to a limit of 1000) within the delay period
        read -t "$DELAY" -r -u 9 -n 1000 event && {
            # Wait for the end of the batch
            read -t "$DELAY" -r -u 9 -n 1000
            do_rsync
        } || true
    done 9< <(fswatch -o "$SOURCE_DIR")
}

# Initial sync
do_rsync

# Watch for changes and sync with delay
sync_with_delay

