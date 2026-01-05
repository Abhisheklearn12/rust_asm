#include "string_utils.h"

void c_string_reverse(char *str, size_t len) {
    if (len <= 1) return;

    for (size_t i = 0; i < len / 2; i++) {
        char temp = str[i];
        str[i] = str[len - 1 - i];
        str[len - 1 - i] = temp;
    }
}

int c_is_palindrome(const char *str, size_t len) {
    if (len <= 1) return 1;

    for (size_t i = 0; i < len / 2; i++) {
        if (str[i] != str[len - 1 - i]) {
            return 0;
        }
    }
    return 1;
}

size_t c_count_char(const char *str, size_t len, char ch) {
    size_t count = 0;
    for (size_t i = 0; i < len; i++) {
        if (str[i] == ch) {
            count++;
        }
    }
    return count;
}

void c_to_uppercase(char *str, size_t len) {
    for (size_t i = 0; i < len; i++) {
        if (str[i] >= 'a' && str[i] <= 'z') {
            str[i] = str[i] - 'a' + 'A';
        }
    }
}

void c_to_lowercase(char *str, size_t len) {
    for (size_t i = 0; i < len; i++) {
        if (str[i] >= 'A' && str[i] <= 'Z') {
            str[i] = str[i] - 'A' + 'a';
        }
    }
}
