
use
{
	log :: { * } ,
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
