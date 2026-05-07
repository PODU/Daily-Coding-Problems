// Day 1483: Shortest closed route from home (0) strictly ascending then
// descending. up[v]: shortest ascending 0->v; down[v]: shortest descending v->0
// (Dijkstra from 0 on reversed descending graph). Answer = min up[v]+down[v].
// Time O((V+E) log V), Space O(V+E).
#include <bits/stdc++.h>
using namespace std;
const long long INF = LLONG_MAX;

void dijkstra(int n, vector<vector<pair<int, long long>>>& adj, int src,
              vector<long long>& dist, vector<int>& pred) {
    dist.assign(n, INF);
    pred.assign(n, -1);
    dist[src] = 0;
    priority_queue<pair<long long, int>, vector<pair<long long, int>>, greater<>> pq;
    pq.push({0, src});
    while (!pq.empty()) {
        long long d = pq.top().first;
        int u = pq.top().second;
        pq.pop();
        if (d > dist[u]) continue;
        for (size_t i = 0; i < adj[u].size(); ++i) {
            int v = adj[u][i].first;
            long long w = adj[u][i].second;
            if (d + w < dist[v]) {
                dist[v] = d + w;
                pred[v] = u;
                pq.push({dist[v], v});
            }
        }
    }
}

int main() {
    int n = 5;
    vector<int> elev = {5, 25, 15, 20, 10};
    vector<array<long long, 3>> edges = {
        {0, 1, 10}, {0, 2, 8}, {0, 3, 15}, {1, 3, 12},
        {2, 4, 10}, {3, 4, 5}, {3, 0, 17}, {4, 0, 10}};

    vector<vector<pair<int, long long>>> up_adj(n), rev_desc(n);
    for (auto& e : edges) {
        int a = e[0], b = e[1]; long long w = e[2];
        if (elev[b] > elev[a]) up_adj[a].push_back({b, w});
        else if (elev[b] < elev[a]) rev_desc[b].push_back({a, w});
    }

    vector<long long> up, down;
    vector<int> up_pred, down_pred;
    dijkstra(n, up_adj, 0, up, up_pred);
    dijkstra(n, rev_desc, 0, down, down_pred);

    long long best = INF; int peak = -1;
    for (int v = 1; v < n; ++v)
        if (up[v] != INF && down[v] != INF && up[v] > 0 && down[v] > 0 && up[v] + down[v] < best) {
            best = up[v] + down[v]; peak = v;
        }

    vector<int> up_path;
    for (int c = peak; c != -1; c = up_pred[c]) up_path.push_back(c);
    reverse(up_path.begin(), up_path.end());
    vector<int> route = up_path;
    for (int c = down_pred[peak]; c != -1; c = down_pred[c]) route.push_back(c);

    cout << "The shortest valid path would be ";
    for (size_t i = 0; i < route.size(); ++i) cout << (i ? " -> " : "") << route[i];
    cout << ", with a distance of " << best << ".\n";
    // The shortest valid path would be 0 -> 2 -> 4 -> 0, with a distance of 28.
}
