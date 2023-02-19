use llvm_plugin::inkwell::values::FunctionValue;
use llvm_plugin::{
    FunctionAnalysisManager, LlvmFunctionPass, PassBuilder, PipelineParsing, PreservedAnalyses,
};

// PICKUP
// read more examples, maybe do the online assignment
// does casing matter
// integrate kaleidoscope with helloplugin
#[llvm_plugin::plugin(name = "HelloWorld", version = "0.1")]
fn plugin_registrar(builder: &mut PassBuilder) {
    builder.add_function_pipeline_parsing_callback(|name, manager| {
        match name {
            "hello-world" => {
                manager.add_pass(HelloWorldPass);
                PipelineParsing::Parsed
            }
            _ => PipelineParsing::NotParsed,
        }
    });
}

struct HelloWorldPass;
impl LlvmFunctionPass for HelloWorldPass {
    fn run_pass(
        &self,
        function: &mut FunctionValue,
        _manager: &FunctionAnalysisManager,
    ) -> PreservedAnalyses {
        println!("Hello from: {:?}", function.get_name());
        println!("  number of arguments: {}", function.count_params());
        PreservedAnalyses::All
    }
}
