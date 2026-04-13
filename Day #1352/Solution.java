// Arbitrage detection: edge weight = -log(rate); negative cycle => arbitrage. Bellman-Ford.
// Time: O(V*E) = O(V^3), Space: O(V).
public class Solution {
    static boolean hasArbitrage(double[][] rates) {
        int n = rates.length;
        double[] dist = new double[n]; // all 0: detect any reachable negative cycle
        for (int k = 0; k < n - 1; k++)
            for (int u = 0; u < n; u++)
                for (int v = 0; v < n; v++) {
                    double w = -Math.log(rates[u][v]);
                    if (dist[u] + w < dist[v] - 1e-12) dist[v] = dist[u] + w;
                }
        for (int u = 0; u < n; u++)
            for (int v = 0; v < n; v++) {
                double w = -Math.log(rates[u][v]);
                if (dist[u] + w < dist[v] - 1e-12) return true;
            }
        return false;
    }

    public static void main(String[] args) {
        double[][] rates = {
            {1.0, 2.0, 1.0},
            {0.5, 1.0, 4.0},
            {1.0, 0.25, 1.0}
        };
        System.out.println("Arbitrage exists: " + hasArbitrage(rates));
    }
}
