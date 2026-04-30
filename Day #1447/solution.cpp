// Day 1447: Does a secret code (6 distinct digits) exist consistent with all
// (guess, score) pairs? Brute force all 6-permutations of 0-9. Time O(P*G*6).
#include <bits/stdc++.h>
using namespace std;

int scoreMatch(const array<int,6>& code, const string& guess) {
    int s = 0;
    for (int i = 0; i < 6; i++) if (code[i] == guess[i] - '0') s++;
    return s;
}

bool consistent(const vector<pair<string,int>>& guesses) {
    array<int,6> code;
    bool used[10] = {false};
    // DFS to build every 6-length permutation of distinct digits (no leading 0)
    function<bool(int)> dfs = [&](int pos) -> bool {
        if (pos == 6) {
            for (size_t k = 0; k < guesses.size(); k++)
                if (scoreMatch(code, guesses[k].first) != guesses[k].second) return false;
            return true;
        }
        for (int d = 0; d < 10; d++) {
            if (used[d]) continue;
            if (pos == 0 && d == 0) continue; // six-digit number, no leading zero
            used[d] = true; code[pos] = d;
            if (dfs(pos + 1)) { used[d] = false; return true; }
            used[d] = false;
        }
        return false;
    };
    return dfs(0);
}

int main() {
    vector<pair<string,int>> e1 = {{"175286",2},{"293416",3},{"654321",0}};
    vector<pair<string,int>> e2 = {{"123456",4},{"345678",4},{"567890",4}};
    cout << (consistent(e1) ? "True" : "False") << "\n";  // True
    cout << (consistent(e2) ? "True" : "False") << "\n";  // False
    return 0;
}
