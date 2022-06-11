# gnulighting-sys

Compiles gnu-lightning 2.3.0 and makes it available via this crate. There are quite a few
irrelevant bindings included so definitely use the [gnu lightning manual](https://www.gnu.org/software/lightning/manual/lightning.html) 
as a reference instead of the exports of this crate.

Tested on macOS likely to work on most linux distros, windows who knows.

# Warning
This is not a safe wrapper around gnulightning. You will NOT be able to write idiomatic rust with it.
Most of the macros are not present as bindings so instead of say
 `jit_addi(JIT_R0, JIT_R0, 1);` you'll be writing `jit_new_node_www(jit_code_addi,R,v,w)

# License
This crate is licensed under the same license as gnu lighting itself. GPL v3. See ./LICENSE and ./include/lightning-2.1.3/COPYING
for a copy of the license.

# Modification
The only modification made to gnulightning was to #include <math.h> in inclue/lightning-2.3.1/check/lightning.c.