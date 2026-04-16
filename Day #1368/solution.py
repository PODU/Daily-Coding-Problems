# Day 1368: Open-ended systems question: walk through events after typing a URL + Enter.
# "Algorithm": print the ordered chain of events. Time O(n), Space O(n) in steps.

STEPS = [
    "1. URL parsing: browser splits the URL into scheme, host, port, path, query, fragment.",
    "2. HSTS / cache check: browser checks HSTS list and its own cache for the resource.",
    "3. DNS resolution: hostname -> IP via browser/OS cache, then recursive resolver (UDP/53, or DoH).",
    "4. TCP handshake: 3-way SYN, SYN-ACK, ACK opens a connection to the server IP:port.",
    "5. TLS handshake (https): negotiate cipher, exchange certs, derive session keys.",
    "6. HTTP request: browser sends GET request with headers (Host, cookies, etc.).",
    "7. Server processing: load balancer/app server build a response, maybe hitting a DB/cache.",
    "8. HTTP response: status line, headers, and body (HTML) stream back to the browser.",
    "9. Parsing & rendering: HTML -> DOM, CSS -> CSSOM, combine into render tree.",
    "10. Subresources: fetch CSS/JS/images/fonts (often multiplexed over HTTP/2 or HTTP/3).",
    "11. JS execution: scripts run, may mutate DOM, trigger reflow/repaint.",
    "12. Layout & paint & composite: pixels drawn to the screen; page becomes interactive.",
]

if __name__ == "__main__":
    for s in STEPS:
        print(s)
