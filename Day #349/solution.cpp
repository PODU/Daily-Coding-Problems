// Soundex: keep first letter, map consonants to digits, collapse same adjacent codes
// (h,w transparent; vowels reset), pad/truncate to 3 digits. Time O(L), Space O(1).
#include <iostream>
#include <string>
#include <cctype>
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
        default: return 0; // vowels, y, w, h
    }
}

string soundex(const string& name) {
    string res(1, (char)toupper(name[0]));
    int prev = code(name[0]);
    for (size_t i = 1; i < name.size() && res.size() < 4; i++) {
        char c = tolower(name[i]);
        int d = code(c);
        if (d != 0 && d != prev) res += char('0' + d);
        if (c == 'h' || c == 'w') continue; // transparent: keep prev
        prev = d;                            // vowels reset prev to 0
    }
    while (res.size() < 4) res += '0';
    return res;
}

int main() {
    cout << soundex("Jackson") << "\n";
    return 0;
}
