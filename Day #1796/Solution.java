// Uniform random in [0,n-1] excluding values in l: precompute sorted allowed array, pick random index. O(n) build, O(allowed) space.
import java.util.*;

public class Solution {
    static class RandomPicker {
        final List<Integer> allowed = new ArrayList<>();
        final Random rng = new Random(12345);
        RandomPicker(int n, int[] l) {
            Set<Integer> ex = new HashSet<>();
            for (int x : l) ex.add(x);
            for (int i = 0; i < n; i++) if (!ex.contains(i)) allowed.add(i);
        }
        int pick() { return allowed.get(rng.nextInt(allowed.size())); }
    }

    public static void main(String[] args) {
        RandomPicker rp = new RandomPicker(10, new int[]{2, 3, 5, 7});
        Set<Integer> aset = new HashSet<>(rp.allowed);
        System.out.println("allowed=" + rp.allowed);
        boolean ok = true;
        for (int i = 0; i < 1000; i++) if (!aset.contains(rp.pick())) ok = false;
        System.out.println("sample in allowed: " + ok);
    }
}
