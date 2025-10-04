// Day 371: Solve a system of addition-only equations over variables/constants.
// Build a linear system A x = b and run Gauss-Jordan elimination; unique integer
// solution -> mapping, otherwise null. Time O(eqs * vars^2).
import java.util.*;

public class Solution {
    static boolean isNumber(String t) {
        if (t.isEmpty()) return false;
        int i = t.charAt(0) == '-' ? 1 : 0;
        if (i == t.length()) return false;
        for (; i < t.length(); i++) if (!Character.isDigit(t.charAt(i))) return false;
        return true;
    }

    static String solve(String text) {
        List<Map<String, Double>> coeffsList = new ArrayList<>();
        List<Double> bs = new ArrayList<>();
        TreeSet<String> varset = new TreeSet<>();
        for (String raw : text.split("\n")) {
            String line = raw.trim();
            if (line.isEmpty()) continue;
            String[] sides = line.split("=");
            Map<String, Double> coeffs = new HashMap<>();
            double b = 0;
            for (String tok : sides[0].split("\\+")) {
                String t = tok.trim();
                if (isNumber(t)) b -= Long.parseLong(t);
                else { coeffs.merge(t, 1.0, Double::sum); varset.add(t); }
            }
            for (String tok : sides[1].split("\\+")) {
                String t = tok.trim();
                if (isNumber(t)) b += Long.parseLong(t);
                else { coeffs.merge(t, -1.0, Double::sum); varset.add(t); }
            }
            coeffsList.add(coeffs);
            bs.add(b);
        }

        List<String> vars = new ArrayList<>(varset);
        Map<String, Integer> idx = new HashMap<>();
        for (int i = 0; i < vars.size(); i++) idx.put(vars.get(i), i);
        int n = vars.size();
        double[][] aug = new double[coeffsList.size()][n + 1];
        for (int r = 0; r < coeffsList.size(); r++) {
            for (Map.Entry<String, Double> e : coeffsList.get(r).entrySet())
                aug[r][idx.get(e.getKey())] += e.getValue();
            aug[r][n] = bs.get(r);
        }

        int m = aug.length, pr = 0;
        List<Integer> pivotCols = new ArrayList<>();
        for (int col = 0; col < n; col++) {
            int sel = -1;
            for (int r = pr; r < m; r++) if (Math.abs(aug[r][col]) > 1e-9) { sel = r; break; }
            if (sel == -1) continue;
            double[] tmp = aug[pr]; aug[pr] = aug[sel]; aug[sel] = tmp;
            double pv = aug[pr][col];
            for (int c = 0; c <= n; c++) aug[pr][c] /= pv;
            for (int r = 0; r < m; r++) if (r != pr && Math.abs(aug[r][col]) > 1e-9) {
                double f = aug[r][col];
                for (int c = 0; c <= n; c++) aug[r][c] -= f * aug[pr][c];
            }
            pivotCols.add(col);
            pr++;
        }
        for (int r = 0; r < m; r++) {
            boolean allZero = true;
            for (int c = 0; c < n; c++) if (Math.abs(aug[r][c]) > 1e-9) allZero = false;
            if (allZero && Math.abs(aug[r][n]) > 1e-9) return "null";
        }
        if (pivotCols.size() < n) return "null";

        TreeMap<String, Long> sol = new TreeMap<>();
        for (int i = 0; i < pivotCols.size(); i++)
            sol.put(vars.get(pivotCols.get(i)), Math.round(aug[i][n]));
        StringBuilder sb = new StringBuilder("{\n");
        boolean first = true;
        for (Map.Entry<String, Long> e : sol.entrySet()) {
            if (!first) sb.append(",\n");
            sb.append("  ").append(e.getKey()).append(": ").append(e.getValue());
            first = false;
        }
        sb.append("\n}");
        return sb.toString();
    }

    public static void main(String[] args) {
        String text = "y = x + 1\n5 = x + 3\n10 = z + y + 2";
        System.out.println(solve(text));
    }
}
