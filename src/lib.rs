extern crate glob;

use std::path::Path;

use glob::{glob_with,Paths,PatternError,MatchOptions};

/// A trait providing glob methods.
/// 
/// The idea is “glob starting from here”
pub trait Glob {
	/// Glob here with explicit options
	fn glob_with(&self, pattern: &str, options: &MatchOptions) -> Result<Paths, PatternError>;
	
	/// Glob here with default options
	fn glob(&self, pattern: &str) -> Result<Paths, PatternError> {
		self.glob_with(pattern, &MatchOptions::new())
	}
	
	/// Glob inside of here with explicit options (`<here>/**/<pattern>`)
	fn rglob_with(&self, pattern: &str, options: &MatchOptions) -> Result<Paths, PatternError> {
		self.glob_with(&format!("**/{}", pattern), options)
	}
	
	/// Glob inside of here with default options (`<here>/**/<pattern>`)
	fn rglob(&self, pattern: &str) -> Result<Paths, PatternError> {
		self.rglob_with(pattern, &MatchOptions::new())
	}
}

impl<P: AsRef<Path>> Glob for P {
	fn glob_with(&self, pattern: &str, options: &MatchOptions) -> Result<Paths, PatternError> {
		glob_with(self.as_ref().join(pattern).to_str().unwrap(), options)
	}
}
