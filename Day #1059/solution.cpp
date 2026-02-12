// Word sense disambiguation: for each ambiguous word pick the meaning whose words
// overlap most with the sentence context (other words). Tie-break -> first meaning.
// Time: O(W * M * L), Space: O(L) for the context set. (W words, M meanings, L meaning length)
#include <bits/stdc++.h>
using namespace std;

static string lower(string s) {
    for (char& c : s) c = tolower((unsigned char)c);
    return s;
}

static vector<string> tokenize(const string& s) {
    vector<string> out;
    string cur;
    for (char c : s) {
        if (isalnum((unsigned char)c)) cur += tolower((unsigned char)c);
        else if (!cur.empty()) { out.push_back(cur); cur.clear(); }
    }
    if (!cur.empty()) out.push_back(cur);
    return out;
}

int main() {
    map<string, vector<string>> dict = {
        {"bank", {"financial institution where people deposit money",
                  "land beside a river or lake"}}
    };
    string sentence = "I went to get money from the bank";
    vector<string> words = tokenize(sentence);

    for (size_t i = 0; i < words.size(); ++i) {
        const string& w = words[i];
        auto it = dict.find(w);
        if (it == dict.end() || it->second.size() <= 1) continue;

        // context = all other words of the sentence
        set<string> context;
        for (size_t j = 0; j < words.size(); ++j)
            if (j != i) context.insert(words[j]);

        int best = -1, bestScore = -1;
        for (size_t m = 0; m < it->second.size(); ++m) {
            int score = 0;
            for (const string& t : tokenize(it->second[m]))
                if (context.count(t)) ++score;
            if (score > bestScore) { bestScore = score; best = (int)m; }
        }
        cout << w << ": " << it->second[best] << "\n";
    }
    return 0;
}
