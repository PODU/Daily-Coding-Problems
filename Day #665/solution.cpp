// Day 665: URL shortener. Base62-encode an incrementing counter into a 6-char code;
// dedup with url->code map so the same URL maps once. shorten/restore O(1) avg.
#include <bits/stdc++.h>
using namespace std;

struct Shortener {
    const string A = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    unordered_map<string, string> code2url, url2code;
    long long counter = 916132831; // seeds a 6-char base62 code

    string encode(long long n) {
        string s;
        for (int i = 0; i < 6; i++) { s += A[n % 62]; n /= 62; }
        reverse(s.begin(), s.end());
        return s;
    }
    string shorten(const string& url) {
        auto it = url2code.find(url);
        if (it != url2code.end()) return it->second;
        string code = encode(counter++);
        code2url[code] = url; url2code[url] = code;
        return code;
    }
    string restore(const string& code) {
        auto it = code2url.find(code);
        return it == code2url.end() ? string("null") : it->second;
    }
};

int main() {
    Shortener s;
    string c = s.shorten("https://example.com/long/path");
    cout << "short: " << c << "\n";                    // 6 chars
    cout << "restore: " << s.restore(c) << "\n";       // original url
    cout << "restore(zzzzzz): " << s.restore("zzzzzz") << "\n"; // null
    return 0;
}
