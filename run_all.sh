for i in {1..25}
	do
		cd day$i
		cargo run --release
		cd ..
	done
