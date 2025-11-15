// Day 604: Soundex phonetic encoding (letter + 3 digits).
// Approach: keep first letter, code consonants, drop repeats/vowels, pad. Time O(L), Space O(L).
#include <bits/stdc++.h>
using namespace std;

// '1'..'6' consonant codes, '0' vowel separator, 'S' transparent (h, w).
char code(char c) {
    switch (c) {
        case 'B': case 'F': case 'P': case 'V': return '1';
        case 'C': case 'G': case 'J': case 'K': case 'Q': case 'S': case 'X': case 'Z': return '2';
        case 'D': case 'T': return '3';
        case 'L': return '4';
        case 'M': case 'N': return '5';
        case 'R': return '6';
        case 'H': case 'W': return 'S';
        default: return '0'; // A E I O U Y
    }
}

string soundex(const string& name) {
    string up;
    for (char c : name) if (isalpha((unsigned char)c)) up += toupper(c);
    if (up.empty()) return "0000";
    string res(1, up[0]);
    char prev = code(up[0]);
    for (size_t i = 1; i < up.size() && res.size() < 4; i++) {
        char c = code(up[i]);
        if (c >= '1' && c <= '6') {
            if (c != prev) res += c;
            prev = c;
        } else if (c == '0') {
            prev = '0';
        } // 'S' (h,w): transparent, prev unchanged
    }
    while (res.size() < 4) res += '0';
    return res.substr(0, 4);
}

int main() {
    cout << soundex("Jackson") << "\n"; // J250
    cout << soundex("Jaxen") << "\n";   // J250
    return 0;
}
