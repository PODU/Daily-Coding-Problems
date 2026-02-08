// Reverse words but keep delimiters fixed in place: extract words, reverse the list,
// rebuild keeping delimiter chars at original positions. O(n) time, O(n) space.
#include <iostream>
#include <string>
#include <vector>
#include <unordered_set>

bool isDelim(char c, const std::unordered_set<char>& d) { return d.count(c) > 0; }

std::string reverseWords(const std::string& s, const std::unordered_set<char>& delims) {
    std::vector<std::string> words;
    std::string cur;
    for (char c : s) {
        if (isDelim(c, delims)) {
            if (!cur.empty()) { words.push_back(cur); cur.clear(); }
        } else {
            cur += c;
        }
    }
    if (!cur.empty()) words.push_back(cur);

    std::vector<std::string> rev(words.rbegin(), words.rend());
    std::string out;
    size_t wi = 0;
    size_t i = 0;
    while (i < s.size()) {
        if (isDelim(s[i], delims)) {
            out += s[i];
            i++;
        } else {
            out += rev[wi++];
            while (i < s.size() && !isDelim(s[i], delims)) i++;
        }
    }
    return out;
}

int main() {
    std::unordered_set<char> delims = {'/', ':'};
    std::vector<std::string> tests = {"hello/world:here", "hello/world:here/", "hello//world:here"};
    for (auto& t : tests)
        std::cout << t << " -> " << reverseWords(t, delims) << "\n";
    return 0;
}
