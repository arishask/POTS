// Операция, выполняемая над двумя подвыражениями
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

// Выражение в форме дерева
#[derive(Debug)]
enum Expression {
    // Операция над двумя подвыражениями
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    // Литеральное значение
    Value(i64),
}

// Рекурсивный вычислитель арифметических выражений
fn eval(e: Expression) -> Result<i64, String> {
    // Определяем вариант
    match e {
        // Операция.
        // Деструктуризация
        Expression::Op { op, left, right } => {
            // Рекурсивно вычисляем левое подвыражение
            let left = match eval(*left) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
            // Рекурсивно вычисляем правое подвыражение
            let right = match eval(*right) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
            // Возвращаем результат, упакованный в `Ok`
            Ok(
              // Определяем тип операции
              match op {
                  Operation::Add => left + right,
                  Operation::Sub => left - right,
                  Operation::Mul => left * right,
                  Operation::Div => {
                      // Если правый операнд равняется 0
                      if right == 0 {
                          // Возвращаем вызывающему (caller) сообщение об ошибке, обернутое в `Err`.
                          // Мы распространяем (propagate) ошибку, поэтому она не оборачивается в `Ok`
                          return Err(String::from("деление на ноль"));
                      } else {
                          left / right
                      }
                  }
              }
            )
        }
        // Значение.
        // Просто возвращаем значение, упакованное в `Ok`
        Expression::Value(v) => Ok(v),
    }
}

fn main() {
    let expr = Expression::Op {
        op: Operation::Sub,
        left: Box::new(Expression::Value(20)),
        right: Box::new(Expression::Value(10)),
    };
    println!("выражение: {:?}", expr);
    println!("результат: {:?}", eval(expr));
}

// Модуль с тестами - код компилируется только при запуске тестов с помощью команды `cargo test`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(eval(Expression::Value(19)), Ok(19));
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Add,
                left: Box::new(Expression::Value(10)),
                right: Box::new(Expression::Value(20)),
            }),
            Ok(30)
        );
    }

    #[test]
    fn test_recursion() {
        let term1 = Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(9)),
        };
        let term2 = Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Op {
                op: Operation::Sub,
                left: Box::new(Expression::Value(3)),
                right: Box::new(Expression::Value(4)),
            }),
            right: Box::new(Expression::Value(5)),
        };
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Add,
                left: Box::new(term1),
                right: Box::new(term2),
            }),
            Ok(85)
        );
    }

    #[test]
    fn test_error() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Div,
                left: Box::new(Expression::Value(99)),
                right: Box::new(Expression::Value(0)),
            }),
            Err(String::from("деление на ноль"))
        );
    }
}
