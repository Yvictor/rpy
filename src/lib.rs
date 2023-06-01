use pyo3::prelude::*;

use lazy_static::lazy_static;
use std::collections::HashMap;

static IX: [f64; 4999] = [
    0.01, 0.02, 0.03, 0.04, 0.05, 0.06, 0.07, 0.08, 0.09, 0.1, 0.11, 0.12, 0.13, 0.14, 0.15, 0.16,
    0.17, 0.18, 0.19, 0.2, 0.21, 0.22, 0.23, 0.24, 0.25, 0.26, 0.27, 0.28, 0.29, 0.3, 0.31, 0.32,
    0.33, 0.34, 0.35, 0.36, 0.37, 0.38, 0.39, 0.4, 0.41, 0.42, 0.43, 0.44, 0.45, 0.46, 0.47, 0.48,
    0.49, 0.5, 0.51, 0.52, 0.53, 0.54, 0.55, 0.56, 0.57, 0.58, 0.59, 0.6, 0.61, 0.62, 0.63, 0.64,
    0.65, 0.66, 0.67, 0.68, 0.69, 0.7, 0.71, 0.72, 0.73, 0.74, 0.75, 0.76, 0.77, 0.78, 0.79, 0.8,
    0.81, 0.82, 0.83, 0.84, 0.85, 0.86, 0.87, 0.88, 0.89, 0.9, 0.91, 0.92, 0.93, 0.94, 0.95, 0.96,
    0.97, 0.98, 0.99, 1.0, 1.01, 1.02, 1.03, 1.04, 1.05, 1.06, 1.07, 1.08, 1.09, 1.1, 1.11, 1.12,
    1.13, 1.14, 1.15, 1.16, 1.17, 1.18, 1.19, 1.2, 1.21, 1.22, 1.23, 1.24, 1.25, 1.26, 1.27, 1.28,
    1.29, 1.3, 1.31, 1.32, 1.33, 1.34, 1.35, 1.36, 1.37, 1.38, 1.39, 1.4, 1.41, 1.42, 1.43, 1.44,
    1.45, 1.46, 1.47, 1.48, 1.49, 1.5, 1.51, 1.52, 1.53, 1.54, 1.55, 1.56, 1.57, 1.58, 1.59, 1.6,
    1.61, 1.62, 1.63, 1.64, 1.65, 1.66, 1.67, 1.68, 1.69, 1.7, 1.71, 1.72, 1.73, 1.74, 1.75, 1.76,
    1.77, 1.78, 1.79, 1.8, 1.81, 1.82, 1.83, 1.84, 1.85, 1.86, 1.87, 1.88, 1.89, 1.9, 1.91, 1.92,
    1.93, 1.94, 1.95, 1.96, 1.97, 1.98, 1.99, 2.0, 2.01, 2.02, 2.03, 2.04, 2.05, 2.06, 2.07, 2.08,
    2.09, 2.1, 2.11, 2.12, 2.13, 2.14, 2.15, 2.16, 2.17, 2.18, 2.19, 2.2, 2.21, 2.22, 2.23, 2.24,
    2.25, 2.26, 2.27, 2.28, 2.29, 2.3, 2.31, 2.32, 2.33, 2.34, 2.35, 2.36, 2.37, 2.38, 2.39, 2.4,
    2.41, 2.42, 2.43, 2.44, 2.45, 2.46, 2.47, 2.48, 2.49, 2.5, 2.51, 2.52, 2.53, 2.54, 2.55, 2.56,
    2.57, 2.58, 2.59, 2.6, 2.61, 2.62, 2.63, 2.64, 2.65, 2.66, 2.67, 2.68, 2.69, 2.7, 2.71, 2.72,
    2.73, 2.74, 2.75, 2.76, 2.77, 2.78, 2.79, 2.8, 2.81, 2.82, 2.83, 2.84, 2.85, 2.86, 2.87, 2.88,
    2.89, 2.9, 2.91, 2.92, 2.93, 2.94, 2.95, 2.96, 2.97, 2.98, 2.99, 3.0, 3.01, 3.02, 3.03, 3.04,
    3.05, 3.06, 3.07, 3.08, 3.09, 3.1, 3.11, 3.12, 3.13, 3.14, 3.15, 3.16, 3.17, 3.18, 3.19, 3.2,
    3.21, 3.22, 3.23, 3.24, 3.25, 3.26, 3.27, 3.28, 3.29, 3.3, 3.31, 3.32, 3.33, 3.34, 3.35, 3.36,
    3.37, 3.38, 3.39, 3.4, 3.41, 3.42, 3.43, 3.44, 3.45, 3.46, 3.47, 3.48, 3.49, 3.5, 3.51, 3.52,
    3.53, 3.54, 3.55, 3.56, 3.57, 3.58, 3.59, 3.6, 3.61, 3.62, 3.63, 3.64, 3.65, 3.66, 3.67, 3.68,
    3.69, 3.7, 3.71, 3.72, 3.73, 3.74, 3.75, 3.76, 3.77, 3.78, 3.79, 3.8, 3.81, 3.82, 3.83, 3.84,
    3.85, 3.86, 3.87, 3.88, 3.89, 3.9, 3.91, 3.92, 3.93, 3.94, 3.95, 3.96, 3.97, 3.98, 3.99, 4.0,
    4.01, 4.02, 4.03, 4.04, 4.05, 4.06, 4.07, 4.08, 4.09, 4.1, 4.11, 4.12, 4.13, 4.14, 4.15, 4.16,
    4.17, 4.18, 4.19, 4.2, 4.21, 4.22, 4.23, 4.24, 4.25, 4.26, 4.27, 4.28, 4.29, 4.3, 4.31, 4.32,
    4.33, 4.34, 4.35, 4.36, 4.37, 4.38, 4.39, 4.4, 4.41, 4.42, 4.43, 4.44, 4.45, 4.46, 4.47, 4.48,
    4.49, 4.5, 4.51, 4.52, 4.53, 4.54, 4.55, 4.56, 4.57, 4.58, 4.59, 4.6, 4.61, 4.62, 4.63, 4.64,
    4.65, 4.66, 4.67, 4.68, 4.69, 4.7, 4.71, 4.72, 4.73, 4.74, 4.75, 4.76, 4.77, 4.78, 4.79, 4.8,
    4.81, 4.82, 4.83, 4.84, 4.85, 4.86, 4.87, 4.88, 4.89, 4.9, 4.91, 4.92, 4.93, 4.94, 4.95, 4.96,
    4.97, 4.98, 4.99, 5.0, 5.01, 5.02, 5.03, 5.04, 5.05, 5.06, 5.07, 5.08, 5.09, 5.1, 5.11, 5.12,
    5.13, 5.14, 5.15, 5.16, 5.17, 5.18, 5.19, 5.2, 5.21, 5.22, 5.23, 5.24, 5.25, 5.26, 5.27, 5.28,
    5.29, 5.3, 5.31, 5.32, 5.33, 5.34, 5.35, 5.36, 5.37, 5.38, 5.39, 5.4, 5.41, 5.42, 5.43, 5.44,
    5.45, 5.46, 5.47, 5.48, 5.49, 5.5, 5.51, 5.52, 5.53, 5.54, 5.55, 5.56, 5.57, 5.58, 5.59, 5.6,
    5.61, 5.62, 5.63, 5.64, 5.65, 5.66, 5.67, 5.68, 5.69, 5.7, 5.71, 5.72, 5.73, 5.74, 5.75, 5.76,
    5.77, 5.78, 5.79, 5.8, 5.81, 5.82, 5.83, 5.84, 5.85, 5.86, 5.87, 5.88, 5.89, 5.9, 5.91, 5.92,
    5.93, 5.94, 5.95, 5.96, 5.97, 5.98, 5.99, 6.0, 6.01, 6.02, 6.03, 6.04, 6.05, 6.06, 6.07, 6.08,
    6.09, 6.1, 6.11, 6.12, 6.13, 6.14, 6.15, 6.16, 6.17, 6.18, 6.19, 6.2, 6.21, 6.22, 6.23, 6.24,
    6.25, 6.26, 6.27, 6.28, 6.29, 6.3, 6.31, 6.32, 6.33, 6.34, 6.35, 6.36, 6.37, 6.38, 6.39, 6.4,
    6.41, 6.42, 6.43, 6.44, 6.45, 6.46, 6.47, 6.48, 6.49, 6.5, 6.51, 6.52, 6.53, 6.54, 6.55, 6.56,
    6.57, 6.58, 6.59, 6.6, 6.61, 6.62, 6.63, 6.64, 6.65, 6.66, 6.67, 6.68, 6.69, 6.7, 6.71, 6.72,
    6.73, 6.74, 6.75, 6.76, 6.77, 6.78, 6.79, 6.8, 6.81, 6.82, 6.83, 6.84, 6.85, 6.86, 6.87, 6.88,
    6.89, 6.9, 6.91, 6.92, 6.93, 6.94, 6.95, 6.96, 6.97, 6.98, 6.99, 7.0, 7.01, 7.02, 7.03, 7.04,
    7.05, 7.06, 7.07, 7.08, 7.09, 7.1, 7.11, 7.12, 7.13, 7.14, 7.15, 7.16, 7.17, 7.18, 7.19, 7.2,
    7.21, 7.22, 7.23, 7.24, 7.25, 7.26, 7.27, 7.28, 7.29, 7.3, 7.31, 7.32, 7.33, 7.34, 7.35, 7.36,
    7.37, 7.38, 7.39, 7.4, 7.41, 7.42, 7.43, 7.44, 7.45, 7.46, 7.47, 7.48, 7.49, 7.5, 7.51, 7.52,
    7.53, 7.54, 7.55, 7.56, 7.57, 7.58, 7.59, 7.6, 7.61, 7.62, 7.63, 7.64, 7.65, 7.66, 7.67, 7.68,
    7.69, 7.7, 7.71, 7.72, 7.73, 7.74, 7.75, 7.76, 7.77, 7.78, 7.79, 7.8, 7.81, 7.82, 7.83, 7.84,
    7.85, 7.86, 7.87, 7.88, 7.89, 7.9, 7.91, 7.92, 7.93, 7.94, 7.95, 7.96, 7.97, 7.98, 7.99, 8.0,
    8.01, 8.02, 8.03, 8.04, 8.05, 8.06, 8.07, 8.08, 8.09, 8.1, 8.11, 8.12, 8.13, 8.14, 8.15, 8.16,
    8.17, 8.18, 8.19, 8.2, 8.21, 8.22, 8.23, 8.24, 8.25, 8.26, 8.27, 8.28, 8.29, 8.3, 8.31, 8.32,
    8.33, 8.34, 8.35, 8.36, 8.37, 8.38, 8.39, 8.4, 8.41, 8.42, 8.43, 8.44, 8.45, 8.46, 8.47, 8.48,
    8.49, 8.5, 8.51, 8.52, 8.53, 8.54, 8.55, 8.56, 8.57, 8.58, 8.59, 8.6, 8.61, 8.62, 8.63, 8.64,
    8.65, 8.66, 8.67, 8.68, 8.69, 8.7, 8.71, 8.72, 8.73, 8.74, 8.75, 8.76, 8.77, 8.78, 8.79, 8.8,
    8.81, 8.82, 8.83, 8.84, 8.85, 8.86, 8.87, 8.88, 8.89, 8.9, 8.91, 8.92, 8.93, 8.94, 8.95, 8.96,
    8.97, 8.98, 8.99, 9.0, 9.01, 9.02, 9.03, 9.04, 9.05, 9.06, 9.07, 9.08, 9.09, 9.1, 9.11, 9.12,
    9.13, 9.14, 9.15, 9.16, 9.17, 9.18, 9.19, 9.2, 9.21, 9.22, 9.23, 9.24, 9.25, 9.26, 9.27, 9.28,
    9.29, 9.3, 9.31, 9.32, 9.33, 9.34, 9.35, 9.36, 9.37, 9.38, 9.39, 9.4, 9.41, 9.42, 9.43, 9.44,
    9.45, 9.46, 9.47, 9.48, 9.49, 9.5, 9.51, 9.52, 9.53, 9.54, 9.55, 9.56, 9.57, 9.58, 9.59, 9.6,
    9.61, 9.62, 9.63, 9.64, 9.65, 9.66, 9.67, 9.68, 9.69, 9.7, 9.71, 9.72, 9.73, 9.74, 9.75, 9.76,
    9.77, 9.78, 9.79, 9.8, 9.81, 9.82, 9.83, 9.84, 9.85, 9.86, 9.87, 9.88, 9.89, 9.9, 9.91, 9.92,
    9.93, 9.94, 9.95, 9.96, 9.97, 9.98, 9.99, 10.0, 10.05, 10.1, 10.15, 10.2, 10.25, 10.3, 10.35,
    10.4, 10.45, 10.5, 10.55, 10.6, 10.65, 10.7, 10.75, 10.8, 10.85, 10.9, 10.95, 11.0, 11.05,
    11.1, 11.15, 11.2, 11.25, 11.3, 11.35, 11.4, 11.45, 11.5, 11.55, 11.6, 11.65, 11.7, 11.75,
    11.8, 11.85, 11.9, 11.95, 12.0, 12.05, 12.1, 12.15, 12.2, 12.25, 12.3, 12.35, 12.4, 12.45,
    12.5, 12.55, 12.6, 12.65, 12.7, 12.75, 12.8, 12.85, 12.9, 12.95, 13.0, 13.05, 13.1, 13.15,
    13.2, 13.25, 13.3, 13.35, 13.4, 13.45, 13.5, 13.55, 13.6, 13.65, 13.7, 13.75, 13.8, 13.85,
    13.9, 13.95, 14.0, 14.05, 14.1, 14.15, 14.2, 14.25, 14.3, 14.35, 14.4, 14.45, 14.5, 14.55,
    14.6, 14.65, 14.7, 14.75, 14.8, 14.85, 14.9, 14.95, 15.0, 15.05, 15.1, 15.15, 15.2, 15.25,
    15.3, 15.35, 15.4, 15.45, 15.5, 15.55, 15.6, 15.65, 15.7, 15.75, 15.8, 15.85, 15.9, 15.95,
    16.0, 16.05, 16.1, 16.15, 16.2, 16.25, 16.3, 16.35, 16.4, 16.45, 16.5, 16.55, 16.6, 16.65,
    16.7, 16.75, 16.8, 16.85, 16.9, 16.95, 17.0, 17.05, 17.1, 17.15, 17.2, 17.25, 17.3, 17.35,
    17.4, 17.45, 17.5, 17.55, 17.6, 17.65, 17.7, 17.75, 17.8, 17.85, 17.9, 17.95, 18.0, 18.05,
    18.1, 18.15, 18.2, 18.25, 18.3, 18.35, 18.4, 18.45, 18.5, 18.55, 18.6, 18.65, 18.7, 18.75,
    18.8, 18.85, 18.9, 18.95, 19.0, 19.05, 19.1, 19.15, 19.2, 19.25, 19.3, 19.35, 19.4, 19.45,
    19.5, 19.55, 19.6, 19.65, 19.7, 19.75, 19.8, 19.85, 19.9, 19.95, 20.0, 20.05, 20.1, 20.15,
    20.2, 20.25, 20.3, 20.35, 20.4, 20.45, 20.5, 20.55, 20.6, 20.65, 20.7, 20.75, 20.8, 20.85,
    20.9, 20.95, 21.0, 21.05, 21.1, 21.15, 21.2, 21.25, 21.3, 21.35, 21.4, 21.45, 21.5, 21.55,
    21.6, 21.65, 21.7, 21.75, 21.8, 21.85, 21.9, 21.95, 22.0, 22.05, 22.1, 22.15, 22.2, 22.25,
    22.3, 22.35, 22.4, 22.45, 22.5, 22.55, 22.6, 22.65, 22.7, 22.75, 22.8, 22.85, 22.9, 22.95,
    23.0, 23.05, 23.1, 23.15, 23.2, 23.25, 23.3, 23.35, 23.4, 23.45, 23.5, 23.55, 23.6, 23.65,
    23.7, 23.75, 23.8, 23.85, 23.9, 23.95, 24.0, 24.05, 24.1, 24.15, 24.2, 24.25, 24.3, 24.35,
    24.4, 24.45, 24.5, 24.55, 24.6, 24.65, 24.7, 24.75, 24.8, 24.85, 24.9, 24.95, 25.0, 25.05,
    25.1, 25.15, 25.2, 25.25, 25.3, 25.35, 25.4, 25.45, 25.5, 25.55, 25.6, 25.65, 25.7, 25.75,
    25.8, 25.85, 25.9, 25.95, 26.0, 26.05, 26.1, 26.15, 26.2, 26.25, 26.3, 26.35, 26.4, 26.45,
    26.5, 26.55, 26.6, 26.65, 26.7, 26.75, 26.8, 26.85, 26.9, 26.95, 27.0, 27.05, 27.1, 27.15,
    27.2, 27.25, 27.3, 27.35, 27.4, 27.45, 27.5, 27.55, 27.6, 27.65, 27.7, 27.75, 27.8, 27.85,
    27.9, 27.95, 28.0, 28.05, 28.1, 28.15, 28.2, 28.25, 28.3, 28.35, 28.4, 28.45, 28.5, 28.55,
    28.6, 28.65, 28.7, 28.75, 28.8, 28.85, 28.9, 28.95, 29.0, 29.05, 29.1, 29.15, 29.2, 29.25,
    29.3, 29.35, 29.4, 29.45, 29.5, 29.55, 29.6, 29.65, 29.7, 29.75, 29.8, 29.85, 29.9, 29.95,
    30.0, 30.05, 30.1, 30.15, 30.2, 30.25, 30.3, 30.35, 30.4, 30.45, 30.5, 30.55, 30.6, 30.65,
    30.7, 30.75, 30.8, 30.85, 30.9, 30.95, 31.0, 31.05, 31.1, 31.15, 31.2, 31.25, 31.3, 31.35,
    31.4, 31.45, 31.5, 31.55, 31.6, 31.65, 31.7, 31.75, 31.8, 31.85, 31.9, 31.95, 32.0, 32.05,
    32.1, 32.15, 32.2, 32.25, 32.3, 32.35, 32.4, 32.45, 32.5, 32.55, 32.6, 32.65, 32.7, 32.75,
    32.8, 32.85, 32.9, 32.95, 33.0, 33.05, 33.1, 33.15, 33.2, 33.25, 33.3, 33.35, 33.4, 33.45,
    33.5, 33.55, 33.6, 33.65, 33.7, 33.75, 33.8, 33.85, 33.9, 33.95, 34.0, 34.05, 34.1, 34.15,
    34.2, 34.25, 34.3, 34.35, 34.4, 34.45, 34.5, 34.55, 34.6, 34.65, 34.7, 34.75, 34.8, 34.85,
    34.9, 34.95, 35.0, 35.05, 35.1, 35.15, 35.2, 35.25, 35.3, 35.35, 35.4, 35.45, 35.5, 35.55,
    35.6, 35.65, 35.7, 35.75, 35.8, 35.85, 35.9, 35.95, 36.0, 36.05, 36.1, 36.15, 36.2, 36.25,
    36.3, 36.35, 36.4, 36.45, 36.5, 36.55, 36.6, 36.65, 36.7, 36.75, 36.8, 36.85, 36.9, 36.95,
    37.0, 37.05, 37.1, 37.15, 37.2, 37.25, 37.3, 37.35, 37.4, 37.45, 37.5, 37.55, 37.6, 37.65,
    37.7, 37.75, 37.8, 37.85, 37.9, 37.95, 38.0, 38.05, 38.1, 38.15, 38.2, 38.25, 38.3, 38.35,
    38.4, 38.45, 38.5, 38.55, 38.6, 38.65, 38.7, 38.75, 38.8, 38.85, 38.9, 38.95, 39.0, 39.05,
    39.1, 39.15, 39.2, 39.25, 39.3, 39.35, 39.4, 39.45, 39.5, 39.55, 39.6, 39.65, 39.7, 39.75,
    39.8, 39.85, 39.9, 39.95, 40.0, 40.05, 40.1, 40.15, 40.2, 40.25, 40.3, 40.35, 40.4, 40.45,
    40.5, 40.55, 40.6, 40.65, 40.7, 40.75, 40.8, 40.85, 40.9, 40.95, 41.0, 41.05, 41.1, 41.15,
    41.2, 41.25, 41.3, 41.35, 41.4, 41.45, 41.5, 41.55, 41.6, 41.65, 41.7, 41.75, 41.8, 41.85,
    41.9, 41.95, 42.0, 42.05, 42.1, 42.15, 42.2, 42.25, 42.3, 42.35, 42.4, 42.45, 42.5, 42.55,
    42.6, 42.65, 42.7, 42.75, 42.8, 42.85, 42.9, 42.95, 43.0, 43.05, 43.1, 43.15, 43.2, 43.25,
    43.3, 43.35, 43.4, 43.45, 43.5, 43.55, 43.6, 43.65, 43.7, 43.75, 43.8, 43.85, 43.9, 43.95,
    44.0, 44.05, 44.1, 44.15, 44.2, 44.25, 44.3, 44.35, 44.4, 44.45, 44.5, 44.55, 44.6, 44.65,
    44.7, 44.75, 44.8, 44.85, 44.9, 44.95, 45.0, 45.05, 45.1, 45.15, 45.2, 45.25, 45.3, 45.35,
    45.4, 45.45, 45.5, 45.55, 45.6, 45.65, 45.7, 45.75, 45.8, 45.85, 45.9, 45.95, 46.0, 46.05,
    46.1, 46.15, 46.2, 46.25, 46.3, 46.35, 46.4, 46.45, 46.5, 46.55, 46.6, 46.65, 46.7, 46.75,
    46.8, 46.85, 46.9, 46.95, 47.0, 47.05, 47.1, 47.15, 47.2, 47.25, 47.3, 47.35, 47.4, 47.45,
    47.5, 47.55, 47.6, 47.65, 47.7, 47.75, 47.8, 47.85, 47.9, 47.95, 48.0, 48.05, 48.1, 48.15,
    48.2, 48.25, 48.3, 48.35, 48.4, 48.45, 48.5, 48.55, 48.6, 48.65, 48.7, 48.75, 48.8, 48.85,
    48.9, 48.95, 49.0, 49.05, 49.1, 49.15, 49.2, 49.25, 49.3, 49.35, 49.4, 49.45, 49.5, 49.55,
    49.6, 49.65, 49.7, 49.75, 49.8, 49.85, 49.9, 49.95, 50.0, 50.1, 50.2, 50.3, 50.4, 50.5, 50.6,
    50.7, 50.8, 50.9, 51.0, 51.1, 51.2, 51.3, 51.4, 51.5, 51.6, 51.7, 51.8, 51.9, 52.0, 52.1, 52.2,
    52.3, 52.4, 52.5, 52.6, 52.7, 52.8, 52.9, 53.0, 53.1, 53.2, 53.3, 53.4, 53.5, 53.6, 53.7, 53.8,
    53.9, 54.0, 54.1, 54.2, 54.3, 54.4, 54.5, 54.6, 54.7, 54.8, 54.9, 55.0, 55.1, 55.2, 55.3, 55.4,
    55.5, 55.6, 55.7, 55.8, 55.9, 56.0, 56.1, 56.2, 56.3, 56.4, 56.5, 56.6, 56.7, 56.8, 56.9, 57.0,
    57.1, 57.2, 57.3, 57.4, 57.5, 57.6, 57.7, 57.8, 57.9, 58.0, 58.1, 58.2, 58.3, 58.4, 58.5, 58.6,
    58.7, 58.8, 58.9, 59.0, 59.1, 59.2, 59.3, 59.4, 59.5, 59.6, 59.7, 59.8, 59.9, 60.0, 60.1, 60.2,
    60.3, 60.4, 60.5, 60.6, 60.7, 60.8, 60.9, 61.0, 61.1, 61.2, 61.3, 61.4, 61.5, 61.6, 61.7, 61.8,
    61.9, 62.0, 62.1, 62.2, 62.3, 62.4, 62.5, 62.6, 62.7, 62.8, 62.9, 63.0, 63.1, 63.2, 63.3, 63.4,
    63.5, 63.6, 63.7, 63.8, 63.9, 64.0, 64.1, 64.2, 64.3, 64.4, 64.5, 64.6, 64.7, 64.8, 64.9, 65.0,
    65.1, 65.2, 65.3, 65.4, 65.5, 65.6, 65.7, 65.8, 65.9, 66.0, 66.1, 66.2, 66.3, 66.4, 66.5, 66.6,
    66.7, 66.8, 66.9, 67.0, 67.1, 67.2, 67.3, 67.4, 67.5, 67.6, 67.7, 67.8, 67.9, 68.0, 68.1, 68.2,
    68.3, 68.4, 68.5, 68.6, 68.7, 68.8, 68.9, 69.0, 69.1, 69.2, 69.3, 69.4, 69.5, 69.6, 69.7, 69.8,
    69.9, 70.0, 70.1, 70.2, 70.3, 70.4, 70.5, 70.6, 70.7, 70.8, 70.9, 71.0, 71.1, 71.2, 71.3, 71.4,
    71.5, 71.6, 71.7, 71.8, 71.9, 72.0, 72.1, 72.2, 72.3, 72.4, 72.5, 72.6, 72.7, 72.8, 72.9, 73.0,
    73.1, 73.2, 73.3, 73.4, 73.5, 73.6, 73.7, 73.8, 73.9, 74.0, 74.1, 74.2, 74.3, 74.4, 74.5, 74.6,
    74.7, 74.8, 74.9, 75.0, 75.1, 75.2, 75.3, 75.4, 75.5, 75.6, 75.7, 75.8, 75.9, 76.0, 76.1, 76.2,
    76.3, 76.4, 76.5, 76.6, 76.7, 76.8, 76.9, 77.0, 77.1, 77.2, 77.3, 77.4, 77.5, 77.6, 77.7, 77.8,
    77.9, 78.0, 78.1, 78.2, 78.3, 78.4, 78.5, 78.6, 78.7, 78.8, 78.9, 79.0, 79.1, 79.2, 79.3, 79.4,
    79.5, 79.6, 79.7, 79.8, 79.9, 80.0, 80.1, 80.2, 80.3, 80.4, 80.5, 80.6, 80.7, 80.8, 80.9, 81.0,
    81.1, 81.2, 81.3, 81.4, 81.5, 81.6, 81.7, 81.8, 81.9, 82.0, 82.1, 82.2, 82.3, 82.4, 82.5, 82.6,
    82.7, 82.8, 82.9, 83.0, 83.1, 83.2, 83.3, 83.4, 83.5, 83.6, 83.7, 83.8, 83.9, 84.0, 84.1, 84.2,
    84.3, 84.4, 84.5, 84.6, 84.7, 84.8, 84.9, 85.0, 85.1, 85.2, 85.3, 85.4, 85.5, 85.6, 85.7, 85.8,
    85.9, 86.0, 86.1, 86.2, 86.3, 86.4, 86.5, 86.6, 86.7, 86.8, 86.9, 87.0, 87.1, 87.2, 87.3, 87.4,
    87.5, 87.6, 87.7, 87.8, 87.9, 88.0, 88.1, 88.2, 88.3, 88.4, 88.5, 88.6, 88.7, 88.8, 88.9, 89.0,
    89.1, 89.2, 89.3, 89.4, 89.5, 89.6, 89.7, 89.8, 89.9, 90.0, 90.1, 90.2, 90.3, 90.4, 90.5, 90.6,
    90.7, 90.8, 90.9, 91.0, 91.1, 91.2, 91.3, 91.4, 91.5, 91.6, 91.7, 91.8, 91.9, 92.0, 92.1, 92.2,
    92.3, 92.4, 92.5, 92.6, 92.7, 92.8, 92.9, 93.0, 93.1, 93.2, 93.3, 93.4, 93.5, 93.6, 93.7, 93.8,
    93.9, 94.0, 94.1, 94.2, 94.3, 94.4, 94.5, 94.6, 94.7, 94.8, 94.9, 95.0, 95.1, 95.2, 95.3, 95.4,
    95.5, 95.6, 95.7, 95.8, 95.9, 96.0, 96.1, 96.2, 96.3, 96.4, 96.5, 96.6, 96.7, 96.8, 96.9, 97.0,
    97.1, 97.2, 97.3, 97.4, 97.5, 97.6, 97.7, 97.8, 97.9, 98.0, 98.1, 98.2, 98.3, 98.4, 98.5, 98.6,
    98.7, 98.8, 98.9, 99.0, 99.1, 99.2, 99.3, 99.4, 99.5, 99.6, 99.7, 99.8, 99.9, 100.0, 100.5,
    101.0, 101.5, 102.0, 102.5, 103.0, 103.5, 104.0, 104.5, 105.0, 105.5, 106.0, 106.5, 107.0,
    107.5, 108.0, 108.5, 109.0, 109.5, 110.0, 110.5, 111.0, 111.5, 112.0, 112.5, 113.0, 113.5,
    114.0, 114.5, 115.0, 115.5, 116.0, 116.5, 117.0, 117.5, 118.0, 118.5, 119.0, 119.5, 120.0,
    120.5, 121.0, 121.5, 122.0, 122.5, 123.0, 123.5, 124.0, 124.5, 125.0, 125.5, 126.0, 126.5,
    127.0, 127.5, 128.0, 128.5, 129.0, 129.5, 130.0, 130.5, 131.0, 131.5, 132.0, 132.5, 133.0,
    133.5, 134.0, 134.5, 135.0, 135.5, 136.0, 136.5, 137.0, 137.5, 138.0, 138.5, 139.0, 139.5,
    140.0, 140.5, 141.0, 141.5, 142.0, 142.5, 143.0, 143.5, 144.0, 144.5, 145.0, 145.5, 146.0,
    146.5, 147.0, 147.5, 148.0, 148.5, 149.0, 149.5, 150.0, 150.5, 151.0, 151.5, 152.0, 152.5,
    153.0, 153.5, 154.0, 154.5, 155.0, 155.5, 156.0, 156.5, 157.0, 157.5, 158.0, 158.5, 159.0,
    159.5, 160.0, 160.5, 161.0, 161.5, 162.0, 162.5, 163.0, 163.5, 164.0, 164.5, 165.0, 165.5,
    166.0, 166.5, 167.0, 167.5, 168.0, 168.5, 169.0, 169.5, 170.0, 170.5, 171.0, 171.5, 172.0,
    172.5, 173.0, 173.5, 174.0, 174.5, 175.0, 175.5, 176.0, 176.5, 177.0, 177.5, 178.0, 178.5,
    179.0, 179.5, 180.0, 180.5, 181.0, 181.5, 182.0, 182.5, 183.0, 183.5, 184.0, 184.5, 185.0,
    185.5, 186.0, 186.5, 187.0, 187.5, 188.0, 188.5, 189.0, 189.5, 190.0, 190.5, 191.0, 191.5,
    192.0, 192.5, 193.0, 193.5, 194.0, 194.5, 195.0, 195.5, 196.0, 196.5, 197.0, 197.5, 198.0,
    198.5, 199.0, 199.5, 200.0, 200.5, 201.0, 201.5, 202.0, 202.5, 203.0, 203.5, 204.0, 204.5,
    205.0, 205.5, 206.0, 206.5, 207.0, 207.5, 208.0, 208.5, 209.0, 209.5, 210.0, 210.5, 211.0,
    211.5, 212.0, 212.5, 213.0, 213.5, 214.0, 214.5, 215.0, 215.5, 216.0, 216.5, 217.0, 217.5,
    218.0, 218.5, 219.0, 219.5, 220.0, 220.5, 221.0, 221.5, 222.0, 222.5, 223.0, 223.5, 224.0,
    224.5, 225.0, 225.5, 226.0, 226.5, 227.0, 227.5, 228.0, 228.5, 229.0, 229.5, 230.0, 230.5,
    231.0, 231.5, 232.0, 232.5, 233.0, 233.5, 234.0, 234.5, 235.0, 235.5, 236.0, 236.5, 237.0,
    237.5, 238.0, 238.5, 239.0, 239.5, 240.0, 240.5, 241.0, 241.5, 242.0, 242.5, 243.0, 243.5,
    244.0, 244.5, 245.0, 245.5, 246.0, 246.5, 247.0, 247.5, 248.0, 248.5, 249.0, 249.5, 250.0,
    250.5, 251.0, 251.5, 252.0, 252.5, 253.0, 253.5, 254.0, 254.5, 255.0, 255.5, 256.0, 256.5,
    257.0, 257.5, 258.0, 258.5, 259.0, 259.5, 260.0, 260.5, 261.0, 261.5, 262.0, 262.5, 263.0,
    263.5, 264.0, 264.5, 265.0, 265.5, 266.0, 266.5, 267.0, 267.5, 268.0, 268.5, 269.0, 269.5,
    270.0, 270.5, 271.0, 271.5, 272.0, 272.5, 273.0, 273.5, 274.0, 274.5, 275.0, 275.5, 276.0,
    276.5, 277.0, 277.5, 278.0, 278.5, 279.0, 279.5, 280.0, 280.5, 281.0, 281.5, 282.0, 282.5,
    283.0, 283.5, 284.0, 284.5, 285.0, 285.5, 286.0, 286.5, 287.0, 287.5, 288.0, 288.5, 289.0,
    289.5, 290.0, 290.5, 291.0, 291.5, 292.0, 292.5, 293.0, 293.5, 294.0, 294.5, 295.0, 295.5,
    296.0, 296.5, 297.0, 297.5, 298.0, 298.5, 299.0, 299.5, 300.0, 300.5, 301.0, 301.5, 302.0,
    302.5, 303.0, 303.5, 304.0, 304.5, 305.0, 305.5, 306.0, 306.5, 307.0, 307.5, 308.0, 308.5,
    309.0, 309.5, 310.0, 310.5, 311.0, 311.5, 312.0, 312.5, 313.0, 313.5, 314.0, 314.5, 315.0,
    315.5, 316.0, 316.5, 317.0, 317.5, 318.0, 318.5, 319.0, 319.5, 320.0, 320.5, 321.0, 321.5,
    322.0, 322.5, 323.0, 323.5, 324.0, 324.5, 325.0, 325.5, 326.0, 326.5, 327.0, 327.5, 328.0,
    328.5, 329.0, 329.5, 330.0, 330.5, 331.0, 331.5, 332.0, 332.5, 333.0, 333.5, 334.0, 334.5,
    335.0, 335.5, 336.0, 336.5, 337.0, 337.5, 338.0, 338.5, 339.0, 339.5, 340.0, 340.5, 341.0,
    341.5, 342.0, 342.5, 343.0, 343.5, 344.0, 344.5, 345.0, 345.5, 346.0, 346.5, 347.0, 347.5,
    348.0, 348.5, 349.0, 349.5, 350.0, 350.5, 351.0, 351.5, 352.0, 352.5, 353.0, 353.5, 354.0,
    354.5, 355.0, 355.5, 356.0, 356.5, 357.0, 357.5, 358.0, 358.5, 359.0, 359.5, 360.0, 360.5,
    361.0, 361.5, 362.0, 362.5, 363.0, 363.5, 364.0, 364.5, 365.0, 365.5, 366.0, 366.5, 367.0,
    367.5, 368.0, 368.5, 369.0, 369.5, 370.0, 370.5, 371.0, 371.5, 372.0, 372.5, 373.0, 373.5,
    374.0, 374.5, 375.0, 375.5, 376.0, 376.5, 377.0, 377.5, 378.0, 378.5, 379.0, 379.5, 380.0,
    380.5, 381.0, 381.5, 382.0, 382.5, 383.0, 383.5, 384.0, 384.5, 385.0, 385.5, 386.0, 386.5,
    387.0, 387.5, 388.0, 388.5, 389.0, 389.5, 390.0, 390.5, 391.0, 391.5, 392.0, 392.5, 393.0,
    393.5, 394.0, 394.5, 395.0, 395.5, 396.0, 396.5, 397.0, 397.5, 398.0, 398.5, 399.0, 399.5,
    400.0, 400.5, 401.0, 401.5, 402.0, 402.5, 403.0, 403.5, 404.0, 404.5, 405.0, 405.5, 406.0,
    406.5, 407.0, 407.5, 408.0, 408.5, 409.0, 409.5, 410.0, 410.5, 411.0, 411.5, 412.0, 412.5,
    413.0, 413.5, 414.0, 414.5, 415.0, 415.5, 416.0, 416.5, 417.0, 417.5, 418.0, 418.5, 419.0,
    419.5, 420.0, 420.5, 421.0, 421.5, 422.0, 422.5, 423.0, 423.5, 424.0, 424.5, 425.0, 425.5,
    426.0, 426.5, 427.0, 427.5, 428.0, 428.5, 429.0, 429.5, 430.0, 430.5, 431.0, 431.5, 432.0,
    432.5, 433.0, 433.5, 434.0, 434.5, 435.0, 435.5, 436.0, 436.5, 437.0, 437.5, 438.0, 438.5,
    439.0, 439.5, 440.0, 440.5, 441.0, 441.5, 442.0, 442.5, 443.0, 443.5, 444.0, 444.5, 445.0,
    445.5, 446.0, 446.5, 447.0, 447.5, 448.0, 448.5, 449.0, 449.5, 450.0, 450.5, 451.0, 451.5,
    452.0, 452.5, 453.0, 453.5, 454.0, 454.5, 455.0, 455.5, 456.0, 456.5, 457.0, 457.5, 458.0,
    458.5, 459.0, 459.5, 460.0, 460.5, 461.0, 461.5, 462.0, 462.5, 463.0, 463.5, 464.0, 464.5,
    465.0, 465.5, 466.0, 466.5, 467.0, 467.5, 468.0, 468.5, 469.0, 469.5, 470.0, 470.5, 471.0,
    471.5, 472.0, 472.5, 473.0, 473.5, 474.0, 474.5, 475.0, 475.5, 476.0, 476.5, 477.0, 477.5,
    478.0, 478.5, 479.0, 479.5, 480.0, 480.5, 481.0, 481.5, 482.0, 482.5, 483.0, 483.5, 484.0,
    484.5, 485.0, 485.5, 486.0, 486.5, 487.0, 487.5, 488.0, 488.5, 489.0, 489.5, 490.0, 490.5,
    491.0, 491.5, 492.0, 492.5, 493.0, 493.5, 494.0, 494.5, 495.0, 495.5, 496.0, 496.5, 497.0,
    497.5, 498.0, 498.5, 499.0, 499.5, 500.0, 501.0, 502.0, 503.0, 504.0, 505.0, 506.0, 507.0,
    508.0, 509.0, 510.0, 511.0, 512.0, 513.0, 514.0, 515.0, 516.0, 517.0, 518.0, 519.0, 520.0,
    521.0, 522.0, 523.0, 524.0, 525.0, 526.0, 527.0, 528.0, 529.0, 530.0, 531.0, 532.0, 533.0,
    534.0, 535.0, 536.0, 537.0, 538.0, 539.0, 540.0, 541.0, 542.0, 543.0, 544.0, 545.0, 546.0,
    547.0, 548.0, 549.0, 550.0, 551.0, 552.0, 553.0, 554.0, 555.0, 556.0, 557.0, 558.0, 559.0,
    560.0, 561.0, 562.0, 563.0, 564.0, 565.0, 566.0, 567.0, 568.0, 569.0, 570.0, 571.0, 572.0,
    573.0, 574.0, 575.0, 576.0, 577.0, 578.0, 579.0, 580.0, 581.0, 582.0, 583.0, 584.0, 585.0,
    586.0, 587.0, 588.0, 589.0, 590.0, 591.0, 592.0, 593.0, 594.0, 595.0, 596.0, 597.0, 598.0,
    599.0, 600.0, 601.0, 602.0, 603.0, 604.0, 605.0, 606.0, 607.0, 608.0, 609.0, 610.0, 611.0,
    612.0, 613.0, 614.0, 615.0, 616.0, 617.0, 618.0, 619.0, 620.0, 621.0, 622.0, 623.0, 624.0,
    625.0, 626.0, 627.0, 628.0, 629.0, 630.0, 631.0, 632.0, 633.0, 634.0, 635.0, 636.0, 637.0,
    638.0, 639.0, 640.0, 641.0, 642.0, 643.0, 644.0, 645.0, 646.0, 647.0, 648.0, 649.0, 650.0,
    651.0, 652.0, 653.0, 654.0, 655.0, 656.0, 657.0, 658.0, 659.0, 660.0, 661.0, 662.0, 663.0,
    664.0, 665.0, 666.0, 667.0, 668.0, 669.0, 670.0, 671.0, 672.0, 673.0, 674.0, 675.0, 676.0,
    677.0, 678.0, 679.0, 680.0, 681.0, 682.0, 683.0, 684.0, 685.0, 686.0, 687.0, 688.0, 689.0,
    690.0, 691.0, 692.0, 693.0, 694.0, 695.0, 696.0, 697.0, 698.0, 699.0, 700.0, 701.0, 702.0,
    703.0, 704.0, 705.0, 706.0, 707.0, 708.0, 709.0, 710.0, 711.0, 712.0, 713.0, 714.0, 715.0,
    716.0, 717.0, 718.0, 719.0, 720.0, 721.0, 722.0, 723.0, 724.0, 725.0, 726.0, 727.0, 728.0,
    729.0, 730.0, 731.0, 732.0, 733.0, 734.0, 735.0, 736.0, 737.0, 738.0, 739.0, 740.0, 741.0,
    742.0, 743.0, 744.0, 745.0, 746.0, 747.0, 748.0, 749.0, 750.0, 751.0, 752.0, 753.0, 754.0,
    755.0, 756.0, 757.0, 758.0, 759.0, 760.0, 761.0, 762.0, 763.0, 764.0, 765.0, 766.0, 767.0,
    768.0, 769.0, 770.0, 771.0, 772.0, 773.0, 774.0, 775.0, 776.0, 777.0, 778.0, 779.0, 780.0,
    781.0, 782.0, 783.0, 784.0, 785.0, 786.0, 787.0, 788.0, 789.0, 790.0, 791.0, 792.0, 793.0,
    794.0, 795.0, 796.0, 797.0, 798.0, 799.0, 800.0, 801.0, 802.0, 803.0, 804.0, 805.0, 806.0,
    807.0, 808.0, 809.0, 810.0, 811.0, 812.0, 813.0, 814.0, 815.0, 816.0, 817.0, 818.0, 819.0,
    820.0, 821.0, 822.0, 823.0, 824.0, 825.0, 826.0, 827.0, 828.0, 829.0, 830.0, 831.0, 832.0,
    833.0, 834.0, 835.0, 836.0, 837.0, 838.0, 839.0, 840.0, 841.0, 842.0, 843.0, 844.0, 845.0,
    846.0, 847.0, 848.0, 849.0, 850.0, 851.0, 852.0, 853.0, 854.0, 855.0, 856.0, 857.0, 858.0,
    859.0, 860.0, 861.0, 862.0, 863.0, 864.0, 865.0, 866.0, 867.0, 868.0, 869.0, 870.0, 871.0,
    872.0, 873.0, 874.0, 875.0, 876.0, 877.0, 878.0, 879.0, 880.0, 881.0, 882.0, 883.0, 884.0,
    885.0, 886.0, 887.0, 888.0, 889.0, 890.0, 891.0, 892.0, 893.0, 894.0, 895.0, 896.0, 897.0,
    898.0, 899.0, 900.0, 901.0, 902.0, 903.0, 904.0, 905.0, 906.0, 907.0, 908.0, 909.0, 910.0,
    911.0, 912.0, 913.0, 914.0, 915.0, 916.0, 917.0, 918.0, 919.0, 920.0, 921.0, 922.0, 923.0,
    924.0, 925.0, 926.0, 927.0, 928.0, 929.0, 930.0, 931.0, 932.0, 933.0, 934.0, 935.0, 936.0,
    937.0, 938.0, 939.0, 940.0, 941.0, 942.0, 943.0, 944.0, 945.0, 946.0, 947.0, 948.0, 949.0,
    950.0, 951.0, 952.0, 953.0, 954.0, 955.0, 956.0, 957.0, 958.0, 959.0, 960.0, 961.0, 962.0,
    963.0, 964.0, 965.0, 966.0, 967.0, 968.0, 969.0, 970.0, 971.0, 972.0, 973.0, 974.0, 975.0,
    976.0, 977.0, 978.0, 979.0, 980.0, 981.0, 982.0, 983.0, 984.0, 985.0, 986.0, 987.0, 988.0,
    989.0, 990.0, 991.0, 992.0, 993.0, 994.0, 995.0, 996.0, 997.0, 998.0, 999.0, 1000.0, 1005.0,
    1010.0, 1015.0, 1020.0, 1025.0, 1030.0, 1035.0, 1040.0, 1045.0, 1050.0, 1055.0, 1060.0, 1065.0,
    1070.0, 1075.0, 1080.0, 1085.0, 1090.0, 1095.0, 1100.0, 1105.0, 1110.0, 1115.0, 1120.0, 1125.0,
    1130.0, 1135.0, 1140.0, 1145.0, 1150.0, 1155.0, 1160.0, 1165.0, 1170.0, 1175.0, 1180.0, 1185.0,
    1190.0, 1195.0, 1200.0, 1205.0, 1210.0, 1215.0, 1220.0, 1225.0, 1230.0, 1235.0, 1240.0, 1245.0,
    1250.0, 1255.0, 1260.0, 1265.0, 1270.0, 1275.0, 1280.0, 1285.0, 1290.0, 1295.0, 1300.0, 1305.0,
    1310.0, 1315.0, 1320.0, 1325.0, 1330.0, 1335.0, 1340.0, 1345.0, 1350.0, 1355.0, 1360.0, 1365.0,
    1370.0, 1375.0, 1380.0, 1385.0, 1390.0, 1395.0, 1400.0, 1405.0, 1410.0, 1415.0, 1420.0, 1425.0,
    1430.0, 1435.0, 1440.0, 1445.0, 1450.0, 1455.0, 1460.0, 1465.0, 1470.0, 1475.0, 1480.0, 1485.0,
    1490.0, 1495.0, 1500.0, 1505.0, 1510.0, 1515.0, 1520.0, 1525.0, 1530.0, 1535.0, 1540.0, 1545.0,
    1550.0, 1555.0, 1560.0, 1565.0, 1570.0, 1575.0, 1580.0, 1585.0, 1590.0, 1595.0, 1600.0, 1605.0,
    1610.0, 1615.0, 1620.0, 1625.0, 1630.0, 1635.0, 1640.0, 1645.0, 1650.0, 1655.0, 1660.0, 1665.0,
    1670.0, 1675.0, 1680.0, 1685.0, 1690.0, 1695.0, 1700.0, 1705.0, 1710.0, 1715.0, 1720.0, 1725.0,
    1730.0, 1735.0, 1740.0, 1745.0, 1750.0, 1755.0, 1760.0, 1765.0, 1770.0, 1775.0, 1780.0, 1785.0,
    1790.0, 1795.0, 1800.0, 1805.0, 1810.0, 1815.0, 1820.0, 1825.0, 1830.0, 1835.0, 1840.0, 1845.0,
    1850.0, 1855.0, 1860.0, 1865.0, 1870.0, 1875.0, 1880.0, 1885.0, 1890.0, 1895.0, 1900.0, 1905.0,
    1910.0, 1915.0, 1920.0, 1925.0, 1930.0, 1935.0, 1940.0, 1945.0, 1950.0, 1955.0, 1960.0, 1965.0,
    1970.0, 1975.0, 1980.0, 1985.0, 1990.0, 1995.0, 2000.0, 2005.0, 2010.0, 2015.0, 2020.0, 2025.0,
    2030.0, 2035.0, 2040.0, 2045.0, 2050.0, 2055.0, 2060.0, 2065.0, 2070.0, 2075.0, 2080.0, 2085.0,
    2090.0, 2095.0, 2100.0, 2105.0, 2110.0, 2115.0, 2120.0, 2125.0, 2130.0, 2135.0, 2140.0, 2145.0,
    2150.0, 2155.0, 2160.0, 2165.0, 2170.0, 2175.0, 2180.0, 2185.0, 2190.0, 2195.0, 2200.0, 2205.0,
    2210.0, 2215.0, 2220.0, 2225.0, 2230.0, 2235.0, 2240.0, 2245.0, 2250.0, 2255.0, 2260.0, 2265.0,
    2270.0, 2275.0, 2280.0, 2285.0, 2290.0, 2295.0, 2300.0, 2305.0, 2310.0, 2315.0, 2320.0, 2325.0,
    2330.0, 2335.0, 2340.0, 2345.0, 2350.0, 2355.0, 2360.0, 2365.0, 2370.0, 2375.0, 2380.0, 2385.0,
    2390.0, 2395.0, 2400.0, 2405.0, 2410.0, 2415.0, 2420.0, 2425.0, 2430.0, 2435.0, 2440.0, 2445.0,
    2450.0, 2455.0, 2460.0, 2465.0, 2470.0, 2475.0, 2480.0, 2485.0, 2490.0, 2495.0, 2500.0, 2505.0,
    2510.0, 2515.0, 2520.0, 2525.0, 2530.0, 2535.0, 2540.0, 2545.0, 2550.0, 2555.0, 2560.0, 2565.0,
    2570.0, 2575.0, 2580.0, 2585.0, 2590.0, 2595.0, 2600.0, 2605.0, 2610.0, 2615.0, 2620.0, 2625.0,
    2630.0, 2635.0, 2640.0, 2645.0, 2650.0, 2655.0, 2660.0, 2665.0, 2670.0, 2675.0, 2680.0, 2685.0,
    2690.0, 2695.0, 2700.0, 2705.0, 2710.0, 2715.0, 2720.0, 2725.0, 2730.0, 2735.0, 2740.0, 2745.0,
    2750.0, 2755.0, 2760.0, 2765.0, 2770.0, 2775.0, 2780.0, 2785.0, 2790.0, 2795.0, 2800.0, 2805.0,
    2810.0, 2815.0, 2820.0, 2825.0, 2830.0, 2835.0, 2840.0, 2845.0, 2850.0, 2855.0, 2860.0, 2865.0,
    2870.0, 2875.0, 2880.0, 2885.0, 2890.0, 2895.0, 2900.0, 2905.0, 2910.0, 2915.0, 2920.0, 2925.0,
    2930.0, 2935.0, 2940.0, 2945.0, 2950.0, 2955.0, 2960.0, 2965.0, 2970.0, 2975.0, 2980.0, 2985.0,
    2990.0, 2995.0, 3000.0, 3005.0, 3010.0, 3015.0, 3020.0, 3025.0, 3030.0, 3035.0, 3040.0, 3045.0,
    3050.0, 3055.0, 3060.0, 3065.0, 3070.0, 3075.0, 3080.0, 3085.0, 3090.0, 3095.0, 3100.0, 3105.0,
    3110.0, 3115.0, 3120.0, 3125.0, 3130.0, 3135.0, 3140.0, 3145.0, 3150.0, 3155.0, 3160.0, 3165.0,
    3170.0, 3175.0, 3180.0, 3185.0, 3190.0, 3195.0, 3200.0, 3205.0, 3210.0, 3215.0, 3220.0, 3225.0,
    3230.0, 3235.0, 3240.0, 3245.0, 3250.0, 3255.0, 3260.0, 3265.0, 3270.0, 3275.0, 3280.0, 3285.0,
    3290.0, 3295.0, 3300.0, 3305.0, 3310.0, 3315.0, 3320.0, 3325.0, 3330.0, 3335.0, 3340.0, 3345.0,
    3350.0, 3355.0, 3360.0, 3365.0, 3370.0, 3375.0, 3380.0, 3385.0, 3390.0, 3395.0, 3400.0, 3405.0,
    3410.0, 3415.0, 3420.0, 3425.0, 3430.0, 3435.0, 3440.0, 3445.0, 3450.0, 3455.0, 3460.0, 3465.0,
    3470.0, 3475.0, 3480.0, 3485.0, 3490.0, 3495.0, 3500.0, 3505.0, 3510.0, 3515.0, 3520.0, 3525.0,
    3530.0, 3535.0, 3540.0, 3545.0, 3550.0, 3555.0, 3560.0, 3565.0, 3570.0, 3575.0, 3580.0, 3585.0,
    3590.0, 3595.0, 3600.0, 3605.0, 3610.0, 3615.0, 3620.0, 3625.0, 3630.0, 3635.0, 3640.0, 3645.0,
    3650.0, 3655.0, 3660.0, 3665.0, 3670.0, 3675.0, 3680.0, 3685.0, 3690.0, 3695.0, 3700.0, 3705.0,
    3710.0, 3715.0, 3720.0, 3725.0, 3730.0, 3735.0, 3740.0, 3745.0, 3750.0, 3755.0, 3760.0, 3765.0,
    3770.0, 3775.0, 3780.0, 3785.0, 3790.0, 3795.0, 3800.0, 3805.0, 3810.0, 3815.0, 3820.0, 3825.0,
    3830.0, 3835.0, 3840.0, 3845.0, 3850.0, 3855.0, 3860.0, 3865.0, 3870.0, 3875.0, 3880.0, 3885.0,
    3890.0, 3895.0, 3900.0, 3905.0, 3910.0, 3915.0, 3920.0, 3925.0, 3930.0, 3935.0, 3940.0, 3945.0,
    3950.0, 3955.0, 3960.0, 3965.0, 3970.0, 3975.0, 3980.0, 3985.0, 3990.0, 3995.0, 4000.0, 4005.0,
    4010.0, 4015.0, 4020.0, 4025.0, 4030.0, 4035.0, 4040.0, 4045.0, 4050.0, 4055.0, 4060.0, 4065.0,
    4070.0, 4075.0, 4080.0, 4085.0, 4090.0, 4095.0, 4100.0, 4105.0, 4110.0, 4115.0, 4120.0, 4125.0,
    4130.0, 4135.0, 4140.0, 4145.0, 4150.0, 4155.0, 4160.0, 4165.0, 4170.0, 4175.0, 4180.0, 4185.0,
    4190.0, 4195.0, 4200.0, 4205.0, 4210.0, 4215.0, 4220.0, 4225.0, 4230.0, 4235.0, 4240.0, 4245.0,
    4250.0, 4255.0, 4260.0, 4265.0, 4270.0, 4275.0, 4280.0, 4285.0, 4290.0, 4295.0, 4300.0, 4305.0,
    4310.0, 4315.0, 4320.0, 4325.0, 4330.0, 4335.0, 4340.0, 4345.0, 4350.0, 4355.0, 4360.0, 4365.0,
    4370.0, 4375.0, 4380.0, 4385.0, 4390.0, 4395.0, 4400.0, 4405.0, 4410.0, 4415.0, 4420.0, 4425.0,
    4430.0, 4435.0, 4440.0, 4445.0, 4450.0, 4455.0, 4460.0, 4465.0, 4470.0, 4475.0, 4480.0, 4485.0,
    4490.0, 4495.0, 4500.0, 4505.0, 4510.0, 4515.0, 4520.0, 4525.0, 4530.0, 4535.0, 4540.0, 4545.0,
    4550.0, 4555.0, 4560.0, 4565.0, 4570.0, 4575.0, 4580.0, 4585.0, 4590.0, 4595.0, 4600.0, 4605.0,
    4610.0, 4615.0, 4620.0, 4625.0, 4630.0, 4635.0, 4640.0, 4645.0, 4650.0, 4655.0, 4660.0, 4665.0,
    4670.0, 4675.0, 4680.0, 4685.0, 4690.0, 4695.0, 4700.0, 4705.0, 4710.0, 4715.0, 4720.0, 4725.0,
    4730.0, 4735.0, 4740.0, 4745.0, 4750.0, 4755.0, 4760.0, 4765.0, 4770.0, 4775.0, 4780.0, 4785.0,
    4790.0, 4795.0, 4800.0, 4805.0, 4810.0, 4815.0, 4820.0, 4825.0, 4830.0, 4835.0, 4840.0, 4845.0,
    4850.0, 4855.0, 4860.0, 4865.0, 4870.0, 4875.0, 4880.0, 4885.0, 4890.0, 4895.0, 4900.0, 4905.0,
    4910.0, 4915.0, 4920.0, 4925.0, 4930.0, 4935.0, 4940.0, 4945.0, 4950.0, 4955.0, 4960.0, 4965.0,
    4970.0, 4975.0, 4980.0, 4985.0, 4990.0, 4995.0, 5000.0, 5005.0, 5010.0, 5015.0, 5020.0, 5025.0,
    5030.0, 5035.0, 5040.0, 5045.0, 5050.0, 5055.0, 5060.0, 5065.0, 5070.0, 5075.0, 5080.0, 5085.0,
    5090.0, 5095.0, 5100.0, 5105.0, 5110.0, 5115.0, 5120.0, 5125.0, 5130.0, 5135.0, 5140.0, 5145.0,
    5150.0, 5155.0, 5160.0, 5165.0, 5170.0, 5175.0, 5180.0, 5185.0, 5190.0, 5195.0, 5200.0, 5205.0,
    5210.0, 5215.0, 5220.0, 5225.0, 5230.0, 5235.0, 5240.0, 5245.0, 5250.0, 5255.0, 5260.0, 5265.0,
    5270.0, 5275.0, 5280.0, 5285.0, 5290.0, 5295.0, 5300.0, 5305.0, 5310.0, 5315.0, 5320.0, 5325.0,
    5330.0, 5335.0, 5340.0, 5345.0, 5350.0, 5355.0, 5360.0, 5365.0, 5370.0, 5375.0, 5380.0, 5385.0,
    5390.0, 5395.0, 5400.0, 5405.0, 5410.0, 5415.0, 5420.0, 5425.0, 5430.0, 5435.0, 5440.0, 5445.0,
    5450.0, 5455.0, 5460.0, 5465.0, 5470.0, 5475.0, 5480.0, 5485.0, 5490.0, 5495.0, 5500.0, 5505.0,
    5510.0, 5515.0, 5520.0, 5525.0, 5530.0, 5535.0, 5540.0, 5545.0, 5550.0, 5555.0, 5560.0, 5565.0,
    5570.0, 5575.0, 5580.0, 5585.0, 5590.0, 5595.0, 5600.0, 5605.0, 5610.0, 5615.0, 5620.0, 5625.0,
    5630.0, 5635.0, 5640.0, 5645.0, 5650.0, 5655.0, 5660.0, 5665.0, 5670.0, 5675.0, 5680.0, 5685.0,
    5690.0, 5695.0, 5700.0, 5705.0, 5710.0, 5715.0, 5720.0, 5725.0, 5730.0, 5735.0, 5740.0, 5745.0,
    5750.0, 5755.0, 5760.0, 5765.0, 5770.0, 5775.0, 5780.0, 5785.0, 5790.0, 5795.0, 5800.0, 5805.0,
    5810.0, 5815.0, 5820.0, 5825.0, 5830.0, 5835.0, 5840.0, 5845.0, 5850.0, 5855.0, 5860.0, 5865.0,
    5870.0, 5875.0, 5880.0, 5885.0, 5890.0, 5895.0, 5900.0, 5905.0, 5910.0, 5915.0, 5920.0, 5925.0,
    5930.0, 5935.0, 5940.0, 5945.0, 5950.0, 5955.0, 5960.0, 5965.0, 5970.0, 5975.0, 5980.0, 5985.0,
    5990.0, 5995.0, 6000.0, 6005.0, 6010.0, 6015.0, 6020.0, 6025.0, 6030.0, 6035.0, 6040.0, 6045.0,
    6050.0, 6055.0, 6060.0, 6065.0, 6070.0, 6075.0, 6080.0, 6085.0, 6090.0, 6095.0, 6100.0, 6105.0,
    6110.0, 6115.0, 6120.0, 6125.0, 6130.0, 6135.0, 6140.0, 6145.0, 6150.0, 6155.0, 6160.0, 6165.0,
    6170.0, 6175.0, 6180.0, 6185.0, 6190.0, 6195.0, 6200.0, 6205.0, 6210.0, 6215.0, 6220.0, 6225.0,
    6230.0, 6235.0, 6240.0, 6245.0, 6250.0, 6255.0, 6260.0, 6265.0, 6270.0, 6275.0, 6280.0, 6285.0,
    6290.0, 6295.0, 6300.0, 6305.0, 6310.0, 6315.0, 6320.0, 6325.0, 6330.0, 6335.0, 6340.0, 6345.0,
    6350.0, 6355.0, 6360.0, 6365.0, 6370.0, 6375.0, 6380.0, 6385.0, 6390.0, 6395.0, 6400.0, 6405.0,
    6410.0, 6415.0, 6420.0, 6425.0, 6430.0, 6435.0, 6440.0, 6445.0, 6450.0, 6455.0, 6460.0, 6465.0,
    6470.0, 6475.0, 6480.0, 6485.0, 6490.0, 6495.0, 6500.0, 6505.0, 6510.0, 6515.0, 6520.0, 6525.0,
    6530.0, 6535.0, 6540.0, 6545.0, 6550.0, 6555.0, 6560.0, 6565.0, 6570.0, 6575.0, 6580.0, 6585.0,
    6590.0, 6595.0, 6600.0, 6605.0, 6610.0, 6615.0, 6620.0, 6625.0, 6630.0, 6635.0, 6640.0, 6645.0,
    6650.0, 6655.0, 6660.0, 6665.0, 6670.0, 6675.0, 6680.0, 6685.0, 6690.0, 6695.0, 6700.0, 6705.0,
    6710.0, 6715.0, 6720.0, 6725.0, 6730.0, 6735.0, 6740.0, 6745.0, 6750.0, 6755.0, 6760.0, 6765.0,
    6770.0, 6775.0, 6780.0, 6785.0, 6790.0, 6795.0, 6800.0, 6805.0, 6810.0, 6815.0, 6820.0, 6825.0,
    6830.0, 6835.0, 6840.0, 6845.0, 6850.0, 6855.0, 6860.0, 6865.0, 6870.0, 6875.0, 6880.0, 6885.0,
    6890.0, 6895.0, 6900.0, 6905.0, 6910.0, 6915.0, 6920.0, 6925.0, 6930.0, 6935.0, 6940.0, 6945.0,
    6950.0, 6955.0, 6960.0, 6965.0, 6970.0, 6975.0, 6980.0, 6985.0, 6990.0, 6995.0, 7000.0, 7005.0,
    7010.0, 7015.0, 7020.0, 7025.0, 7030.0, 7035.0, 7040.0, 7045.0, 7050.0, 7055.0, 7060.0, 7065.0,
    7070.0, 7075.0, 7080.0, 7085.0, 7090.0, 7095.0, 7100.0, 7105.0, 7110.0, 7115.0, 7120.0, 7125.0,
    7130.0, 7135.0, 7140.0, 7145.0, 7150.0, 7155.0, 7160.0, 7165.0, 7170.0, 7175.0, 7180.0, 7185.0,
    7190.0, 7195.0, 7200.0, 7205.0, 7210.0, 7215.0, 7220.0, 7225.0, 7230.0, 7235.0, 7240.0, 7245.0,
    7250.0, 7255.0, 7260.0, 7265.0, 7270.0, 7275.0, 7280.0, 7285.0, 7290.0, 7295.0, 7300.0, 7305.0,
    7310.0, 7315.0, 7320.0, 7325.0, 7330.0, 7335.0, 7340.0, 7345.0, 7350.0, 7355.0, 7360.0, 7365.0,
    7370.0, 7375.0, 7380.0, 7385.0, 7390.0, 7395.0, 7400.0, 7405.0, 7410.0, 7415.0, 7420.0, 7425.0,
    7430.0, 7435.0, 7440.0, 7445.0, 7450.0, 7455.0, 7460.0, 7465.0, 7470.0, 7475.0, 7480.0, 7485.0,
    7490.0, 7495.0, 7500.0, 7505.0, 7510.0, 7515.0, 7520.0, 7525.0, 7530.0, 7535.0, 7540.0, 7545.0,
    7550.0, 7555.0, 7560.0, 7565.0, 7570.0, 7575.0, 7580.0, 7585.0, 7590.0, 7595.0, 7600.0, 7605.0,
    7610.0, 7615.0, 7620.0, 7625.0, 7630.0, 7635.0, 7640.0, 7645.0, 7650.0, 7655.0, 7660.0, 7665.0,
    7670.0, 7675.0, 7680.0, 7685.0, 7690.0, 7695.0, 7700.0, 7705.0, 7710.0, 7715.0, 7720.0, 7725.0,
    7730.0, 7735.0, 7740.0, 7745.0, 7750.0, 7755.0, 7760.0, 7765.0, 7770.0, 7775.0, 7780.0, 7785.0,
    7790.0, 7795.0, 7800.0, 7805.0, 7810.0, 7815.0, 7820.0, 7825.0, 7830.0, 7835.0, 7840.0, 7845.0,
    7850.0, 7855.0, 7860.0, 7865.0, 7870.0, 7875.0, 7880.0, 7885.0, 7890.0, 7895.0, 7900.0, 7905.0,
    7910.0, 7915.0, 7920.0, 7925.0, 7930.0, 7935.0, 7940.0, 7945.0, 7950.0, 7955.0, 7960.0, 7965.0,
    7970.0, 7975.0, 7980.0, 7985.0, 7990.0, 7995.0,
];

