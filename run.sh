#!/bin/sh

mkdir -p sample.d

output="./sample.d/output.asn1"

echo converting a float value to der bytes...
node --eval '
	import { readFile, writeFile } from "node:fs/promises";

	const filename="./rs_float2asn1.wasm";

	const main = () => {
		return Promise.resolve()
		.then(_ => {
			return readFile(filename);
		})
		.then(WebAssembly.compile)
		.then(WebAssembly.instantiate)
		.then(instance => {
			const {
				memory,
				der_byte_offset,
				double2der,
				bin2der,
			} = instance.exports;
			//const size = double2der(2.99792458);
			const size = bin2der(299792458, 10, -8);
			//const size = bin2der(1.0, 2, 3);
			//const size = bin2der(1.0, 2, -3);
			//const size = bin2der(0.125, 2, 0);
			//const size = bin2der(42195.0, 10, -3);
			//const size = double2der(42.195);
			const offset = der_byte_offset();
			const buffer = memory.buffer;
			const view = new DataView(buffer, offset, size);
			return writeFile("/dev/stdout", view);
		});
	};

	main()
	.catch(console.error);
' |
	cat > "${output}"

echo
echo converting DER to JER...
cat "${output}" |	
	xxd -ps |
	tr -d '\n' |
	python3 \
		-m asn1tools \
		convert \
		-i der \
		-o jer \
		./double.asn \
		DoubleNumber \
		-

echo
echo dumping DER bytes...
cat "${output}" |	
	fq -d asn1_ber

