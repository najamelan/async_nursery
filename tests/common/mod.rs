#![ allow( unused_imports, dead_code) ]

pub type DynError     = Box< dyn std::error::Error + Send + Sync + 'static >;
pub type DynResult<T> = Result<T, DynError>;

pub mod import
{
	pub use
	{
		async_executors :: { *                           } ,
		async_nursery   :: { *                           } ,
		futures         :: { StreamExt, TryStreamExt     } ,
		tokio           :: { runtime::Builder            } ,
		futures         :: { executor::block_on, SinkExt, channel::mpsc, task::SpawnExt } ,
		std             :: { convert::TryFrom, rc::Rc, sync::{ Arc, atomic::{ AtomicBool, AtomicUsize, Ordering::SeqCst } } } ,
		std             :: { time::Duration           } ,
		futures_timer   :: { Delay                    } ,
	};
}
