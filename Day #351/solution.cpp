// Simplified Lesk: score each gloss by word overlap with sentence context; pick max (ties->first).
// Time O(words * meanings * glossLen), Space O(vocab).
#include <bits/stdc++.h>
using namespace std;

static vector<string> tokenize(const string& s) {
    vector<string> out; string cur;
    for (char c : s) {
        char lc = (char)tolower((unsigned char)c);
        if (isspace((unsigned char)c)) { if (!cur.empty()) { out.push_back(cur); cur.clear(); } }
        else cur.push_back(lc);
    }
    if (!cur.empty()) out.push_back(cur);
    return out;
}

int main() {
    map<string, vector<string>> meanings = {
        {"bank", {"place where people deposit and withdraw money",
                  "sloping land beside a river or lake of water"}},
        {"money", {"currency cash that people deposit"}},
        {"river", {"large natural stream of water"}},
    };
    string sentence = "I went to get money from the bank";
    vector<string> tokens = tokenize(sentence);

    for (size_t i = 0; i < tokens.size(); ++i) {
        auto it = meanings.find(tokens[i]);
        if (it == meanings.end() || it->second.size() < 2) continue; // not ambiguous
        set<string> context;
        for (size_t j = 0; j < tokens.size(); ++j) if (j != i) context.insert(tokens[j]);
        int bestIdx = 0, bestScore = -1;
        for (size_t idx = 0; idx < it->second.size(); ++idx) {
            set<string> gw;
            for (auto& w : tokenize(it->second[idx])) gw.insert(w);
            int score = 0;
            for (auto& w : gw) if (context.count(w)) score++;
            if (score > bestScore) { bestScore = score; bestIdx = (int)idx; }
        }
        cout << tokens[i] << ": " << it->second[bestIdx] << "\n";
    }
    return 0;
}
