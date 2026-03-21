// Celebrity problem: O(N) elimination to one candidate, then O(N) verify.
#include <bits/stdc++.h>
using namespace std;

vector<vector<int>> M;
bool knows(int a, int b) { return M[a][b] == 1; }

int findCelebrity(int n) {
    int cand = 0;
    for (int i = 1; i < n; ++i)
        if (knows(cand, i)) cand = i;
    for (int i = 0; i < n; ++i)
        if (i != cand && (knows(cand, i) || !knows(i, cand))) return -1;
    return cand;
}

int main() {
    M = {{0, 1, 1, 0}, {0, 0, 1, 0}, {0, 0, 0, 0}, {0, 1, 1, 0}};
    cout << findCelebrity(4) << "\n";
    return 0;
}
