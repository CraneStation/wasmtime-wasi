use crate::host::{fd_table, fd_table_init, fd_table_insert_existing};
use cranelift_codegen::ir::types;
use cranelift_codegen::{ir, isa};
use cranelift_entity::PrimaryMap;
use cranelift_wasm::DefinedFuncIndex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;
use syscalls;
use target_lexicon::HOST;
use wasmtime_environ::{translate_signature, Export, Module};
use wasmtime_runtime::{Imports, Instance, InstantiationError, VMFunctionBody};

pub(crate) struct WASIState {
    pub curfds: Box<fd_table>,
}

/// Return an instance implementing the "wasi" interface.
pub fn instantiate_wasi(
    prefix: &str,
    global_exports: Rc<RefCell<HashMap<String, Option<wasmtime_runtime::Export>>>>,
) -> Result<Instance, InstantiationError> {
    let pointer_type = types::Type::triple_pointer_type(&HOST);
    let mut module = Module::new();
    let mut finished_functions: PrimaryMap<DefinedFuncIndex, *const VMFunctionBody> =
        PrimaryMap::new();
    let call_conv = isa::CallConv::triple_default(&HOST);

    macro_rules! signature {
        ( $name:ident ( $( $args:ident ),* ) -> ( $( $rets:ident ),* ) ) => {
            {
                use cranelift_codegen::ir::types::*;
                let sig = module.signatures.push(translate_signature(
                    ir::Signature {
                        params: vec![$( ir::AbiParam::new($args) ),*],
                        returns: vec![$( ir::AbiParam::new($rets) ),*],
                        call_conv,
                    },
                    pointer_type,
                ));
                let func = module.functions.push(sig);
                module
                    .exports
                    .insert(prefix.to_owned() + stringify!($name), Export::Function(func));
                finished_functions.push(syscalls::$name as *const VMFunctionBody);
            }
        }
    }

    // TODO: It'd be even cooler if we could infer the signatures automatically.
    signature!(clock_res_get(I32, I32) -> (I32));
    signature!(clock_time_get(I32, I64, I32) -> (I32));
    signature!(fd_close(I32) -> (I32));
    signature!(fd_create1(I32, I32) -> (I32));
    signature!(fd_create2(I32, I32, I32) -> (I32));
    signature!(fd_datasync(I32) -> (I32));
    signature!(fd_dup(I32, I32) -> (I32));
    signature!(fd_pread(I32, I32, I32, I64, I32) -> (I32));
    signature!(fd_pwrite(I32, I32, I32, I64, I32) -> (I32));
    signature!(fd_read(I32, I32, I32, I32) -> (I32));
    signature!(fd_replace(I32, I32) -> (I32));
    signature!(fd_seek(I32, I64, I32, I32) -> (I32));
    signature!(fd_stat_get(I32, I32) -> (I32));
    signature!(fd_stat_put(I32, I32, I32) -> (I32));
    signature!(fd_sync(I32) -> (I32));
    signature!(fd_write(I32, I32, I32, I32) -> (I32));
    signature!(file_advise(I32, I64, I32, I32) -> (I32));
    signature!(file_allocate(I32, I64, I32) -> (I32));
    signature!(file_create(I32, I32, I32, I32) -> (I32));
    signature!(file_link(I32, I32, I32, I32, I32, I32) -> (I32));
    signature!(file_open(I32, I32, I32, I32, I32, I32) -> (I32));
    signature!(file_readdir(I32, I32, I32, I64, I32) -> (I32));
    signature!(file_readlink(I32, I32, I32, I32, I32, I32) -> (I32));
    signature!(file_rename(I32, I32, I32, I32, I32, I32) -> (I32));
    signature!(file_stat_fget(I32, I32) -> (I32));
    signature!(file_stat_fput(I32, I32, I32) -> (I32));
    signature!(file_stat_get(I32, I32, I32, I32) -> (I32));
    signature!(file_stat_put(I32, I32, I32, I32, I32) -> (I32));
    signature!(file_symlink(I32, I32, I32, I32, I32) -> (I32));
    signature!(file_unlink(I32, I32, I32, I32) -> (I32));
    signature!(mem_advise(I32, I32, I32) -> (I32));
    signature!(mem_map(I32, I32, I32, I32, I32, I64, I32) -> (I32));
    signature!(mem_protect(I32, I32, I32) -> (I32));
    signature!(mem_sync(I32, I32, I32) -> (I32));
    signature!(mem_unmap(I32, I32) -> (I32));
    signature!(poll(I32, I32, I32, I32) -> (I32));
    signature!(proc_exit(I32) -> ());
    signature!(proc_raise(I32) -> (I32));
    signature!(random_get(I32, I32) -> (I32));
    signature!(sched_yield() -> (I32));
    signature!(sock_recv(I32, I32, I32) -> (I32));
    signature!(sock_send(I32, I32, I32) -> (I32));
    signature!(sock_shutdown(I32, I32) -> (I32));

    let imports = Imports::none();
    let data_initializers = Vec::new();
    let signatures = PrimaryMap::new();
    let mut curfds = Box::new(unsafe { mem::zeroed::<fd_table>() });

    unsafe {
        let curfds: *mut fd_table = &mut *curfds;
        fd_table_init(curfds);

        // Prepopulate curfds with stdin, stdout, and stderr file descriptors.
        assert!(fd_table_insert_existing(curfds, 0, 0));
        assert!(fd_table_insert_existing(curfds, 1, 1));
        assert!(fd_table_insert_existing(curfds, 2, 2));
    }
    let host_state = WASIState { curfds };

    Instance::new(
        Rc::new(module),
        global_exports,
        finished_functions.into_boxed_slice(),
        imports,
        &data_initializers,
        signatures.into_boxed_slice(),
        Box::new(host_state),
    )
}
