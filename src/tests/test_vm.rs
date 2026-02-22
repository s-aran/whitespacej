#[cfg(test)]
mod tests {
    use crate::{imp::Program, parser::Parser, vm::Machine};

    #[test]
    fn test_modulo_lhs_negative() {
        // -5 % 3 = -2
        let code = r"  		 	
   		
	 			
 	";

        let program = Parser::parse::<i64>(code);
        assert!(program.is_ok());

        let mut vm = Machine::new(program.unwrap());

        // push -5
        {
            let program = vm.fetch().clone();
            if let Program::Push(v) = program {
                assert_eq!(-5, v);
            } else {
                assert!(false);
            }

            let result = vm.exec(&program);
            assert!(result.is_ok());

            let stack = &vm.stack;
            assert_eq!(1, stack.stack.len());
            if let Some(e) = stack.stack.last() {
                assert_eq!(&-5, e);
            } else {
                assert!(false);
            }

            assert!(vm.next().is_ok());
        }

        // push 3
        {
            let program = vm.fetch().clone();
            if let Program::Push(v) = program {
                assert_eq!(3, v);
            } else {
                assert!(false);
            }

            let result = vm.exec(&program);
            assert!(result.is_ok());

            let stack = &vm.stack;
            assert_eq!(2, stack.stack.len());
            if let Some(e) = stack.stack.last() {
                assert_eq!(&3, e);
            } else {
                assert!(false);
            }

            assert!(vm.next().is_ok());
        }

        // mod
        {
            let program = vm.fetch().clone();
            assert!(Program::Modulo == program);

            let result = vm.exec(&program);
            assert!(result.is_ok());

            let stack = &vm.stack;
            assert_eq!(1, stack.stack.len());
            if let Some(e) = stack.stack.last() {
                assert_eq!(&-2, e);
            } else {
                assert!(false);
            }

            assert!(vm.next().is_ok());
        }

        // print number
        {
            let program = vm.fetch().clone();
            assert!(Program::OutputInt == program);

            let result = vm.exec(&program);
            assert!(result.is_ok());

            let stack = &vm.stack;
            assert_eq!(0, stack.stack.len());

            assert!(vm.next().is_err());
        }
    }

    #[test]
    fn test_modulo_rhs_negative() {
        // 5 % -3 = 2
        let code = r"   	 	
  			
	 			
 	";

        let program = Parser::parse::<i64>(code);
        assert!(program.is_ok());

        let mut vm = Machine::new(program.unwrap());

        // push -5
        {
            let program = vm.fetch().clone();
            if let Program::Push(v) = program {
                assert_eq!(5, v);
            } else {
                assert!(false);
            }

            let result = vm.exec(&program);
            assert!(result.is_ok());

            let stack = &vm.stack;
            assert_eq!(1, stack.stack.len());
            if let Some(e) = stack.stack.last() {
                assert_eq!(&5, e);
            } else {
                assert!(false);
            }

            assert!(vm.next().is_ok());
        }

        // push 3
        {
            let program = vm.fetch().clone();
            if let Program::Push(v) = program {
                assert_eq!(-3, v);
            } else {
                assert!(false);
            }

            let result = vm.exec(&program);
            assert!(result.is_ok());

            let stack = &vm.stack;
            assert_eq!(2, stack.stack.len());
            if let Some(e) = stack.stack.last() {
                assert_eq!(&-3, e);
            } else {
                assert!(false);
            }

            assert!(vm.next().is_ok());
        }

        // mod
        {
            let program = vm.fetch().clone();
            assert!(Program::Modulo == program);

            let result = vm.exec(&program);
            assert!(result.is_ok());

            let stack = &vm.stack;
            assert_eq!(1, stack.stack.len());
            if let Some(e) = stack.stack.last() {
                assert_eq!(&2, e);
            } else {
                assert!(false);
            }

            assert!(vm.next().is_ok());
        }

        // print number
        {
            let program = vm.fetch().clone();
            assert!(Program::OutputInt == program);

            let result = vm.exec(&program);
            assert!(result.is_ok());

            let stack = &vm.stack;
            assert_eq!(0, stack.stack.len());

            assert!(vm.next().is_err());
        }
    }
}
