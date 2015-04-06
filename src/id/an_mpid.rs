/*  Copyright 2014 MaidSafe.net limited

    This MaidSafe Software is licensed to you under (1) the MaidSafe.net Commercial License,
    version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
    licence you accepted on initial access to the Software (the "Licences").

    By contributing code to the MaidSafe Software, or to this project generally, you agree to be
    bound by the terms of the MaidSafe Contributor Agreement, version 1.0, found in the root
    directory of this project at LICENSE, COPYING and CONTRIBUTOR respectively and also
    available at: http://www.maidsafe.net/licenses

    Unless required by applicable law or agreed to in writing, the MaidSafe Software distributed
    under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS
    OF ANY KIND, either express or implied.

    See the Licences for the specific language governing permissions and limitations relating to
    use of the MaidSafe Software.                                                                 */

extern crate rustc_serialize;
extern crate sodiumoxide;
extern crate cbor;
extern crate rand;

use cbor::CborTagEncode;
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use sodiumoxide::crypto;
use helper::*;
use common::NameType;
use traits::RoutingTrait;
use Random;
use std::mem;
use std::fmt;

/// AnMpid
///
/// #Examples
/// ```
/// extern crate sodiumoxide;
/// extern crate maidsafe_types;
///
/// // Generating publick and secret keys using sodiumoxide
/// let (pub_sign_key, sec_sign_key) = sodiumoxide::crypto::sign::gen_keypair();
/// let (pub_asym_key, sec_asym_key) = sodiumoxide::crypto::asymmetricbox::gen_keypair();
/// // Create AnMpid
/// let an_mpid = maidsafe_types::AnMpid::new((pub_sign_key, pub_asym_key), (sec_sign_key, sec_asym_key), maidsafe_types::NameType([3u8; 64]));
/// // Retrieving the values
/// let ref publicKeys = an_mpid.get_public_keys();
/// let ref secret_keys = an_mpid.get_secret_keys();
/// let ref name = an_mpid.get_name();
/// ```
///
pub struct AnMpid {
	public_keys: (crypto::sign::PublicKey, crypto::asymmetricbox::PublicKey),
	secret_keys: (crypto::sign::SecretKey, crypto::asymmetricbox::SecretKey),
	name: NameType,
}

impl RoutingTrait for AnMpid {
    fn get_name(&self) -> NameType {
        let sign_arr = &(&self.public_keys.0).0;
        let asym_arr = &(&self.public_keys.1).0;

        let mut arr_combined = [0u8; 64 * 2];

        for i in 0..sign_arr.len() {
            arr_combined[i] = sign_arr[i];
        }
        for i in 0..asym_arr.len() {
            arr_combined[64 + i] = asym_arr[i];
        }

        let digest = crypto::hash::sha512::hash(&arr_combined);

        NameType(digest.0)
    }

    fn get_owner(&self) -> Option<Vec<u8>> {
        Some(array_as_vector(&self.name.0))
    }
}

impl Clone for AnMpid {
	fn clone(&self) -> Self {
		let public_keys_clone: (crypto::sign::PublicKey, crypto::asymmetricbox::PublicKey)  = self.public_keys.clone();
		let secret_keys_clone: (crypto::sign::SecretKey, crypto::asymmetricbox::SecretKey)   = self.secret_keys.clone();
		let name_clone = self.name.clone();

		AnMpid{
			public_keys: public_keys_clone,
			secret_keys: secret_keys_clone,
			name: name_clone
		}
	}
}

impl fmt::Debug for AnMpid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let (crypto::sign::PublicKey(sign_pub_key), crypto::asymmetricbox::PublicKey(asym_pub_key)) = self.public_keys;
    	let (crypto::sign::SecretKey(sign_sec_key), crypto::asymmetricbox::SecretKey(asym_sec_key)) = self.secret_keys;

        write!(f, "AnMpid(public_keys:({:?}, {:?}), secret_keys:({:?}, {:?}), name: {:?})", sign_pub_key, asym_pub_key, sign_sec_key.to_vec(), asym_sec_key, self.name)
    }
}

impl PartialEq for AnMpid {
	fn eq(&self, other: &AnMpid) -> bool {		
		if self.name != other.name {
			return false;
		}
		let crypto::sign::PublicKey(self_sig_pub_key) = self.public_keys.0;  
		let crypto::sign::SecretKey(self_sig_sec_key) = self.secret_keys.0;  
		let crypto::asymmetricbox::PublicKey(self_asym_pub_key) = self.public_keys.1;  
		let crypto::asymmetricbox::SecretKey(self_asym_sec_key) = self.secret_keys.1;

		let crypto::sign::PublicKey(other_sig_pub_key) = other.public_keys.0;  
		let crypto::sign::SecretKey(other_sig_sec_key) = other.secret_keys.0;  
		let crypto::asymmetricbox::PublicKey(other_asym_pub_key) = other.public_keys.1;  
		let crypto::asymmetricbox::SecretKey(other_asym_sec_key) = other.secret_keys.1;

		self_sig_pub_key.iter().zip(other_sig_pub_key.iter()).all(|(a,b)| a == b) &&
		self_asym_pub_key.iter().zip(other_asym_pub_key.iter()).all(|(a,b)| a == b) &&
		self_sig_sec_key.iter().zip(other_sig_sec_key.iter()).all(|(a,b)| a == b) &&
		self_asym_sec_key.iter().zip(other_asym_sec_key.iter()).all(|(a,b)| a == b)		
	}
}

