use std::fs;
use std::path::Path;
use std::time::SystemTime;

use gc::{Finalize, Trace};

use super::{
	CallContext,
	RustFun,
	NativeFun,
	Dict,
	Panic,
	Value,
};


inventory::submit! { RustFun::from(FileExists) }
inventory::submit! { RustFun::from(DirExists) }
inventory::submit! { RustFun::from(Metadata) }

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

#[derive(Trace, Finalize)]
struct Metadata;

impl Metadata {
	fn time(time: std::io::Result<SystemTime>) -> Option<Value> {
		match time {
			Ok(t) => match t.duration_since(SystemTime::UNIX_EPOCH) {
				Ok(duration) => Some((duration.as_millis() as i64).into()),
				Err(_) => None
			},
			Err(_) => None
		}
	}
}

impl NativeFun for Metadata {
	fn name(&self) -> &'static str { "std.io.metadata" }

	fn call(&self, context: CallContext) -> Result<Value, Panic> {
		match context.args() {
			[ Value::String(ref string) ] => match fs::metadata(&Path::new(string)) {
				Ok(metadata) => {
					let dict = Dict::default();

					// As hush uses i64 for Int, large files will overflow
					dict.insert("len".into(), (metadata.len() as i64).into());
					dict.insert("is_file".into(), metadata.is_file().into());
					dict.insert("is_dir".into(), metadata.is_dir().into());
					dict.insert("is_symlink".into(), metadata.is_symlink().into());

					if let Some(created) = Self::time(metadata.created()) {
						dict.insert("created".into(), created);
					}
					if let Some(accessed) = Self::time(metadata.accessed()) {
						dict.insert("accessed".into(), accessed);
					}
					if let Some(modified) = Self::time(metadata.modified()) {
						dict.insert("modified".into(), modified);
					}

					Ok(dict.into())
				},
				Err(error) => Ok(false.into())
			},

			[ other ] => Err(Panic::type_error(other.copy(), "string", context.pos)),
			args => Err(Panic::invalid_args(args.len() as u32, 1, context.pos))
		}
	}
}



