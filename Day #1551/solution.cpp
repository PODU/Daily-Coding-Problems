// Sentence validator over a char stream: split on terminal marks, validate each candidate.
// isValid checks capital start, lowercase/separators body, single spaces, terminal end. Time O(n).
// Demo uses ASCII terminals . ? ! (interrobang treated as terminal if present).
#include <iostream>
#include <string>
#include <vector>

bool isTerminal(char c) { return c == '.' || c == '?' || c == '!'; }
bool isSeparator(char c) { return c == ',' || c == ';' || c == ':'; }
bool isLower(char c) { return c >= 'a' && c <= 'z'; }
bool isUpper(char c) { return c >= 'A' && c <= 'Z'; }

bool isValid(const std::string& s) {
    size_t n = s.size();
    if (n < 2) return false;
    if (!isUpper(s[0])) return false;
    if (!(isLower(s[1]) || s[1] == ' ')) return false;
    if (!isTerminal(s[n - 1])) return false;
    if (!(isLower(s[n - 2]) || isUpper(s[n - 2]))) return false;
    for (size_t i = 1; i + 1 < n; ++i) {
        char c = s[i];
        if (isLower(c) || isSeparator(c)) continue;
        if (c == ' ') {
            if (s[i - 1] == ' ') return false;
            continue;
        }
        return false;
    }
    return true;
}

std::string trim(const std::string& s) {
    size_t a = 0, b = s.size();
    while (a < b && s[a] == ' ') ++a;
    while (b > a && s[b - 1] == ' ') --b;
    return s.substr(a, b - a);
}

int main() {
    std::string stream = "Hello world. this is bad.";
    std::string buf;
    for (char ch : stream) {
        buf.push_back(ch);
        if (isTerminal(ch)) {
            std::string candidate = trim(buf);
            if (isValid(candidate)) std::cout << candidate << "\n";
            buf.clear();
        }
    }
    return 0;
}