impl Random for AnMpid {
	fn generate_random() -> AnMpid {
		let mut arr: [u8; 64] = unsafe { mem::uninitialized() };
        for i in 0..64 {
            arr[i] = rand::random::<u8>();
        }

        let (sign_pub_key, sign_sec_key) = crypto::sign::gen_keypair();
        let (asym_pub_key, asym_sec_key) = crypto::asymmetricbox::gen_keypair();        
		AnMpid {
			public_keys: (sign_pub_key, asym_pub_key),
			secret_keys: (sign_sec_key, asym_sec_key),
			name: NameType(arr)
		}
	}
}

impl AnMpid {
	pub fn new(public_keys: (crypto::sign::PublicKey, crypto::asymmetricbox::PublicKey),
						 secret_keys: (crypto::sign::SecretKey, crypto::asymmetricbox::SecretKey),
						 name_type: NameType) -> AnMpid {
		AnMpid {
		public_keys: public_keys,
		secret_keys: secret_keys,
		name: name_type
		}
	}
	pub fn get_public_keys(&self) -> &(crypto::sign::PublicKey, crypto::asymmetricbox::PublicKey) {
		&self.public_keys
	}
	pub fn get_secret_keys(&self) -> &(crypto::sign::SecretKey, crypto::asymmetricbox::SecretKey) {
		&self.secret_keys
	}
	pub fn get_name(&self) -> &NameType {
		&self.name
	}
}

impl Encodable for AnMpid {
	fn encode<E: Encoder>(&self, e: &mut E)->Result<(), E::Error> {
		let (crypto::sign::PublicKey(pub_sign_vec), crypto::asymmetricbox::PublicKey(pub_asym_vec)) = self.public_keys;
		let (crypto::sign::SecretKey(sec_sign_vec), crypto::asymmetricbox::SecretKey(sec_asym_vec)) = self.secret_keys;

		CborTagEncode::new(5483_001, &(
				array_as_vector(&pub_sign_vec),
					array_as_vector(&pub_asym_vec),
					array_as_vector(&sec_sign_vec),
					array_as_vector(&sec_asym_vec),
				&self.name)).encode(e)
	}
}

impl Decodable for AnMpid {
	fn decode<D: Decoder>(d: &mut D)-> Result<AnMpid, D::Error> {
		try!(d.read_u64());
		let(pub_sign_vec, pub_asym_vec, sec_sign_vec, sec_asym_vec, name) = try!(Decodable::decode(d));
		let pub_keys = (crypto::sign::PublicKey(vector_as_u8_32_array(pub_sign_vec)),
				crypto::asymmetricbox::PublicKey(vector_as_u8_32_array(pub_asym_vec)));
		let sec_keys = (crypto::sign::SecretKey(vector_as_u8_64_array(sec_sign_vec)),
				crypto::asymmetricbox::SecretKey(vector_as_u8_32_array(sec_asym_vec)));
		Ok(AnMpid::new(pub_keys, sec_keys, name))
	}
}

#[test]
fn serialisation_an_mpid() {
	let (pub_sign_key, sec_sign_key) = crypto::sign::gen_keypair();
	let (pub_asym_key, sec_asym_key) = crypto::asymmetricbox::gen_keypair();

																																																																					 let obj_before = AnMpid::new((pub_sign_key, pub_asym_key), (sec_sign_key, sec_asym_key), NameType([3u8; 64]));

	let mut e = cbor::Encoder::from_memory();
	e.encode(&[&obj_before]).unwrap();

	let mut d = cbor::Decoder::from_bytes(e.as_bytes());
	let obj_after: AnMpid = d.decode().next().unwrap().unwrap();

	let &(crypto::sign::PublicKey(pub_sign_arr_before), crypto::asymmetricbox::PublicKey(pub_asym_arr_before)) = obj_before.get_public_keys();
																																																																					 let &(crypto::sign::PublicKey(pub_sign_arr_after), crypto::asymmetricbox::PublicKey(pub_asym_arr_after)) = obj_after.get_public_keys();
	let &(crypto::sign::SecretKey(sec_sign_arr_before), crypto::asymmetricbox::SecretKey(sec_asym_arr_before)) = obj_before.get_secret_keys();
	let &(crypto::sign::SecretKey(sec_sign_arr_after), crypto::asymmetricbox::SecretKey(sec_asym_arr_after)) = obj_after.get_secret_keys();
	let NameType(name_before) = *obj_before.get_name();
	let NameType(name_after) = *obj_after.get_name();

																																																																					 assert!(compare_u8_array(&name_before, &name_after));
	assert_eq!(pub_sign_arr_before, pub_sign_arr_after);
	assert_eq!(pub_asym_arr_before, pub_asym_arr_after);
	assert!(compare_u8_array(&sec_sign_arr_before, &sec_sign_arr_after));
	assert_eq!(sec_asym_arr_before, sec_asym_arr_after);
}
