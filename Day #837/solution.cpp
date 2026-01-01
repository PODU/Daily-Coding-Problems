// Day 837: Sentence checker over a character stream.
// Accumulate chars until a terminal mark, then validate the buffered sentence; print if valid.
// O(N) over the stream; O(L) per sentence. (UTF-8 aware for the '‽' terminal, U+203D = E2 80 BD.)
#include <bits/stdc++.h>
using namespace std;

static bool isLower(char c) { return c >= 'a' && c <= 'z'; }
static bool isUpper(char c) { return c >= 'A' && c <= 'Z'; }
static bool isSep(char c) { return c == ',' || c == ';' || c == ':'; }

// Validate: ^[A-Z][a-z]*[,;:]?( [a-z]+[,;:]?)*[term]$  where the trailing terminal is already removed.
bool validBody(const string& body) {
    int n = (int)body.size();
    if (n == 0 || !isUpper(body[0])) return false;
    int i = 1;
    while (i < n && isLower(body[i])) i++;
    if (i < n && isSep(body[i])) i++;
    // subsequent words: " " + [a-z]+ + optional sep
    while (i < n) {
        if (body[i] != ' ') return false;
        i++;
        int start = i;
        while (i < n && isLower(body[i])) i++;
        if (i == start) return false; // need at least one lowercase letter
        if (i < n && isSep(body[i])) i++;
    }
    return true;
}

vector<string> checkStream(const string& stream) {
    vector<string> results;
    string buf;
    size_t i = 0;
    while (i < stream.size()) {
        // detect terminal: single-byte . ? ! or 3-byte ‽ (E2 80 BD)
        unsigned char c = (unsigned char)stream[i];
        bool terminal = false;
        size_t adv = 1;
        if (c == '.' || c == '?' || c == '!') {
            terminal = true;
        } else if (c == 0xE2 && i + 2 < stream.size() &&
                   (unsigned char)stream[i + 1] == 0x80 &&
                   (unsigned char)stream[i + 2] == 0xBD) {
            terminal = true;
            adv = 3;
        }
        if (terminal) {
            string sentence = buf;
            // trim single leading space
            size_t p = 0;
            while (p < sentence.size() && sentence[p] == ' ') p++;
            string body = sentence.substr(p);
            string full = body + stream.substr(i, adv);
            if (validBody(body)) results.push_back(full);
            buf.clear();
        } else {
            buf += stream.substr(i, adv);
        }
        i += adv;
    }
    return results;
}

int main() {
    string stream = "Hello, world. this is wrong. The cat sat.";
    for (auto& s : checkStream(stream)) cout << s << "\n";
    return 0;
}
