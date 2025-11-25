# Day 659: Approach: enumerate the standard request lifecycle as an ordered list.
# Time O(k), Space O(k), where k = number of steps.

def steps():
    return [
        "Parse and normalize the URL (scheme, host, port, path, query).",
        "Check HSTS / browser caches (memory, disk, service worker).",
        "Resolve the domain to an IP via DNS (browser cache -> OS cache -> resolver -> root/TLD/authoritative).",
        "Establish a TCP connection (three-way handshake: SYN, SYN-ACK, ACK).",
        "Perform the TLS handshake for HTTPS (certificate validation, key exchange).",
        "Send the HTTP request (method, headers, cookies).",
        "Server processes the request and returns an HTTP response (status, headers, body).",
        "Browser handles the response (redirects, caching, content type).",
        "Parse HTML to build the DOM and CSS to build the CSSOM.",
        "Construct the render tree, then layout (reflow) and paint/composite to pixels.",
        "Execute JavaScript, which may trigger further requests and DOM updates.",
        "Connection kept alive or closed; page becomes interactive.",
    ]


def main():
    for i, s in enumerate(steps(), 1):
        print(f"{i}. {s}")


if __name__ == "__main__":
    main()
