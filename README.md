# gnulighting-sys

Compiles gnu-lightning 2.3.0 and makes it available via this crate. There are quite a few
irrelevant bindings included so definitely use the [gnu lightning manual](https://www.gnu.org/software/lightning/manual/lightning.html) 
as a reference instead of the exports of this crate.

Tested on macOS likely to work on most linux distros, windows who knows.

# Warning / Next Steps
It is very hard to write code with just the raw bindings. For this to be useful I or someone else will need to write a safe wrapper.  
You will NOT be able to write idiomatic rust with this crate.
Most of the macros are not present as bindings so instead of say
 `jit_addi(JIT_R0, JIT_R0, 1);` you'll be writing `_jit_new_node_www(jit,jit_code_t_jit_code_addi, jit_reg_t__RAX as jit_word_t, jit_reg_t__RAX as jit_word_t, 1);`

# License
This crate is licensed under the same license as gnu lighting itself. GPL v3. See ./LICENSE and ./include/lightning-2.1.3/COPYING
for a copy of the license.

# Modification
The only modification made to gnulightning was to #include <math.h> in inclue/lightning-2.3.1/check/lightning.c.