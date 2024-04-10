mod parsing_and_numbers {
	use forth::*;

	#[test]
	fn numbers_just_get_pushed_onto_the_stack() {
		let mut f = Forth::new();
		assert!(f.eval("1 2 3 4 5").is_ok());
		assert_eq!(f.stack(), [1, 2, 3, 4, 5]);
	}

	#[test]
	fn pushes_negative_numbers_onto_the_stack() {
		let mut f = Forth::new();
		assert!(f.eval("-1 -2 -3 -4 -5").is_ok());
		assert_eq!(f.stack(), [-1, -2, -3, -4, -5]);
	}
}

mod addition {
	use forth::*;

	#[test]
	fn can_add_two_numbers() {
		let mut f = Forth::new();
		assert!(f.eval("1 2 +").is_ok());
		assert_eq!(f.stack(), [3]);
	}

	#[test]
	fn errors_if_there_is_nothing_on_the_stack() {
		let mut f = Forth::new();
		assert_eq!(f.eval("+"), Err(FError::StackUnderflow));
	}

	#[test]
	fn errors_if_there_is_only_one_value_on_the_stack() {
		let mut f = Forth::new();
		assert_eq!(f.eval("1 +"), Err(FError::StackUnderflow));
	}
}

mod subtraction {
	use forth::*;

	#[test]
	fn can_subtract_two_numbers() {
		let mut f = Forth::new();
		assert!(f.eval("3 4 -").is_ok());
		assert_eq!(f.stack(), [-1]);
	}

	#[test]
	fn errors_if_there_is_nothing_on_the_stack() {
		let mut f = Forth::new();
		assert_eq!(f.eval("-"), Err(FError::StackUnderflow));
	}

	#[test]
	fn errors_if_there_is_only_one_value_on_the_stack() {
		let mut f = Forth::new();
		assert_eq!(f.eval("1 -"), Err(FError::StackUnderflow));
	}
}

mod multiplication {
	use forth::*;

	#[test]
	fn can_multiply_two_numbers() {
		let mut f = Forth::new();
		assert!(f.eval("2 4 *").is_ok());
		assert_eq!(f.stack(), [8]);
	}

	#[test]
	fn errors_if_there_is_nothing_on_the_stack() {
		let mut f = Forth::new();
		assert_eq!(f.eval("*"), Err(FError::StackUnderflow));
	}

	#[test]
	fn errors_if_there_is_only_one_value_on_the_stack() {
		let mut f = Forth::new();
		assert_eq!(f.eval("1 *"), Err(FError::StackUnderflow));
	}
}

mod division {
	use forth::*;

	#[test]
	fn can_divide_two_numbers() {
		let mut f = Forth::new();
		assert!(f.eval("12 3 /").is_ok());
		assert_eq!(f.stack(), [4]);
	}

	#[test]
	fn performs_integer_division() {
		let mut f = Forth::new();
		assert!(f.eval("8 3 /").is_ok());
		assert_eq!(f.stack(), [2]);
	}

	#[test]
	fn errors_if_dividing_by_zero() {
		let mut f = Forth::new();
		assert_eq!(f.eval("4 0 /"), Err(FError::DivisionByZero));
	}

	#[test]
	fn errors_if_there_is_nothing_on_the_stack() {
		let mut f = Forth::new();
		assert_eq!(f.eval("/"), Err(FError::StackUnderflow));
	}

	#[test]
	fn errors_if_there_is_only_one_value_on_the_stack() {
		let mut f = Forth::new();
		assert_eq!(f.eval("1 /"), Err(FError::StackUnderflow));
	}
}

mod combined_arithmetic {
	use forth::*;

	#[test]
	fn addition_and_subtraction() {
		let mut f = Forth::new();
		assert!(f.eval("1 2 + 4 -").is_ok());
		assert_eq!(f.stack(), [-1]);
	}

	#[test]
	fn multiplication_and_division() {
		let mut f = Forth::new();
		assert!(f.eval("2 4 * 3 /").is_ok());
		assert_eq!(f.stack(), [2]);
	}
}

mod dup {
	use forth::*;

