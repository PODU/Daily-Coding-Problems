// Day 1424: URL shortener (shorten -> 6-char code, restore -> original or null).
// Approach: two hash maps + base62 counter; same URL reuses its code. O(1) amortized per op.
#include <bits/stdc++.h>
using namespace std;

class URLShortener {
    const string ALPHABET = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    long long counter = 0;
    unordered_map<string,string> urlToShort, shortToUrl;

    string encode(long long n) {
        string s(6, '0');
        for (int i = 5; i >= 0; --i) { s[i] = ALPHABET[n % 62]; n /= 62; }
        return s;
    }
public:
    string shorten(const string& url) {
        auto it = urlToShort.find(url);
        if (it != urlToShort.end()) return it->second; // same URL -> same code
        string code = encode(counter++);
        urlToShort[url] = code;
        shortToUrl[code] = url;
        return code;
    }
    string restore(const string& code) {
        auto it = shortToUrl.find(code);
        return it == shortToUrl.end() ? string("null") : it->second;
    }
};

int main() {
    URLShortener s;
    string a = s.shorten("https://example.com/page");
    string b = s.shorten("https://example.com/page");
    cout << a << "\n";                       // 000000
    cout << (a == b ? "true" : "false") << "\n"; // true (same URL -> same code)
    cout << s.restore(a) << "\n";            // https://example.com/page
    cout << s.restore("zzzzzz") << "\n";     // null
    return 0;
}
