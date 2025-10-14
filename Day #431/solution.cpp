// Day 431: Sentence validator via finite-state-machine scan over Unicode codepoints.
// Mirrors regex ^[A-Z][a-z]*([,;:]? [a-z]+)*[,;:]?[.?!‽]$ (no backtracking needed).
// O(n) time, O(n) space (UTF-8 decode) per sentence.
#include <bits/stdc++.h>
using namespace std;

static vector<unsigned> decodeUtf8(const string& s) {
    vector<unsigned> cp;
    size_t i = 0, n = s.size();
    while (i < n) {
        unsigned char c = s[i];
        unsigned val; int len;
        if (c < 0x80)            { val = c;         len = 1; }
        else if ((c >> 5) == 0x6){ val = c & 0x1F;  len = 2; }
        else if ((c >> 4) == 0xE){ val = c & 0x0F;  len = 3; }
        else if ((c >> 3) == 0x1E){ val = c & 0x07; len = 4; }
        else                     { val = c;         len = 1; }
        for (int k = 1; k < len && i + k < n; k++) val = (val << 6) | (s[i + k] & 0x3F);
        cp.push_back(val);
        i += len;
    }
    return cp;
}
static bool isSep(unsigned c)  { return c == ',' || c == ';' || c == ':'; }
static bool isTerm(unsigned c) { return c == '.' || c == '?' || c == '!' || c == 0x203D; }
static bool isLow(unsigned c)  { return c >= 'a' && c <= 'z'; }
static bool isUp(unsigned c)   { return c >= 'A' && c <= 'Z'; }

bool isValidSentence(const string& s) {
    vector<unsigned> a = decodeUtf8(s);
    int n = (int)a.size();
    if (n == 0) return false;
    if (!isUp(a[0])) return false;
    int i = 1;
    while (i < n && isLow(a[i])) i++;
    while (true) {
        int j = i;
        if (j < n && isSep(a[j])) j++;
        if (j < n && a[j] == ' ') {
            j++;
            if (j < n && isLow(a[j])) {
                while (j < n && isLow(a[j])) j++;
                i = j;
                continue;
            }
        }
        break;
    }
    if (i < n && isSep(a[i])) i++;
    return i == n - 1 && isTerm(a[i]);
}

int main() {
    vector<string> tests = {"The quick brown fox.", "hello world.", "Hello  world.",
                            "Hello world", "Hi there, friend!"};
    for (auto& t : tests)
        cout << (isValidSentence(t) ? "Valid: " : "Invalid: ") << t << "\n";
}
