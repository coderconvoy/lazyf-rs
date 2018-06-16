//!#Lazyf
//!
//!Lazyf is both a file format, and a mechanism for loading configurations.
//!It's primary aim is to allow the user to be as lazy as possible when
//!trying to get user options out of their code.
//!
//!With lazyf user options come from two places. -Flags and config files.
//!The cfg (config) module combines the lzlist (lazyfile) module and the
//!flag module
//!
//!The lazyf file format basically looks like this:
//!
//!...
//!Superman:
//!     power:Flying
//!     age:29
//!
//!Batman:
//!     power:Money
//!     age:40
//!...
//!
//!The simplesy way to get config options is:
//!
//!...
//!use lazyf::get::SGetter;
//!use lazyf::cfg::Cfg;
//!
//!let cf = cfg::Cfg::load_first("-c",   &["--config-location--"]);
//!let age = cf.get_t_def(("-bman","Batman.age),10);
//!//age == 40
//!...
//!
//!In this config location will be the location specified after the flag -c
//!or the first of the config locations to return a result.
//!if none are found a Cfg is still returned, as flags can still be used.
//!



pub mod flag;
pub mod lzlist;
pub mod cfg;
pub mod brace;

pub mod get;

//make trait public
pub use get::SGetter;



#[cfg(test)]
mod tests;


