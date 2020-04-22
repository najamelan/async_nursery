pub type DynError  = Box< dyn std::error::Error + Send + Sync + 'static >;
pub type DynResult = Result<(), DynError>;

pub mod import
{
	pub use
	{
		async_executors :: { *                  } ,
		async_nursery   :: { *                  } ,
		futures         :: { StreamExt          } ,
		tokio           :: { runtime::Builder   } ,
		futures         :: { executor::block_on } ,
		std             :: { convert::TryFrom   } ,
	};
}
