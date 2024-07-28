use super::*;

#[test]
fn stmt_2() {
    let left = vec![Statement::Impl {
        name: Token {
            token: Ident,
            len: 4,
            lexeme: "Name".to_string(),
            value: None,
            line: 1,
        },
        body: vec![Statement::Func {
            name: Token {
                token: Ident,
                len: 5,
                lexeme: "empty".to_string(),
                value: None,
                line: 1,
            },
            value_type: Token {
                token: VoidIdent,
                len: 4,
                lexeme: "void".to_string(),
                value: None,
                line: 1,
            },
            body: FuncBody::Statements(vec![]),
            params: vec![],
            is_async: false,
            is_pub: false,
            is_impl: false,
            is_mut: false,
        }],
    }];
    let right = get_ast("impl Name {fn empty() -> void {}}");

    assert_eq!(left, right, "testing `fn empty() -> void {{}}`");
}

#[test]
fn stmt_1() {
    let left = vec![Statement::Impl {
        name: Token {
            token: Ident,
            len: 4,
            lexeme: "Name".to_string(),
            value: None,
            line: 1,
        },
        body: vec![],
    }];
    let right = get_ast("impl Name {}");

    assert_eq!(left, right, "testing `impl Name {{}}`");
}
