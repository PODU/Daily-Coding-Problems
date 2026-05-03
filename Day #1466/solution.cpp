// Sentence similarity (non-transitive). Store synonym pairs both directions in a set; compare word by word.
// Time O(words + pairs), Space O(pairs).
#include <bits/stdc++.h>
using namespace std;

bool areSimilar(const vector<string>& s1, const vector<string>& s2,
                const vector<pair<string, string>>& pairs) {
    if (s1.size() != s2.size()) return false;
    set<pair<string, string>> syn;
    for (auto& p : pairs) {
        syn.insert({p.first, p.second});
        syn.insert({p.second, p.first});
    }
    for (size_t i = 0; i < s1.size(); ++i) {
        if (s1[i] == s2[i]) continue;
        if (syn.count({s1[i], s2[i]})) continue;
        return false;
    }
    return true;
}

vector<string> tokenize(const string& s) {
    vector<string> out;
    string cur;
    for (char c : s) {
        if (c == ' ') { if (!cur.empty()) { out.push_back(cur); cur.clear(); } }
        else if (c == '.') continue; // strip period
        else cur += c;
    }
    if (!cur.empty()) out.push_back(cur);
    return out;
}

int main() {
    vector<pair<string, string>> syn = {{"big", "large"}, {"eat", "consume"}};
    auto a = tokenize("He wants to eat food.");
    auto b = tokenize("He wants to consume food.");
    cout << (areSimilar(a, b, syn) ? "True" : "False") << "\n";
    return 0;
}
