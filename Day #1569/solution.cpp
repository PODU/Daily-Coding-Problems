// URL shortener: base62 6-char codes from a counter, two hash maps + reverse map for dedupe.
// Time O(1) per shorten/restore (amortized), space O(n).
#include <bits/stdc++.h>
using namespace std;

class URLShortener {
    static const string ALPHABET;
    long long counter;
    unordered_map<string,string> codeToUrl;
    unordered_map<string,string> urlToCode;

    string encode(long long num) {
        string s(6, '0');
        for (int i = 5; i >= 0; i--) { s[i] = ALPHABET[num % 62]; num /= 62; }
        return s;
    }
    long long decode(const string& s) {
        long long num = 0;
        for (char c : s) num = num * 62 + ALPHABET.find(c);
        return num;
    }
public:
    URLShortener() { counter = decode("abcdef"); }

    string shorten(const string& url) {
        auto it = urlToCode.find(url);
        if (it != urlToCode.end()) return it->second;
        string code = encode(counter++);
        codeToUrl[code] = url;
        urlToCode[url] = code;
        return code;
    }
    // returns empty-string-as-null sentinel handling done by caller
    bool restore(const string& code, string& out) {
        auto it = codeToUrl.find(code);
        if (it == codeToUrl.end()) return false;
        out = it->second; return true;
    }
};
const string URLShortener::ALPHABET =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

int main() {
    URLShortener s;
    string code = s.shorten("https://www.example.com/some/long/path");
    cout << code << endl;
    string out;
    cout << (s.restore(code, out) ? out : "null") << endl;
    cout << (s.restore("XXXXXX", out) ? out : "null") << endl;
    return 0;
}
