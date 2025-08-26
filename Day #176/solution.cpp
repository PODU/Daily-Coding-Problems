// Bijective char mapping check: track s1->s2 map and set of used s2 chars; reject conflicts.
// Time O(n), Space O(unique chars).
#include <iostream>
#include <string>
#include <unordered_map>
#include <unordered_set>
using namespace std;

bool isBijective(const string& s1, const string& s2) {
    if (s1.size() != s2.size()) return false;
    unordered_map<char, char> mapping;
    unordered_set<char> used;
    for (size_t i = 0; i < s1.size(); i++) {
        char a = s1[i], b = s2[i];
        auto it = mapping.find(a);
        if (it != mapping.end()) {
            if (it->second != b) return false;
        } else {
            if (used.count(b)) return false;
            mapping[a] = b;
            used.insert(b);
        }
    }
    return true;
}

int main() {
    cout << (isBijective("abc", "bcd") ? "true" : "false") << endl;
    cout << (isBijective("foo", "bar") ? "true" : "false") << endl;
    return 0;
}
