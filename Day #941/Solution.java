// Day 941: Arbitrage exists iff a cycle has product of rates > 1, i.e. a negative cycle
// under weights w = -log(rate). Bellman-Ford. Time O(V^3), Space O(V).
public class Solution {
    static boolean hasArbitrage(double[][] rate) {
        int n = rate.length;
        double[] dist = new double[n]; // virtual source: all 0
        for (int iter = 0; iter < n - 1; iter++)
            for (int u = 0; u < n; u++)
                for (int v = 0; v < n; v++) {
                    double w = -Math.log(rate[u][v]);
                    if (dist[u] + w < dist[v] - 1e-12) dist[v] = dist[u] + w;
                }
        for (int u = 0; u < n; u++)
            for (int v = 0; v < n; v++) {
                double w = -Math.log(rate[u][v]);
                if (dist[u] + w < dist[v] - 1e-12) return true;
            }
        return false;
    }

    public static void main(String[] args) {
        double[][] rate = {
            {1.0, 2.0, 1.0},
            {0.5, 1.0, 2.0},
            {1.0, 0.5, 1.0}};
        System.out.println(hasArbitrage(rate)); // true (0->1->2->0 = 4 > 1)
    }
}
