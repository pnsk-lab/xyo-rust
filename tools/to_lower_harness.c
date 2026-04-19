#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

struct xyo_string_struct
{
    uint64_t length;
    uint16_t *data;
    uint64_t hash_1;
    uint64_t hash_2;
};

bool string_to_bool(const struct xyo_string_struct *input);

static struct xyo_string_struct make_ascii_string(const char *ascii)
{
    size_t len = 0;
    while (ascii[len] != '\0')
    {
        ++len;
    }

    uint16_t *buf = malloc(len * sizeof(uint16_t));
    if (buf == NULL)
    {
        fprintf(stderr, "malloc failed\n");
        exit(1);
    }

    for (size_t i = 0; i < len; ++i)
    {
        buf[i] = (uint8_t)ascii[i];
    }

    struct xyo_string_struct s = {
        .length = len,
        .data = buf,
        .hash_1 = 0,
        .hash_2 = 0,
    };
    return s;
}

static void free_string(struct xyo_string_struct *s)
{
    free(s->data);
    s->data = NULL;
}

static int run_case(const char *input, bool expected)
{
    struct xyo_string_struct s = make_ascii_string(input);
    bool actual = string_to_bool(&s);
    printf("input=\"%s\" actual=%s expected=%s\n",
           input,
           actual ? "true" : "false",
           expected ? "true" : "false");
    free_string(&s);
    return actual == expected ? 0 : 1;
}

int main(void)
{
    int failures = 0;
    failures += run_case("", false);
    failures += run_case("0", false);
    failures += run_case("false", false);
    failures += run_case("FALSE", false);
    failures += run_case("true", true);
    failures += run_case("TrUe", true);
    failures += run_case("1", true);
    return failures == 0 ? 0 : 1;
}
