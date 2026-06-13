// Simplified Lesk WSD: score each candidate meaning by overlap with the union of other
// in-dict context words and their meaning texts. O(W*M*L) time, O(V) space.
#include <bits/stdc++.h>
using namespace std;

vector<string> words(const string &s) {
    string t = s;
    transform(t.begin(), t.end(), t.begin(), ::tolower);
    istringstream iss(t);
    vector<string> out; string w;
    while (iss >> w) out.push_back(w);
    return out;
}

int main() {
    vector<pair<string, vector<string>>> meanings = {
        {"bank", {"place where people deposit and withdraw money", "land beside a river or lake"}},
        {"money", {"currency coins and cash used to buy things"}},
        {"river", {"a large natural stream of flowing water"}},
    };
    auto lookup = [&](const string &w) -> vector<string>* {
        for (auto &p : meanings) if (p.first == w) return &p.second;
        return nullptr;
    };
    string sentence = "I went to get money from the bank";
    vector<string> toks = words(sentence);
    for (auto &w : toks) {
        auto *cands = lookup(w);
        if (cands && cands->size() > 1) {
            set<string> ctx;
            for (auto &o : toks) {
                if (o != w) {
                    auto *om = lookup(o);
                    if (om) {
                        ctx.insert(o);
                        for (auto &m : *om) for (auto &x : words(m)) ctx.insert(x);
                    }
                }
            }
            string best = (*cands)[0]; int bestScore = -1;
            for (auto &cand : *cands) {
                int score = 0;
                for (auto &t : words(cand)) if (ctx.count(t)) score++;
                if (score > bestScore) { bestScore = score; best = cand; }
            }
            cout << w << ": " << best << "\n";
        }
    }
    return 0;
}
