// Day 1433: Sentence checker over a character stream.
// Approach: finite-state machine validating one sentence at a time. Time: O(n), Space: O(n) buffer.
#include <bits/stdc++.h>
using namespace std;

// Terminal marks: . ? ! and U+203D (‽, UTF-8 E2 80 BD). We treat the multi-byte
// interrobang as a single terminal token while scanning bytes.
static bool isTerminalAscii(char c) { return c == '.' || c == '?' || c == '!'; }
static bool isSeparator(char c) { return c == ',' || c == ';' || c == ':'; }

bool isValidSentence(const string& s) {
    int n = (int)s.size();
    if (n < 2) return false;
    // Rule 1: capital then lowercase or space.
    if (!isupper((unsigned char)s[0])) return false;
    if (!(islower((unsigned char)s[1]) || s[1] == ' ')) return false;

    bool prevWasLetter = isalpha((unsigned char)s[0]);
    for (int i = 1; i < n; ++i) {
        char c = s[i];
        bool terminal = false;
        int adv = 0;
        // detect interrobang ‽ (UTF-8 bytes E2 80 BD) as a single terminal token
        if ((unsigned char)c == 0xE2 && i + 2 < n &&
            (unsigned char)s[i + 1] == 0x80 && (unsigned char)s[i + 2] == 0xBD) {
            terminal = true; adv = 2;
        } else if (isTerminalAscii(c)) {
            terminal = true;
        }

        if (terminal) {
            // Rule 4: terminal must immediately follow a word (letter) and end the sentence.
            if (!prevWasLetter) return false;
            return i + adv == n - 1;
        }
        if (c == ' ') {
            // Rule 3: single space between words -> no double space, must be between letters.
            if (s[i - 1] == ' ') return false;
            prevWasLetter = false;
            continue;
        }
        if (islower((unsigned char)c)) { prevWasLetter = true; continue; }
        if (isSeparator(c)) { prevWasLetter = false; continue; }
        return false; // any other char invalid in body
    }
    return false; // never hit a terminating mark
}

int main() {
    vector<string> tests = {
        "The quick brown fox.",
        "Hello world!",
        "lowercase start.",       // invalid: no capital
        "No terminal mark",       // invalid: no ending mark
        "Two  spaces here."       // invalid: double space
    };
    for (auto& t : tests) {
        if (isValidSentence(t)) cout << t << endl;
    }
    return 0;
}
