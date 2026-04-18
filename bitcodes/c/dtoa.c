#include "./lib/dtoa.c"

char* xyo_dtoa(double n){
    char buf[1024];
    JSDTOATempMem mem_dtoa;
    JSATODTempMem mem_atod;

    js_dtoa(buf, n, 10, 0, 0, &mem_dtoa);
    return buf;
}