	#[test]
	fn copies_a_value_on_the_stack() {
		let mut f = Forth::new();
		assert!(f.eval("1 dup").is_ok());
		assert_eq!(f.stack(), [1, 1]);
	}

	#[test]
	fn copies_the_top_value_on_the_stack() {
		let mut f = Forth::new();
		assert!(f.eval("1 2 dup").is_ok());
		assert_eq!(f.stack(), [1, 2, 2]);
	}

	#[test]
	fn errors_if_there_is_nothing_on_the_stack() {
		let mut f = Forth::new();
		assert_eq!(f.eval("dup"), Err(FError::StackUnderflow));
	}
}

mod drop {
	use forth::*;

	#[test]
	fn removes_the_top_value_on_the_stack_if_it_is_the_only_one() {
		let mut f = Forth::new();
		assert!(f.eval("1 drop").is_ok());
		assert_eq!(f.stack(), []);
	}

	#[test]
	fn removes_the_top_value_on_the_stack_if_it_is_not_the_only_one() {
		let mut f = Forth::new();
		assert!(f.eval("1 2 drop").is_ok());
		assert_eq!(f.stack(), [1]);
	}

	#[test]
	fn errors_if_there_is_nothing_on_the_stack() {
		let mut f = Forth::new();
		assert_eq!(f.eval("drop"), Err(FError::StackUnderflow));
	}
}

mod swap {
	use forth::*;

	#[test]
	fn swaps_the_top_two_values_on_the_stack_if_they_are_the_only_ones() {
		let mut f = Forth::new();
		assert!(f.eval("1 2 swap").is_ok());
		assert_eq!(f.stack(), [2, 1]);
	}

	#[test]
	fn swaps_the_top_two_values_on_the_stack_if_they_are_not_the_only_ones() {
		let mut f = Forth::new();
		assert!(f.eval("1 2 3 swap").is_ok());
		assert_eq!(f.stack(), [1, 3, 2]);
	}

	#[test]
	fn errors_if_there_is_nothing_on_the_stack() {
		let mut f = Forth::new();
		assert_eq!(f.eval("swap"), Err(FError::StackUnderflow));
	}

	#[test]
	fn errors_if_there_is_only_one_value_on_the_stack() {
		let mut f = Forth::new();
		assert_eq!(f.eval("1 swap"), Err(FError::StackUnderflow));
	}
}

mod over {
	use forth::*;

	#[test]
	fn copies_the_second_element_if_there_are_only_two() {
		let mut f = Forth::new();
		assert!(f.eval("1 2 over").is_ok());
		assert_eq!(f.stack(), [1, 2, 1]);
	}

	#[test]
	fn copies_the_second_element_if_there_are_more_than_two() {
		let mut f = Forth::new();
		assert!(f.eval("1 2 3 over").is_ok());
		assert_eq!(f.stack(), [1, 2, 3, 2]);
	}

	#[test]
	fn errors_if_there_is_nothing_on_the_stack() {
		let mut f = Forth::new();
		assert_eq!(f.eval("over"), Err(FError::StackUnderflow));
	}

	#[test]
	fn errors_if_there_is_only_one_value_on_the_stack() {
		let mut f = Forth::new();
		assert_eq!(f.eval("1 over"), Err(FError::StackUnderflow));
	}
}

mod user_defined_words {
	use forth::*;

	#[test]
	fn can_consist_of_built_in_words() {
		let mut f = Forth::new();
		let addition = dbg!(f.eval(": dup-twice dup dup ;"));
		assert!(addition.is_ok());
		assert!(f.eval("1 dup-twice").is_ok());
		assert_eq!(f.stack(), [1, 1, 1]);
	}

	#[test]
	fn execute_in_the_right_order() {
		let mut f = Forth::new();
		assert!(f.eval(": countup 1 2 3 ;").is_ok());
		assert!(f.eval("countup").is_ok());
		assert_eq!(f.stack(), [1, 2, 3]);
	}

	#[test]
	#[ignore]
	fn can_override_other_user_defined_words() {
		let mut f = Forth::new();
		assert!(f.eval(": foo dup ;").is_ok());
		assert!(f.eval(": foo dup dup ;").is_ok());
		assert!(f.eval("1 foo").is_ok());
		assert_eq!(f.stack(), [1, 1, 1]);
	}

