extern crate rustc_hex as hex;

use self::hex::FromHex;
use super::*;
use bigint::U256;
use lib::*;

#[test]
fn simple() {
	let payload: &[u8; 32] = &[
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45
	];

	let mut stream = Stream::new(&payload[..]);

	let val: u32 = stream.pop().unwrap();

	assert_eq!(val, 69);
}

#[test]
fn bytes() {
	let encoded = ("".to_owned() +
		"0000000000000000000000000000000000000000000000000000000000000020" +
		"0000000000000000000000000000000000000000000000000000000000000002" +
		"1234000000000000000000000000000000000000000000000000000000000000")
		.from_hex().unwrap();

	let mut stream = Stream::new(&encoded);

	let bytes: Bytes = stream.pop().unwrap();

	assert_eq!(vec![0x12u8, 0x34], bytes.0);
}

#[test]
fn two_bytes() {
	let encoded = ("".to_owned() +
		"0000000000000000000000000000000000000000000000000000000000000040" +
		"0000000000000000000000000000000000000000000000000000000000000080" +
		"000000000000000000000000000000000000000000000000000000000000001f" +
		"1000000000000000000000000000000000000000000000000000000000000200" +
		"0000000000000000000000000000000000000000000000000000000000000020" +
		"0010000000000000000000000000000000000000000000000000000000000002"
	).from_hex().unwrap();

	let mut stream = Stream::new(&encoded);

	let bytes1: Bytes = stream.pop().unwrap();
	let bytes2: Bytes = stream.pop().unwrap();

	assert_eq!(bytes1.0, "10000000000000000000000000000000000000000000000000000000000002".from_hex().unwrap());
	assert_eq!(bytes2.0, "0010000000000000000000000000000000000000000000000000000000000002".from_hex().unwrap());
}

fn double_decode<T1: super::AbiType, T2: super::AbiType>(payload: &[u8]) -> (T1, T2) {
	let mut stream = super::Stream::new(payload);
	(
		stream.pop().expect("argument type 1 should be decoded"),
		stream.pop().expect("argument type 2 should be decoded"),
	)
}

fn triple_decode<T1: super::AbiType, T2: super::AbiType, T3: super::AbiType>(payload: &[u8]) -> (T1, T2, T3) {
	let mut stream = super::Stream::new(payload);
	(
		stream.pop().expect("argument type 1 should be decoded"),
		stream.pop().expect("argument type 2 should be decoded"),
		stream.pop().expect("argument type 3 should be decoded"),
	)
}

fn single_encode<T: super::AbiType>(val: T) -> Vec<u8> {
	let mut sink = super::Sink::new(1);
	sink.push(val);
	sink.finalize_panicking()
}

#[test]
fn u32_encode() {
	assert_eq!(
		single_encode(69),
		vec![
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
			0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45
		]
	);
}

#[test]
fn bytes_encode() {
	assert_eq!(
		single_encode(Bytes(vec![0x12u8, 0x34])),
		("".to_owned() +
		"0000000000000000000000000000000000000000000000000000000000000020" +
		"0000000000000000000000000000000000000000000000000000000000000002" +
		"1234000000000000000000000000000000000000000000000000000000000000")
		.from_hex().unwrap()
	);
}

#[test]
fn sample1_decode() {
	let payload: &[u8] = &[
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45,
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
	];

	let (v1, v2) = double_decode::<u32, bool>(&payload);

	assert_eq!(v1, 69);
	assert_eq!(v2, true);
}

#[test]
fn sample1_encode() {
	let sample: &[u8] = &[
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45,
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
	];

	let mut sink = Sink::new(2);
	sink.push(69u32);
	sink.push(true);

	assert_eq!(&sink.finalize_panicking()[..], &sample[..]);
}

#[test]
fn sample2_decode() {
	let sample: &[u8] = &[
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60,
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa0,
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
		0x64, 0x61, 0x76, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03,
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
		0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03,
	];

	let (v1, v2, v3) = triple_decode::<Bytes, bool, Vec<U256>>(&sample);

	assert_eq!(v1.0, vec![100, 97, 118, 101]);
	assert_eq!(v2, true);
	assert_eq!(v3, vec![U256::from(1), U256::from(2), U256::from(3)]);
}

#[test]
fn negative_i32() {
	let x: i32 = -1;
	let mut sink = ::eth::Sink::new(1);
	sink.push(x);
	let payload = sink.finalize_panicking();

	assert_eq!(
		&payload[..],
		&[0xff; 32]
	);

	let mut stream = ::eth::Stream::new(&payload[..]);
	let value: i32 = stream.pop().expect("x failed to pop");
	assert_eq!(value, x);
}

#[test]
fn negative_i32_max() {
	let x: i32 = i32::min_value();
	let mut sink = ::eth::Sink::new(1);
	sink.push(x);
	let payload = sink.finalize_panicking();

	assert_eq!(
		&payload[..],
		&[
			0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
		  	0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x80, 0x00, 0x00, 0x00
		]
	);

	let mut stream = ::eth::Stream::new(&payload[..]);
	let value: i32 = stream.pop().expect("x failed to pop");
	assert_eq!(value, x);
}

#[test]
fn padding_test_i32() {
	let mut sample = [0xff; 32];
	sample[0] = 0x80;
	let mut stream = ::eth::Stream::new(&sample);
	assert_eq!(stream.pop::<i32>().unwrap_err(), Error::InvalidPadding);
}

#[test]
fn padding_test_i64() {
	let mut sample = [0xff; 32];
	sample[0] = 0x80;
	let mut stream = ::eth::Stream::new(&sample);
	assert_eq!(stream.pop::<i64>().unwrap_err(), Error::InvalidPadding);
}
