func test(to_print: number) {
	print(to_print);
}

func print_add(n1: number, n2: number) {
	print(n1 + n2);
}

func calc(n1: number, n2: number) -> number {
	number result = n1 + n2;	

	return result;
}

func test_with_var() {
	number test_var = 0;
}

func print_loop() {
	number i = 85;
	while (i > 65) {
		print(i);	
		i = i - 1;
	}
}

func custom_mul(n1: number, n2: number) -> number {
	number result = 0;
	while (n2 > 0) {
		result = result + n1;
		n2 = n2 - 1;
	}

	return result;
}

func main() {
	test(66);

	number calced = calc(64, 3);
	print(calced);
	print(calc(65, 2));

	print_add(65, 2);

	number another_test = 65 + 2;

	print(another_test);
	print(another_test);

	if (another_test == 67) {
		print(66);
	}

	print_loop();

	print(custom_mul(17, 4));
}
