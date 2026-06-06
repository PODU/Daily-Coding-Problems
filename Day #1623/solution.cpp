// Day 1623: Sentence equivalence via synonym set.
// Build symmetric synonym set; compare word-by-word. Time O(W), Space O(S).
#include <bits/stdc++.h>
using namespace std;

bool equivalent(const string& a, const string& b,
                const vector<pair<string,string>>& syns) {
    set<pair<string,string>> pairs;
    for (auto& p : syns) { pairs.insert({p.first, p.second}); pairs.insert({p.second, p.first}); }
    istringstream sa(a), sb(b);
    vector<string> wa, wb; string w;
    while (sa >> w) wa.push_back(w);
    while (sb >> w) wb.push_back(w);
    if (wa.size() != wb.size()) return false;
    for (size_t i = 0; i < wa.size(); i++)
        if (wa[i] != wb[i] && !pairs.count({wa[i], wb[i]})) return false;
    return true;
}

int main() {
    vector<pair<string,string>> syns = {{"big","large"},{"eat","consume"}};
    bool eq = equivalent("He wants to eat food.", "He wants to consume food.", syns);
    cout << (eq ? "The two sentences are equivalent." : "The two sentences are not equivalent.") << endl;
    return 0;
}
