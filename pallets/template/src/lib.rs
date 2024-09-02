#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_system::{self as system, ensure_signed};
use cpython::{Python};

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
    trait Store for Module<T: Config> as TemplateModule {
        AIResult get(fn ai_result): Option<f64>;
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
        AIResultStored(AccountId, f64),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        PythonExecutionFailed,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn call_ai_model(origin) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;

            let gil = Python::acquire_gil();
            let py = gil.python();

            // Execute the Python function and handle potential errors
            let ai_result: f64 = match py.eval("predict_anomaly()", None, None) {
                Ok(value) => match value.extract() {
                    Ok(result) => result,
                    Err(_) => return Err(Error::<T>::PythonExecutionFailed.into()),
                },
                Err(_) => return Err(Error::<T>::PythonExecutionFailed.into()),
            };

            AIResult::put(ai_result);

            Self::deposit_event(Event::AIResultStored(who, ai_result));

            Ok(())
        }
    }
}
