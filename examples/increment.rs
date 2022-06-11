use std::env;
use gnulightning_sys::{_jit_arg, _jit_arg_d, _jit_clear_state, _jit_destroy_state, _jit_emit, _jit_getarg_l, _jit_new_node, _jit_new_node_www, _jit_prolog, _jit_retr, finish_jit, init_jit, jit_code_t_jit_code_addi, jit_code_t_jit_code_addi_d, jit_code_t_jit_code_prolog, jit_gpr_t, jit_new_state, jit_node_t, jit_reg_t__RAX, jit_state_t, jit_word_t};

fn main() {
    let argv: Vec<String> = env::args().collect();
    let arg0 = argv[0].as_ptr() as *const std::os::raw::c_char; // if we panic here at the [] it would be ironic given what follows
    unsafe {
        init_jit(arg0);
        let jit: *mut jit_state_t = jit_new_state();
        _jit_prolog(jit); // Function prolog
        let node_in: *mut jit_node_t = _jit_arg(jit); // Initialize a node to hold arg0;

        // Load arg0 into node. (_l for long as in long)
        _jit_getarg_l(jit, jit_reg_t__RAX as jit_gpr_t, node_in);

        // _www is for word word word as opposed to _wwf for word word float or any other combination of args
        _jit_new_node_www(
            jit,
            jit_code_t_jit_code_addi,  // Create a new add immediate instruction
            jit_reg_t__RAX as jit_word_t,  // Source is rax
            jit_reg_t__RAX as jit_word_t,  // Dest is rax
            1); // immediate is 1
        _jit_retr(jit, jit_reg_t__RAX as jit_gpr_t); // Return RAX

        // jit_emit gives us a void pointer which rust will not call (it points to the jit'ed function)
        let void_func_ptr = _jit_emit(jit);
        // transmute it into an extern "C" fn(i32) -> i32; so we can call it
        let jit_compiled_incr: extern "C" fn(i32) -> i32 = std::mem::transmute(void_func_ptr);

        println!("5 + 1 = {}", jit_compiled_incr(5));
    }
}
