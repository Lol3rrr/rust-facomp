func test(to_print: number) {
	print(to_print);
}

func print_add(n1: number, n2: number) {
	print(n1 + n2);
}

func main() {
	test(66);

	print_add(65, 2);

	number another_test = 65 + 2;

	print(another_test);
	print(another_test);

	if (another_test == 67) {
		print(66);
	}
}
