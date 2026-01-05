#ifndef STRING_UTILS
#define STRING_UTILS
#include <stdint.h>
#include <stddef.h>

// Reverse a string in-place
void c_string_reverse(char *str, size_t len);

// Check if string is palindrome
int c_is_palindrome(const char *str, size_t len);

// Count character occurrences
size_t c_count_char(const char *str, size_t len, char ch);

// Convert string to uppercase in-place
void c_to_uppercase(char *str, size_t len);

// Convert string to lowercase in-place
void c_to_lowercase(char *str, size_t len);

#endif
