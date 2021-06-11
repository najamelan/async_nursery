// Not everything is used in all examples.
//
#![allow(dead_code)]

use
{
	tracing_crate :: { warn } ,
};


pub type DynResult<T> = Result<T, Box< dyn std::error::Error + Send + Sync + 'static >>;


// Will print something when dropped.
//
#[ derive( Debug ) ]
//
pub struct AlertOnDrop(pub &'static str);

impl Drop for AlertOnDrop
{
	fn drop( &mut self )
	{
		warn!( "Dropped: {}", self.0 );
	}
}



pub fn setup_tracing()
{
	let _ = tracing_subscriber::fmt::Subscriber::builder()

		.with_env_filter( "debug,async_std=warn" )
		.without_time()
	   .try_init()
	;
}
