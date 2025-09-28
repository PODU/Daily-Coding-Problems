// Sentence Similarity. Direct (non-transitive) synonym pairs via hash set.
// Time O(P + N) for P pairs and N words. Space O(P).
// Secondary union-find approach (transitive follow-up) commented below.
#include <iostream>
#include <sstream>
#include <unordered_set>
#include <vector>
#include <string>
using namespace std;

vector<string> tokenize(const string& s) {
    vector<string> out;
    istringstream iss(s);
    string w;
    while (iss >> w) {
        while (!w.empty() && ispunct((unsigned char)w.back())) w.pop_back();
        out.push_back(w);
    }
    return out;
}

bool areSimilar(const vector<pair<string,string>>& synonyms,
                const string& s1, const string& s2) {
    unordered_set<string> pairs;
    for (auto& p : synonyms) {
        pairs.insert(p.first + "\0" + p.second);
        pairs.insert(p.second + "\0" + p.first);
    }
    auto w1 = tokenize(s1), w2 = tokenize(s2);
    if (w1.size() != w2.size()) return false;
    for (size_t i = 0; i < w1.size(); i++) {
        if (w1[i] == w2[i]) continue;
        if (pairs.count(w1[i] + "\0" + w2[i])) continue;
        return false;
    }
    return true;
}

// --- Follow-up (transitive): union-find ---
// Build DSU over all synonym tokens; union each pair. Two words match at a
// position if equal OR find(w1) == find(w2). Same O(P*alpha + N) overall.

int main() {
    vector<pair<string,string>> synonyms = {{"big","large"},{"eat","consume"}};
    string s1 = "He wants to eat food.";
    string s2 = "He wants to consume food.";
    cout << (areSimilar(synonyms, s1, s2) ? "equivalent" : "not equivalent") << endl;
    return 0;
}
