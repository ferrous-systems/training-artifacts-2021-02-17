#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

intptr_t parse_and_eval(const char *maybe_cstr, int64_t *output);

} // extern "C"
