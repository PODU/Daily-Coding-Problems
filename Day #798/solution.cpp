// Day 798: Sentence equivalence via synonym pairs (NOT transitive).
// Store each unordered pair in a set; words match if equal or directly paired.
// Time O(W) per comparison, Space O(P).
#include <bits/stdc++.h>
using namespace std;

struct Equiv {
    set<pair<string, string>> syn;
    void add(const string& a, const string& b) {
        syn.insert({min(a, b), max(a, b)});
    }
    bool same(const string& a, const string& b) {
        return a == b || syn.count({min(a, b), max(a, b)});
    }
    bool equivalent(vector<string>& s1, vector<string>& s2) {
        if (s1.size() != s2.size()) return false;
        for (size_t i = 0; i < s1.size(); i++)
            if (!same(s1[i], s2[i])) return false;
        return true;
    }
};

int main() {
    Equiv e;
    e.add("big", "large");
    e.add("eat", "consume");
    vector<string> a = {"He", "wants", "to", "eat", "food."};
    vector<string> b = {"He", "wants", "to", "consume", "food."};
    cout << (e.equivalent(a, b) ? "True (equivalent)" : "False") << "\n";
    return 0;
}
