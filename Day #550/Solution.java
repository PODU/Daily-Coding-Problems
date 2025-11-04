// Arbitrage detection: weight=-log(rate), Bellman-Ford finds a negative-weight cycle => arbitrage. O(V^3) (V*E edges, V-1 passes).
public class Solution {
    static boolean hasArbitrage(double[][] rates) {
        int n = rates.length;
        double[] dist = new double[n]; // virtual source: all start at 0
        for (int it = 0; it < n - 1; it++) {
            for (int u = 0; u < n; u++)
                for (int v = 0; v < n; v++) {
                    double w = -Math.log(rates[u][v]);
                    if (dist[u] + w < dist[v] - 1e-12) dist[v] = dist[u] + w;
                }
        }
        for (int u = 0; u < n; u++)
            for (int v = 0; v < n; v++) {
                double w = -Math.log(rates[u][v]);
                if (dist[u] + w < dist[v] - 1e-12) return true;
            }
        return false;
    }

    public static void main(String[] args) {
        double[][] arb = {{1, 0.5, 0.2}, {2, 1, 0.5}, {5, 2, 1}};
        double[][] consistent = {{1, 2, 4}, {0.5, 1, 2}, {0.25, 0.5, 1}};
        System.out.println(hasArbitrage(arb) ? "true" : "false");
        System.out.println(hasArbitrage(consistent) ? "true" : "false");
    }
}
