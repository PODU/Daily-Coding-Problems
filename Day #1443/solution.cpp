// Day 1443: Word sense disambiguation (simplified Lesk algorithm).
// For each ambiguous word pick the meaning whose words overlap most with the
// rest of the sentence's words. Time O(W * M * L), Space O(vocab).
#include <bits/stdc++.h>
using namespace std;

static vector<string> tokenize(const string& s) {
    vector<string> out; string cur;
    for (char c : s) {
        if (isalnum((unsigned char)c)) cur += tolower(c);
        else if (!cur.empty()) { out.push_back(cur); cur.clear(); }
    }
    if (!cur.empty()) out.push_back(cur);
    return out;
}

// Returns chosen meaning for each ambiguous word in the sentence.
map<string,string> disambiguate(const map<string,vector<string>>& meanings,
                                const string& sentence) {
    vector<string> words = tokenize(sentence);
    set<string> context(words.begin(), words.end());
    map<string,string> result;
    for (const string& w : words) {
        auto it = meanings.find(w);
        if (it == meanings.end() || it->second.size() <= 1) continue;
        int best = -1; string bestMeaning;
        for (const string& m : it->second) {
            set<string> mt;
            for (auto& t : tokenize(m)) mt.insert(t);
            int score = 0;
            for (auto& t : mt) if (t != w && context.count(t)) score++;
            if (score > best) { best = score; bestMeaning = m; }
        }
        result[w] = bestMeaning;
    }
    return result;
}

int main() {
    map<string,vector<string>> meanings = {
        {"bank", {"financial institution where people deposit money",
                  "sloping land beside a river or lake"}}
    };
    string sentence = "I went to the bank to deposit money";
    auto res = disambiguate(meanings, sentence);
    cout << "bank: " << res["bank"] << "\n";
    return 0;
}
