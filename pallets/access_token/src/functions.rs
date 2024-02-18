use crate::*;

impl<T: Config> Pallet<T> {
	pub fn do_hash_exist(hash: &T::Hash) -> Result<(), Error<T>> {
		if Self::access_token_by_hash(hash).is_some() {
			return Err(Error::<T>::AlreadyExists);
		}

		Ok(())
	}
}
