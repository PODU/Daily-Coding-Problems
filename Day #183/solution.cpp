// Day 183: Open-ended systems question -- "what happens when you type a URL and press Enter".
// Concrete interpretation: print the ordered pipeline of events. Time/Space O(1).
#include <bits/stdc++.h>
using namespace std;

int main() {
    const char* steps[] = {
        "1. URL parsing: scheme, host, port, path, query are extracted from the typed URL.",
        "2. Browser checks caches (HSTS, in-memory, disk) and may auto-upgrade http -> https.",
        "3. DNS resolution: hostname -> IP via OS cache, hosts file, then recursive resolver.",
        "4. TCP handshake (SYN, SYN-ACK, ACK) opens a connection to the server IP:port.",
        "5. TLS handshake for https: certificate validation, key exchange, secure session.",
        "6. HTTP request sent (e.g. GET /path with headers, cookies, Host).",
        "7. Server processing: routing, app logic, DB/cache access; returns an HTTP response.",
        "8. Browser receives status, headers, and body (often gzip/br compressed).",
        "9. Rendering: parse HTML -> DOM, CSS -> CSSOM, build render tree, layout, paint.",
        "10. Subresources (CSS, JS, images, fonts) fetched; JS executes and can mutate the DOM.",
        "11. Page becomes interactive; further XHR/fetch requests may load more data."
    };
    for (auto s : steps) cout << s << "\n";
    return 0;
}
