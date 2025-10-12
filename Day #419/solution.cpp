// Day 419: Min steps to reduce N to 1 (decrement, or jump to larger factor).
// BFS over values 1..N. Time: O(N*sqrt(N)), Space: O(N).
#include <bits/stdc++.h>
using namespace std;

int minSteps(int N) {
    if (N <= 1) return 0;
    vector<int> dist(N + 1, -1);
    queue<int> q;
    dist[N] = 0;
    q.push(N);
    while (!q.empty()) {
        int v = q.front(); q.pop();
        if (v == 1) return dist[1];
        // neighbor: v - 1
        if (v - 1 >= 1 && dist[v - 1] == -1) {
            dist[v - 1] = dist[v] + 1;
            q.push(v - 1);
        }
        // neighbor: larger factor v/a for each divisor a >= 2, a <= sqrt(v)
        for (int a = 2; (long long)a * a <= v; ++a) {
            if (v % a == 0) {
                int larger = v / a; // larger of (a, v/a)
                if (dist[larger] == -1) {
                    dist[larger] = dist[v] + 1;
                    q.push(larger);
                }
            }
        }
    }
    return dist[1];
}

int main() {
    int N = 100;
    cout << minSteps(N) << "  (route: 100 -> 10 -> 9 -> 3 -> 2 -> 1)" << endl;
    return 0;
}
