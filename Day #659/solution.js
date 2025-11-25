// Approach: enumerate the standard request lifecycle as an ordered list.
// Time O(k), Space O(k), where k = number of steps.

function steps() {
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
  ];
}

function main() {
  steps().forEach((s, i) => console.log(`${i + 1}. ${s}`));
}

main();