lazy_static! {
    static ref IXMAP: HashMap<i64, i64> = {
        let mut ixmap: HashMap<i64, i64> = HashMap::new();
        for (i, v) in IX.iter().enumerate() {
            ixmap.insert((v * 100.) as i64, i as i64);
        }
        ixmap
    };
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn get_price_tick_move(a: f64, b: i64) -> f64 {
    let idx: &i64 = IXMAP.get(&((a * 100.) as i64)).unwrap();
    IX[(idx + b) as usize]
}

#[pyfunction]
fn get_price_between_tick(a: f64, b: f64) -> i64 {
    let a_idx: &i64 = IXMAP.get(&((a * 100.) as i64)).unwrap();
    let b_idx: &i64 = IXMAP.get(&((b * 100.) as i64)).unwrap();
    b_idx - a_idx
}

#[pyfunction]
fn get_price_between_ticks(a: f64, b: f64) -> Vec<f64> {
    let a_idx: i64 = *IXMAP.get(&((a * 100.) as i64)).unwrap();
    let b_idx: i64 = *IXMAP.get(&((b * 100.) as i64)).unwrap();
    if b_idx > a_idx {
        IX[a_idx as usize..=b_idx as usize].to_vec()
    } else {
        let mut v = IX[b_idx as usize..=a_idx as usize].to_vec();
        v.reverse();
        v
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn rs2py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_price_tick_move, m)?)?;
    m.add_function(wrap_pyfunction!(get_price_between_tick, m)?)?;
    m.add_function(wrap_pyfunction!(get_price_between_ticks, m)?)?;
    Ok(())
}
