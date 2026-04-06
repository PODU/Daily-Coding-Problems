// Soundex phonetic encoding: keep first letter, map consonants to digits,
// collapse same-code runs, drop vowels, pad/truncate to 3 digits. O(n) time.
#include <bits/stdc++.h>
using namespace std;

int code(char c) {
    switch (tolower(c)) {
        case 'b': case 'f': case 'p': case 'v': return 1;
        case 'c': case 'g': case 'j': case 'k': case 'q':
        case 's': case 'x': case 'z': return 2;
        case 'd': case 't': return 3;
        case 'l': return 4;
        case 'm': case 'n': return 5;
        case 'r': return 6;
        default: return 0; // vowels, y, h, w
    }
}

string soundex(const string& name) {
    if (name.empty()) return "";
    string out(1, toupper(name[0]));
    int prev = code(name[0]);
    for (size_t i = 1; i < name.size() && out.size() < 4; ++i) {
        char ch = tolower(name[i]);
        int c = code(ch);
        if (c != 0 && c != prev) out += char('0' + c);
        if (ch == 'h' || ch == 'w') continue; // do not reset prev
        prev = c;                              // vowels reset prev to 0
    }
    while (out.size() < 4) out += '0';
    return out.substr(0, 4);
}

int main() {
    cout << soundex("Jackson") << "\n"; // J250
    cout << soundex("Jaxen") << "\n";   // J250
    return 0;
}
