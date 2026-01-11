// 2D iterator over array of arrays without flattening. next/hasNext amortized O(1).
import java.util.*;

public class Solution {
    static class Iterator2D {
        int[][] data;
        int row = 0, col = 0;
        Iterator2D(int[][] d){ data = d; }
        private void advance(){
            while(row < data.length && col >= data[row].length){ row++; col = 0; }
        }
        boolean hasNext(){ advance(); return row < data.length; }
        int next(){
            if(!hasNext()) throw new NoSuchElementException();
            return data[row][col++];
        }
    }

    public static void main(String[] args){
        int[][] arr = {{1, 2}, {3}, {}, {4, 5, 6}};
        Iterator2D it = new Iterator2D(arr);
        StringBuilder sb = new StringBuilder();
        boolean first = true;
        while(it.hasNext()){
            if(!first) sb.append(", ");
            sb.append(it.next());
            first = false;
        }
        System.out.println(sb);
    }
}
