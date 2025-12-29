// Word break via DP with backpointers: dp[i] reachable, prev[i] start of last word. O(n^2) time, O(n) space.
#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
using namespace std;

bool wordBreak(const string& s, const unordered_set<string>& dict, vector<string>& out) {
    int n = s.size();
    vector<int> prev(n + 1, -2); // -2 = unreachable
    prev[0] = -1;
    for (int i = 1; i <= n; i++) {
        for (int j = i - 1; j >= 0; j--) { // prefer shortest last word
            if (prev[j] != -2 && dict.count(s.substr(j, i - j))) {
                prev[i] = j;
                break;
            }
        }
    }
    if (prev[n] == -2) return false;
    for (int i = n; i > 0; i = prev[i])
        out.insert(out.begin(), s.substr(prev[i], i - prev[i]));
    return true;
}

void run(const string& s, const unordered_set<string>& dict) {
    vector<string> r;
    if (!wordBreak(s, dict, r)) { cout << "null\n"; return; }
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) {
        cout << r[i];
        if (i + 1 < r.size()) cout << ", ";
    }
    cout << "]\n";
}

int main() {
    run("thequickbrownfox", {"quick", "brown", "the", "fox"});
    run("bedbathandbeyond", {"bed", "bath", "bedbath", "and", "beyond"});
    return 0;
}
