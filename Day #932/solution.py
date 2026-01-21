# Day 932: Open-ended systems question. We print the ordered sequence of events
# that happen from typing a URL and pressing Enter until the page renders.
# Time: O(k) to print k steps, Space: O(1).

STEPS = [
    "1. URL parsing: browser splits the URL into scheme, host, port, path, query.",
    "2. HSTS / cache checks: browser checks HSTS list, then its own cache.",
    "3. DNS resolution: hostname -> IP via OS cache, resolver, recursive DNS (UDP/TCP 53).",
    "4. TCP connection: 3-way handshake (SYN, SYN-ACK, ACK) to the server IP:port.",
    "5. TLS handshake (for https): negotiate cipher, exchange certs/keys, establish session.",
    "6. HTTP request: browser sends e.g. 'GET / HTTP/1.1' with headers (Host, Cookie, ...).",
    "7. Server processing: load balancer/app server builds and returns an HTTP response.",
    "8. HTTP response: status line, headers, and body (HTML) travel back to the browser.",
    "9. Parsing: browser parses HTML into the DOM, CSS into the CSSOM, builds the render tree.",
    "10. Subresources: fetch CSS/JS/images/fonts (more requests, possibly more connections).",
    "11. JavaScript execution: scripts run, may mutate the DOM and trigger reflows.",
    "12. Layout & paint: compute geometry, paint layers, composite, and display the page.",
]

if __name__ == "__main__":
    for s in STEPS:
        print(s)