	#[test]
	#[ignore]
	fn can_override_built_in_words() {
		let mut f = Forth::new();
		assert!(f.eval(": swap dup ;").is_ok());
		assert!(f.eval("1 swap").is_ok());
		assert_eq!(f.stack(), [1, 1]);
	}

	#[test]
	#[ignore]
	fn can_override_built_in_operators() {
		let mut f = Forth::new();
		assert!(f.eval(": + * ;").is_ok());
		assert!(f.eval("3 4 +").is_ok());
		assert_eq!(f.stack(), [12]);
	}

	#[test]
	fn can_use_different_words_with_the_same_name() {
		let mut f = Forth::new();
		assert!(f.eval(": foo 5 ;").is_ok());
		assert!(f.eval(": bar foo ;").is_ok());
		assert!(f.eval(": foo 6 ;").is_ok());
		assert!(f.eval("bar foo").is_ok());
		assert_eq!(f.stack(), [5, 6]);
	}

	#[test]
	#[ignore]
	fn can_define_word_that_uses_word_with_the_same_name() {
		let mut f = Forth::new();
		assert!(f.eval(": foo 10 ;").is_ok());
		assert!(f.eval(": foo foo 1 + ;").is_ok());
		assert!(f.eval("foo").is_ok());
		assert_eq!(f.stack(), [11]);
	}

	#[test]
	#[ignore]
	fn cannot_redefine_non_negative_numbers() {
		let mut f = Forth::new();
		assert_eq!(f.eval(": 1 2 ;"), Err(FError::InvalidWord));
	}

	#[test]
	#[ignore]
	fn cannot_redefine_negative_numbers() {
		let mut f = Forth::new();
		assert_eq!(f.eval(": -1 2 ;"), Err(FError::InvalidWord));
	}

	#[test]
	#[ignore]
	fn errors_if_executing_a_non_existent_word() {
		let mut f = Forth::new();
		assert_eq!(f.eval("foo"), Err(FError::UnknownWord));
	}

	#[test]
	#[ignore]
	fn only_defines_locally() {
		let mut f = Forth::new();
		assert!(f.eval(": + - ;").is_ok());
		assert!(f.eval("1 1 +").is_ok());
		assert_eq!(f.stack(), [0]);
		let mut f = Forth::new();
		assert!(f.eval("1 1 +").is_ok());
		assert_eq!(f.stack(), [2]);
	}
}

mod case_insensitivity {
	use forth::*;

	#[test]
	#[ignore]
	fn dup_is_case_insensitive() {
		let mut f = Forth::new();
		assert!(f.eval("1 DUP Dup dup").is_ok());
		assert_eq!(f.stack(), [1, 1, 1, 1]);
	}

	#[test]
	#[ignore]
	fn drop_is_case_insensitive() {
		let mut f = Forth::new();
		assert!(f.eval("1 2 3 4 DROP Drop drop").is_ok());
		assert_eq!(f.stack(), [1]);
	}

	#[test]
	#[ignore]
	fn swap_is_case_insensitive() {
		let mut f = Forth::new();
		assert!(f.eval("1 2 SWAP 3 Swap 4 swap").is_ok());
		assert_eq!(f.stack(), [2, 3, 4, 1]);
	}

	#[test]
	#[ignore]
	fn over_is_case_insensitive() {
		let mut f = Forth::new();
		assert!(f.eval("1 2 OVER Over over").is_ok());
		assert_eq!(f.stack(), [1, 2, 1, 2, 1]);
	}

	#[test]
	#[ignore]
	fn user_defined_words_are_case_insensitive() {
		let mut f = Forth::new();
		assert!(f.eval(": foo dup ;").is_ok());
		assert!(f.eval("1 FOO Foo foo").is_ok());
		assert_eq!(f.stack(), [1, 1, 1, 1]);
	}

	#[test]
	#[ignore]
	fn definitions_are_case_insensitive() {
		let mut f = Forth::new();
		assert!(f.eval(": SWAP DUP Dup dup ;").is_ok());
		assert!(f.eval("1 swap").is_ok());
		assert_eq!(f.stack(), [1, 1, 1, 1]);
	}
}
