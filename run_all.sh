for i in {1..20}
	do
		cd day$i
		cargo run --release
		cd ..
	done
