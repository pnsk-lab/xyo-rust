#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <stdio.h>
#include "unicode/ucasemap.h"
#include "unicode/uchar.h"
#include "unicode/utf16.h"
#include "unicode/ustring.h"

struct xyo_string_struct
{
    uint64_t length;
    uint16_t *data;
    uint64_t hash_1;
    uint64_t hash_2;
};

bool str_to_bool(const struct xyo_string_struct *state, const struct xyo_string_struct *input)
{
    if (input == NULL || input->data == NULL)
        return false;

    const UChar *s = (const UChar *)input->data;
    int32_t len = (int32_t)input->length;

    while (len > 0 && u_isUWhiteSpace(s[0]))
    {
        s++;
        len--;
    }
    while (len > 0 && u_isUWhiteSpace(s[len - 1]))
    {
        len--;
    }

    if (len == 0)
        return false;

    if (len == 1 && s[0] == (UChar)'0')
        return false;

    static const UChar false_word[] = {'f', 'a', 'l', 's', 'e', 0};
    UErrorCode status = U_ZERO_ERROR;
    UChar lowered[16];
    int32_t out_len = u_strToLower(lowered, 16, s, len, "", &status);
    if (U_FAILURE(status) || out_len != 5)
        return true;

    if (u_strncmp(lowered, false_word, 5) == 0)
        return false;

    return true;
}

static int32_t str_cmp_lowered(const struct xyo_string_struct *a, const struct xyo_string_struct *b)
{
    const UChar *a_data = (const UChar *)a->data;
    const UChar *b_data = (const UChar *)b->data;
    int32_t a_index = 0;
    int32_t b_index = 0;
    int32_t a_length = (int32_t)a->length;
    int32_t b_length = (int32_t)b->length;

    while (a_index < a_length && b_index < b_length)
    {
        UChar32 a_cp;
        UChar32 b_cp;

        U16_NEXT(a_data, a_index, a_length, a_cp);
        U16_NEXT(b_data, b_index, b_length, b_cp);

        a_cp = u_tolower(a_cp);
        b_cp = u_tolower(b_cp);

        if (a_cp < b_cp)
            return -1;
        if (a_cp > b_cp)
            return 1;
    }

    if (a_index < a_length)
        return 1;
    if (b_index < b_length)
        return -1;

    return 0;
}

bool str_cmp_gt(const struct xyo_string_struct *a, const struct xyo_string_struct *b)
{
    return str_cmp_lowered(a, b) > 0;
}

bool str_cmp_lt(const struct xyo_string_struct *a, const struct xyo_string_struct *b)
{
    return str_cmp_lowered(a, b) < 0;
}

bool str_cmp_eq(const struct xyo_string_struct *a, const struct xyo_string_struct *b)
{
    // printf("cmp_eq");
    return str_cmp_lowered(a, b) == 0;
}
