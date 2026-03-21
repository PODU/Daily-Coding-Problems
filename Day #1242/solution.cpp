// Word ladder: BFS over words differing by one letter. Time O(N*L*26).
#include <bits/stdc++.h>
using namespace std;

vector<string> wordLadder(string start, string end, set<string> dict) {
    if (!dict.count(end)) return {};
    queue<vector<string>> q;
    q.push({start});
    set<string> seen{start};
    while (!q.empty()) {
        auto path = q.front(); q.pop();
        string word = path.back();
        if (word == end) return path;
        for (size_t i = 0; i < word.size(); ++i) {
            string nxt = word;
            for (char c = 'a'; c <= 'z'; ++c) {
                nxt[i] = c;
                if (dict.count(nxt) && !seen.count(nxt)) {
                    seen.insert(nxt);
                    auto np = path; np.push_back(nxt);
                    q.push(np);
                }
            }
        }
    }
    return {};
}

void printPath(const vector<string>& p) {
    if (p.empty()) { cout << "null\n"; return; }
    cout << "[";
    for (size_t i = 0; i < p.size(); ++i) {
        cout << "'" << p[i] << "'";
        if (i + 1 < p.size()) cout << ", ";
    }
    cout << "]\n";
}

int main() {
    printPath(wordLadder("dog", "cat", {"dot", "dop", "dat", "cat"}));
    printPath(wordLadder("dog", "cat", {"dot", "tod", "dat", "dar"}));
    return 0;
}
