#include <math.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include "lib/dtoa.c"
#include "lib/cutils.c"

struct xyo_string_struct
{
    uint64_t length;
    uint16_t *data;
    uint64_t hash_1;
    uint64_t hash_2;
};

static bool is_ecma_ws_or_lt_u16(uint16_t c)
{
    switch (c)
    {
    case 0x0009: // TAB
    case 0x000A: // LF
    case 0x000B: // VT
    case 0x000C: // FF
    case 0x000D: // CR
    case 0x0020: // SP
    case 0x00A0: // NBSP
    case 0x1680: // OGHAM SPACE MARK
    case 0x2000: // EN QUAD
    case 0x2001:
    case 0x2002:
    case 0x2003:
    case 0x2004:
    case 0x2005:
    case 0x2006:
    case 0x2007:
    case 0x2008:
    case 0x2009:
    case 0x200A: // HAIR SPACE
    case 0x2028: // LINE SEPARATOR
    case 0x2029: // PARAGRAPH SEPARATOR
    case 0x202F: // NARROW NBSP
    case 0x205F: // MEDIUM MATHEMATICAL SPACE
    case 0x3000: // IDEOGRAPHIC SPACE
    case 0xFEFF: // BOM
        return true;
    default:
        return false;
    }
}

double xyo_atod(const struct xyo_string_struct *input)
{
    if (input == NULL || input->data == NULL)
    {
        return NAN;
    }
    uint64_t left = 0;
    uint64_t right = input->length;
    while (left < right && is_ecma_ws_or_lt_u16(input->data[left]))
    {
        ++left;
    }
    while (right > left && is_ecma_ws_or_lt_u16(input->data[right - 1]))
    {
        --right;
    }
    if (left == right)
    {
        return 0.0;
    }
    uint64_t len = right - left;
    char buf[len + 1];
    for (uint64_t i = 0; i < len; ++i)
    {
        uint16_t c = input->data[left + i];
        if (c == '_' || c == 'n')
        {
            return NAN;
        }
        if (c > 0x7F)
        {
            return NAN;
        }
        buf[i] = (char)c;
    }
    buf[len] = '\0';
    const char *end = NULL;
    JSATODTempMem tmp = {0};
    double x = js_atod(buf, &end, 0, JS_ATOD_ACCEPT_BIN_OCT, &tmp);
    if (end == NULL || *end != '\0')
    {
        return NAN;
    }
    return x;
}

struct xyo_string_struct *xyo_dtoa(double n, uint64_t hash_seed_1, uint64_t hash_base_1, uint64_t hash_seed_2, uint64_t hash_base_2)
{
    char buf[1024];
    JSDTOATempMem mem_dtoa;
    JSATODTempMem mem_atod;

    uint64_t length = js_dtoa(buf, n, 10, 0, 0, &mem_dtoa);
    struct xyo_string_struct *result = malloc(sizeof(struct xyo_string_struct) + length * sizeof(uint16_t));
    result->length = length;
    result->data = (uint16_t *)(result + 1);
    __uint128_t hash_1 = 0;
    __uint128_t hash_2 = 0;
    for (uint64_t i = 0; i < length; i++)
    {
        uint16_t c = (uint16_t)buf[i];
        result->data[i] = c;
        hash_1 = (hash_1 * hash_base_1 + c) % hash_seed_1;
        hash_2 = (hash_2 * hash_base_2 + c) % hash_seed_2;
    }
    result->hash_1 = (uint64_t)hash_1;
    result->hash_2 = (uint64_t)hash_2;

    return result;
}
bool str_is_num(struct xyo_string_struct *from)
{
    for (int i = 0; i < from->length; ++i)
    {
        if (from->data[i] == '.')
        {
            return true;
        }
    }
    return false;
}
