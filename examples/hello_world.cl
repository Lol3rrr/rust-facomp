func test() {
	print(80);
}

func main() {
	number test = 65 + 2;

	test();

	number another_test = 65 + 2;

	print(test);
	print(test);
	print(another_test);
	print(another_test);

	if (another_test == test) {
		print(66);
	}
}
