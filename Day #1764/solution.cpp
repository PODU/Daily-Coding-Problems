// Soundex phonetic encoding (NARA rules): keep first letter, map rest to digits,
// collapse adjacent same-codes (h/w transparent), drop vowels, pad/truncate to 3 digits.
// Time: O(n) per name, Space: O(n).
#include <bits/stdc++.h>
using namespace std;

int code(char c) {
    c = tolower(c);
    switch (c) {
        case 'b': case 'f': case 'p': case 'v': return 1;
        case 'c': case 'g': case 'j': case 'k': case 'q':
        case 's': case 'x': case 'z': return 2;
        case 'd': case 't': return 3;
        case 'l': return 4;
        case 'm': case 'n': return 5;
        case 'r': return 6;
        default: return 0; // vowels a,e,i,o,u,y and h,w
    }
}

string soundex(const string& name) {
    string s;
    for (char c : name) if (isalpha((unsigned char)c)) s += c;
    if (s.empty()) return "";
    string res;
    res += toupper(s[0]);
    int prev = code(s[0]);
    for (size_t i = 1; i < s.size() && res.size() < 4; i++) {
        int d = code(s[i]);
        char lc = tolower(s[i]);
        if (d != 0 && d != prev) res += char('0' + d);
        // h, w are transparent: don't reset prev so they don't break a run
        if (lc != 'h' && lc != 'w') prev = d;
    }
    while (res.size() < 4) res += '0';
    return res.substr(0, 4);
}

int main() {
    cout << soundex("Jackson") << "\n";
    cout << soundex("Jaxen") << "\n";
    return 0;
}
