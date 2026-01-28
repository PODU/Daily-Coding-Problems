// URL shortener: base62-encode an incrementing counter (zero-padded to 6 chars).
// Dedup via url->code map so identical URLs map to the same code.
// shorten/restore: O(L) per call (L = code length); Space: O(N) for N stored URLs.
#include <bits/stdc++.h>
using namespace std;

class URLShortener {
    static const string ALPHA; // 62 alphanumeric chars
    long long counter = 0;
    unordered_map<string, string> urlToCode;
    unordered_map<string, string> codeToUrl;

    string encode(long long n) {
        string s(6, ALPHA[0]);
        int i = 5;
        while (n > 0 && i >= 0) {
            s[i--] = ALPHA[n % 62];
            n /= 62;
        }
        return s;
    }

public:
    string shorten(const string& url) {
        auto it = urlToCode.find(url);
        if (it != urlToCode.end()) return it->second; // same URL -> same code
        string code = encode(counter++);
        urlToCode[url] = code;
        codeToUrl[code] = url;
        return code;
    }

    // Returns the original URL, or empty optional-like sentinel "<null>" if unknown.
    bool restore(const string& code, string& out) {
        auto it = codeToUrl.find(code);
        if (it == codeToUrl.end()) return false;
        out = it->second;
        return true;
    }
};

const string URLShortener::ALPHA =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

int main() {
    URLShortener s;
    string url = "https://www.example.com/some/long/path";
    string code = s.shorten(url);
    cout << "shorten(" << url << ") = " << code << "\n";

    string out;
    if (s.restore(code, out)) cout << "restore(" << code << ") = " << out << "\n";

    if (!s.restore("zzzzzz", out)) cout << "restore(zzzzzz) = null\n";

    string code2 = s.shorten(url);
    cout << "shorten same url again = " << code2
         << " (same as before: " << (code == code2 ? "true" : "false") << ")\n";
    return 0;
}
