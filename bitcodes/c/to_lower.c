#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>
#include <unicode/ucasemap.h>
#include <unicode/ustring.h>

struct xyo_string_struct
{
    uint64_t length;
    uint16_t *data;
    uint64_t hash_1;
    uint64_t hash_2;
};

bool string_to_bool(const struct xyo_string_struct *input)
{
    UErrorCode status = U_ZERO_ERROR;
    UCaseMap *csm = ucasemap_open("", 0, &status);

    const UChar *src = (const UChar *)input->data;
    UChar dest[32];
    int32_t len = u_strToLower(
        dest,
        32,
        src,
        input->length,
        NULL,
        &status);

    ucasemap_close(csm);

    if (U_FAILURE(status))
        return true;

    char out[64];
    u_austrcpy(out, dest);
    out[len] = '\0';

    if (out[0] == '\0')
        return false;
    if (strcmp(out, "0") == 0)
        return false;
    if (strcmp(out, "false") == 0)
        return false;

    return true;
}
