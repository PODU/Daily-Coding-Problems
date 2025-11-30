// Day 675: Sentence equivalence under (non-transitive) synonym pairs. For each position,
// words must be equal or a known synonym pair. Time O(W) with a hashed pair set.
#include <bits/stdc++.h>
using namespace std;

vector<string> tokens(const string& s) {
    vector<string> out; string cur;
    for (char c : s) {
        if (isalnum((unsigned char)c)) cur += (char)tolower(c);
        else if (!cur.empty()) { out.push_back(cur); cur.clear(); }
    }
    if (!cur.empty()) out.push_back(cur);
    return out;
}

bool equivalent(const vector<pair<string,string>>& syn, const string& s1, const string& s2) {
    set<pair<string,string>> S;
    for (auto& p : syn) { S.insert({p.first, p.second}); S.insert({p.second, p.first}); }
    auto a = tokens(s1), b = tokens(s2);
    if (a.size() != b.size()) return false;
    for (size_t i = 0; i < a.size(); i++)
        if (a[i] != b[i] && !S.count({a[i], b[i]})) return false;
    return true;
}

int main() {
    vector<pair<string,string>> syn = {{"big", "large"}, {"eat", "consume"}};
    cout << (equivalent(syn, "He wants to eat food.", "He wants to consume food.") ? "True" : "False") << "\n";
    return 0; // True
}
