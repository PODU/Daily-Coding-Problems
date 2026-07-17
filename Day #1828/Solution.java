// Arbitrage = negative cycle in graph with weights -log(rate). Bellman-Ford.
// O(V^3) for a dense rate table.
public class Solution {
    static boolean hasArbitrage(double[][] rate){
        int n = rate.length;
        double[][] w = new double[n][n];
        for(int i = 0; i < n; i++)
            for(int j = 0; j < n; j++)
                w[i][j] = -Math.log(rate[i][j]);
        double[] dist = new double[n]; // virtual source, all 0
        for(int it = 0; it < n - 1; it++)
            for(int u = 0; u < n; u++)
                for(int v = 0; v < n; v++)
                    if(dist[u] + w[u][v] < dist[v]) dist[v] = dist[u] + w[u][v];
        for(int u = 0; u < n; u++)
            for(int v = 0; v < n; v++)
                if(dist[u] + w[u][v] < dist[v] - 1e-12) return true;
        return false;
    }
    public static void main(String[] args){
        double[][] rate = {
            {1.0, 0.8, 0.5},
            {1.3, 1.0, 1.9},
            {1.9, 0.5, 1.0}
        };
        System.out.println(hasArbitrage(rate) ? "true" : "false"); // true
    }
}
