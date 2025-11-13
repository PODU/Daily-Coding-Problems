// Word sense disambiguation via simplified Lesk.
// Score each candidate gloss by content-word overlap with the sentence
// context (other words + their glosses); pick the highest.
// Time O(words * meanings * glossLen). Space O(vocab).
#include <iostream>
#include <map>
#include <set>
#include <vector>
#include <string>
#include <sstream>
#include <algorithm>
using namespace std;

set<string> STOP = {
    "i","to","the","a","an","of","by","and","or","where","people",
    "that","is","are","in","on","at","with","went","sat","this"
};

set<string> tokens(const string& text) {
    set<string> out;
    stringstream ss(text);
    string w;
    while (ss >> w) {
        string lw;
        for (char c : w) if (isalpha((unsigned char)c)) lw += tolower(c);
        if (!lw.empty() && !STOP.count(lw)) out.insert(lw);
    }
    return out;
}

int main() {
    map<string, vector<string>> meanings = {
        {"bank", {"a financial institution where people deposit and withdraw money",
                  "the land alongside a river or lake"}},
        {"money", {"currency that people deposit and withdraw"}},
        {"river", {"a large natural stream of water"}}
    };

    vector<string> sentences = {
        "I went to the bank to deposit money",
        "I sat by the bank of the river"
    };

    for (const auto& sentence : sentences) {
        stringstream ss(sentence);
        string word;
        vector<string> words;
        while (ss >> word) words.push_back(word);

        for (const string& w : words) {
            string lw;
            for (char c : w) lw += tolower(c);
            auto it = meanings.find(lw);
            if (it == meanings.end() || it->second.size() <= 1) continue;

            // build context: other words + their glosses
            set<string> context;
            for (const string& other : words) {
                string ol;
                for (char c : other) ol += tolower(c);
                if (ol == lw) continue;
                for (const auto& t : tokens(other)) context.insert(t);
                auto mit = meanings.find(ol);
                if (mit != meanings.end()) {
                    for (const string& g : mit->second)
                        for (const auto& t : tokens(g)) context.insert(t);
                }
            }

            int best = -1; string bestGloss;
            for (const string& gloss : it->second) {
                set<string> g = tokens(gloss);
                int overlap = 0;
                for (const auto& t : g) if (context.count(t)) overlap++;
                if (overlap > best) { best = overlap; bestGloss = gloss; }
            }
            cout << lw << ": " << bestGloss << "\n";
        }
    }
    return 0;
}
