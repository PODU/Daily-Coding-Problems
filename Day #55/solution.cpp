// Day 55: URL shortener. 6-char base62 code; same URL maps to same code.
// Time: O(1) amortized per op, Space: O(n).
#include <bits/stdc++.h>
using namespace std;

class URLShortener {
    static constexpr const char* ALPHA =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    unordered_map<string, string> toLong, toShort;
    mt19937 rng{random_device{}()};

    string randCode() {
        string s(6, ' ');
        uniform_int_distribution<int> d(0, 61);
        for (char& c : s) c = ALPHA[d(rng)];
        return s;
    }
public:
    string shorten(const string& url) {
        auto it = toShort.find(url);
        if (it != toShort.end()) return it->second; // same URL -> same code
        string code;
        do { code = randCode(); } while (toLong.count(code));
        toLong[code] = url;
        toShort[url] = code;
        return code;
    }
    // Returns the original URL, or "" (null) if the code is unknown.
    string restore(const string& code) {
        auto it = toLong.find(code);
        return it == toLong.end() ? string() : it->second;
    }
};

int main() {
    URLShortener s;
    string a = s.shorten("https://example.com/foo");
    string b = s.shorten("https://example.com/foo"); // same URL twice
    cout << "same code reused: " << (a == b ? "true" : "false") << endl;
    cout << "restore: " << s.restore(a) << endl;
    cout << "restore unknown: " << (s.restore("zzzzzz").empty() ? "null" : s.restore("zzzzzz")) << endl;
    return 0;
}
