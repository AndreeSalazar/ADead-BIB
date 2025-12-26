/**
 * ADead-BIB Matrix Class
 * Author: Eddi Andreé Salazar Matos
 */
package com.adead;

import java.util.Random;

public class Matrix {
    public float[] data;
    public int rows;
    public int cols;
    
    public Matrix(int rows, int cols) {
        this.rows = rows;
        this.cols = cols;
        this.data = new float[rows * cols];
    }
    
    public static Matrix zeros(int rows, int cols) {
        return new Matrix(rows, cols);
    }
    
    public static Matrix ones(int rows, int cols) {
        Matrix m = new Matrix(rows, cols);
        for (int i = 0; i < m.data.length; i++) {
            m.data[i] = 1.0f;
        }
        return m;
    }
    
    public static Matrix random(int rows, int cols) {
        Matrix m = new Matrix(rows, cols);
        Random rand = new Random();
        for (int i = 0; i < m.data.length; i++) {
            m.data[i] = rand.nextFloat() * 2 - 1;
        }
        return m;
    }
    
    public static Matrix eye(int size) {
        Matrix m = new Matrix(size, size);
        for (int i = 0; i < size; i++) {
            m.data[i * size + i] = 1.0f;
        }
        return m;
    }
    
    public float get(int row, int col) {
        return data[row * cols + col];
    }
    
    public void set(int row, int col, float value) {
        data[row * cols + col] = value;
    }
    
    public float[] toArray() {
        return data.clone();
    }
    
    public static Matrix fromArray(float[][] arr) {
        int rows = arr.length;
        int cols = arr[0].length;
        Matrix m = new Matrix(rows, cols);
        for (int i = 0; i < rows; i++) {
            for (int j = 0; j < cols; j++) {
                m.set(i, j, arr[i][j]);
            }
        }
        return m;
    }
}
