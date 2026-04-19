#include <stdio.h>
#include "./lib/dtoa.c"
#include <stdint.h>

struct xyo_string_struct
{
    uint64_t length;
    uint16_t *data;
    uint64_t hash_1;
    uint64_t hash_2;
};

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