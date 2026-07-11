// Word wrap greedily: pack max words per line <= k, return empty (null) if any word > k. O(total length) time.
#include <iostream>
#include <vector>
#include <sstream>
#include <string>

// Returns {ok, lines}; ok=false means null.
std::pair<bool, std::vector<std::string>> wordWrap(const std::string& s, int k) {
    std::vector<std::string> words, lines;
    std::istringstream iss(s);
    std::string w;
    while (iss >> w) {
        if ((int)w.size() > k) return {false, {}};
        words.push_back(w);
    }
    std::string cur;
    for (auto& word : words) {
        if (cur.empty()) cur = word;
        else if ((int)(cur.size() + 1 + word.size()) <= k) cur += " " + word;
        else { lines.push_back(cur); cur = word; }
    }
    if (!cur.empty()) lines.push_back(cur);
    return {true, lines};
}

int main() {
    auto res = wordWrap("the quick brown fox jumps over the lazy dog", 10);
    if (!res.first) { std::cout << "null\n"; return 0; }
    std::cout << "[";
    for (size_t i = 0; i < res.second.size(); ++i)
        std::cout << "\"" << res.second[i] << "\"" << (i + 1 < res.second.size() ? ", " : "");
    std::cout << "]\n";
    return 0;
}
