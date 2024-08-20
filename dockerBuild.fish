#!/usr/bin/env fish

if not test -d ./Database/scylla
    mkdir -p ./Database/scylla
    echo "Created directory ./Database/scylla/"
end
# Start Docker Compose
docker compose up -d
# Wait for ScyllaDB to be ready
while true
    set result (docker exec scylla-node1 cqlsh -e "DESCRIBE KEYSPACES;" 2>/dev/null)
    if test -n "$result"
        echo "Keyspace exists!"
        break
    else
        echo "Waiting for keyspace to be available..."
        sleep 5
    end
end

# Copy the initialization script
docker cp (pwd)/schema/scylla_init.txt scylla-node1:/tmp/scylla_init.txt

# Execute the initialization script
docker exec -i scylla-node1 cqlsh -f /tmp/scylla_init.txt
