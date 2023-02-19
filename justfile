hello_world:
	cargo b --example hello_world
	$LLVM_DIR/bin/clang -O1 -S -emit-llvm c_src/input_for_hello.c -o ll_src/input_for_hello.ll
	$LLVM_DIR/bin/opt --load-pass-plugin=target/debug/examples/libhello_world.so --passes=hello-world ll_src/input_for_hello.ll -S -o out/input_for_hello.ll

opcode_counter:
	cargo b --example opcode_counter
	$LLVM_DIR/bin/clang -O1 -S -emit-llvm c_src/input_for_cc.c -o ll_src/input_for_cc.ll
	$LLVM_DIR/bin/opt --load-pass-plugin=target/debug/examples/libopcode_counter.so --passes=opcode-counter-printer ll_src/input_for_cc.ll -S -o out/input_for_cc.ll
