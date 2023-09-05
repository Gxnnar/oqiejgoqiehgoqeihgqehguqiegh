CREATE TABLE IF NOT EXISTS requests (
    -- IP address of the client
    client_address TEXT NOT NULL,

    -- Request
    request_method TEXT NOT NULL,
    request_url TEXT NOT NULL,
    request_version TEXT NOT NULL,
    request_headers TEXT NOT NULL,
    request_body TEXT NOT NULL,
    
    -- Response status line
    response_status TEXT NOT NULL,
    -- Time to first byte
    response_latency_ms INTEGER NOT NULL,

    -- Date of the request
    date INTEGER NOT NULL
)