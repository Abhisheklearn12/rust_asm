#ifndef ARRAY_OPS
#define ARRAY_OPS
#include <stdint.h>
#include <stddef.h>

// Sum of array elements
int64_t c_array_sum(const int64_t *arr, size_t len);

// Find maximum element
int64_t c_array_max(const int64_t *arr, size_t len);

// Find minimum element
int64_t c_array_min(const int64_t *arr, size_t len);

// Calculate average
double c_array_average(const int64_t *arr, size_t len);

// Binary search (array must be sorted)
int c_binary_search(const int64_t *arr, size_t len, int64_t target);

// Bubble sort (ascending)
void c_bubble_sort(int64_t *arr, size_t len);

#endif
