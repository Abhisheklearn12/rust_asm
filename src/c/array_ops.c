#include "array_ops.h"

int64_t c_array_sum(const int64_t *arr, size_t len) {
    int64_t sum = 0;
    for (size_t i = 0; i < len; i++) {
        sum += arr[i];
    }
    return sum;
}

int64_t c_array_max(const int64_t *arr, size_t len) {
    if (len == 0) return 0;

    int64_t max = arr[0];
    for (size_t i = 1; i < len; i++) {
        if (arr[i] > max) {
            max = arr[i];
        }
    }
    return max;
}

int64_t c_array_min(const int64_t *arr, size_t len) {
    if (len == 0) return 0;

    int64_t min = arr[0];
    for (size_t i = 1; i < len; i++) {
        if (arr[i] < min) {
            min = arr[i];
        }
    }
    return min;
}

double c_array_average(const int64_t *arr, size_t len) {
    if (len == 0) return 0.0;

    int64_t sum = c_array_sum(arr, len);
    return (double)sum / (double)len;
}

int c_binary_search(const int64_t *arr, size_t len, int64_t target) {
    int left = 0;
    int right = len - 1;

    while (left <= right) {
        int mid = left + (right - left) / 2;

        if (arr[mid] == target) {
            return mid;
        }

        if (arr[mid] < target) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return -1;  // Not found
}

void c_bubble_sort(int64_t *arr, size_t len) {
    for (size_t i = 0; i < len - 1; i++) {
        for (size_t j = 0; j < len - i - 1; j++) {
            if (arr[j] > arr[j + 1]) {
                int64_t temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}
