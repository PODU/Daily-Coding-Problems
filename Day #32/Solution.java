// Currency arbitrage: weights = -log(rate), Bellman-Ford detects negative cycle. O(V*E).
public class Solution {
    static boolean hasArbitrage(double[][] rates) {
        int n = rates.length;
        double[][] w = new double[n][n];
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                w[i][j] = -Math.log(rates[i][j]);
        double[] dist = new double[n]; // virtual super-source reaching all nodes at 0
        for (int it = 0; it < n - 1; it++)
            for (int u = 0; u < n; u++)
                for (int v = 0; v < n; v++)
                    if (dist[u] + w[u][v] < dist[v] - 1e-12) dist[v] = dist[u] + w[u][v];
        for (int u = 0; u < n; u++)
            for (int v = 0; v < n; v++)
                if (dist[u] + w[u][v] < dist[v] - 1e-12) return true;
        return false;
    }

    public static void main(String[] args) {
        double[][] r1 = {{1.0, 0.7, 0.5}, {1.4, 1.0, 0.7}, {2.1, 1.4, 1.0}};
        double[][] r2 = {{1.0, 2.0, 4.0}, {0.5, 1.0, 2.0}, {0.25, 0.5, 1.0}};
        System.out.println(hasArbitrage(r1));
        System.out.println(hasArbitrage(r2));
    }
}
