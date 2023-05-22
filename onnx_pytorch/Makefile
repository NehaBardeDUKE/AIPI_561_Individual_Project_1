install:
	#download onnxruntime
	wget https://github.com/onnx/models/blob/main/text/machine_comprehension/bert-squad/model/bertsquad-12-int8.onnx
format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run