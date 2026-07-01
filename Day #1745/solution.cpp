// Celebrity finder: two-phase candidate elimination then verify. O(N) knows() calls, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int N = 4;
int knowsMat[4][4] = {
    {0, 1, 1, 0},
    {0, 0, 1, 0},
    {0, 0, 0, 0},
    {0, 1, 1, 0}
};

bool knows(int a, int b) { return knowsMat[a][b] == 1; }

int findCelebrity(int n) {
    int cand = 0;
    for (int i = 1; i < n; i++)
        if (knows(cand, i)) cand = i;
    for (int i = 0; i < n; i++) {
        if (i == cand) continue;
        if (knows(cand, i) || !knows(i, cand)) return -1;
    }
    return cand;
}

int main() {
    cout << findCelebrity(N) << "\n";
    return 0;
}
