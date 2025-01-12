#![ allow( unused_imports, dead_code) ]

pub type DynError         = Box< dyn std::error::Error + 'static >;
pub type DynResult<T>     = Result<T, DynError>;
pub type DynSendError     = Box< dyn std::error::Error + Send + Sync + 'static >;
pub type DynSendResult<T> = Result<T, DynSendError>;

pub mod import
{
	pub use
	{
		async_executors :: { *                                                                                              } ,
		async_nursery   :: { *                                                                                              } ,
		futures         :: { StreamExt, TryStreamExt                                                                        } ,
		futures         :: { executor::block_on, SinkExt, channel::mpsc, task::SpawnExt                                     } ,
		std             :: { convert::TryFrom, rc::Rc, sync::{ Arc, atomic::{ AtomicBool, AtomicUsize, Ordering::SeqCst } } } ,
		std             :: { time::Duration                                                                                 } ,
		futures_timer   :: { Delay                                                                                          } ,
	};

	#[ cfg( not(target_arch = "wasm32") ) ]
	use tokio::{ runtime::Builder };
}
