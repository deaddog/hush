use std::path::Path;

use gc::{Finalize, Trace};

use super::{
	CallContext,
	RustFun,
	NativeFun,
	Panic,
	Value,
};


inventory::submit! { RustFun::from(FileExists) }
inventory::submit! { RustFun::from(DirExists) }

#[derive(Trace, Finalize)]
struct FileExists;

impl NativeFun for FileExists {
	fn name(&self) -> &'static str { "std.io.file_exists" }

	fn call(&self, context: CallContext) -> Result<Value, Panic> {
		match context.args() {
			[ Value::String(ref string) ] => Ok(Path::new(string).is_file().into()),

			[ other ] => Err(Panic::type_error(other.copy(), "string", context.pos)),
			args => Err(Panic::invalid_args(args.len() as u32, 1, context.pos))
		}
	}
}

#[derive(Trace, Finalize)]
struct DirExists;

impl NativeFun for DirExists {
	fn name(&self) -> &'static str { "std.io.dir_exists" }

	fn call(&self, context: CallContext) -> Result<Value, Panic> {
		match context.args() {
			[ Value::String(ref string) ] => Ok(Path::new(string).is_dir().into()),

			[ other ] => Err(Panic::type_error(other.copy(), "string", context.pos)),
			args => Err(Panic::invalid_args(args.len() as u32, 1, context.pos))
		}
	}
}

