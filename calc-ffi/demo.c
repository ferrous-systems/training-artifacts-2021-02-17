#include "stdint.h"
#include "calc_ffi.h"
#include "stdio.h"

// Doesn't work
// `gcc -L. -l:libcalc_ffi.a demo.c -o ./demo`
// Does work (modulo linking)
// `gcc ./demo.c -L. -l:libcalc_ffi.a -o demo`

int main() {
    char* text = "3 3 3 3 3 sqr";
    int64_t output = 0;

    printf("Evaluating: '%s'\n", text);

    intptr_t retcode = parse_and_eval((char *)0xCAFECAFECAFECAFE, &output);

    if(!retcode) {
        printf("Result: %lli\n", output);
    } else {
        printf("FAILED: error code: %lli\n", retcode);
    }

    return 0;
}

// -L. -ltestlib
