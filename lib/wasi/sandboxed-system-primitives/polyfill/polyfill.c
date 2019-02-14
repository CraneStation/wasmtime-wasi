#include <emscripten.h>
#include "wasmtime_ssp.h"
#include "../src/posix.h"

static __thread struct fd_table curfds_pointee;

int main(int argc, char *argv[]) {
    struct fd_table *curfds = &curfds_pointee;

    fd_table_init(curfds);

    // Prepopulate curfds with stdin, stdout, and stderr file descriptors.
    if (!fd_table_insert_existing(curfds, 0, 0))
        __builtin_trap();
    if (!fd_table_insert_existing(curfds, 1, 1))
        __builtin_trap();
    if (!fd_table_insert_existing(curfds, 2, 2))
        __builtin_trap();

    EM_ASM(" \
console.log('fetching program.wasm...'); \
fetch('program.wasm').then(function(response) { \
    response.arrayBuffer().then(function(buffer) { \
        WebAssembly.compile(buffer).then(function(module) { \
            const imports = { wasi: WASIPolyfill }; \
            console.log('instantiating...'); \
            WebAssembly.instantiate(module, imports).then(function(instance) { \
                updateGuestBuffer(instance.exports.memory.buffer); \
                updateGuestBufferViews(); \
                console.log('running...'); \
                try { \
                    instance.exports._start(); \
                } catch (e) { \
                    if (e instanceof WASIExit) { \
                        handleWASIExit(e); \
                    } else { \
                        console.log(e.name + ': ' + e.message); \
                    } \
                } \
            }); \
        }); \
    }); \
}); \
");
    return 0;
}
