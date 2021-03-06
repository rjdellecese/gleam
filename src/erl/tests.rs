use super::*;
use std::default::Default;

#[test]
fn record_definition_test() {
    assert_eq!(
        record_definition("PetCat", &[&"name", &"is_cute",]),
        "-record(pet_cat, {name, is_cute}).\n".to_string()
    );
}

#[test]
fn module_test() {
    use std::collections::HashMap;
    let m = Module {
        type_info: crate::typ::Module {
            name: vec!["magic".to_string()],
            types: HashMap::new(),
            values: HashMap::new(),
        },
        name: vec!["magic".to_string()],
        statements: vec![
            Statement::ExternalType {
                meta: Default::default(),
                public: true,
                name: "Any".to_string(),
                args: vec![],
            },
            Statement::CustomType {
                meta: Default::default(),
                public: true,
                name: "Any".to_string(),
                args: vec![],
                constructors: vec![RecordConstructor {
                    meta: Default::default(),
                    name: "Ok".to_string(),
                    args: vec![],
                }],
            },
            Statement::Import {
                meta: Default::default(),
                module: vec!["result".to_string()],
                as_name: None,
                unqualified: vec![],
            },
            Statement::ExternalFn {
                meta: Default::default(),
                args: vec![
                    ExternalFnArg {
                        label: None,
                        typ: TypeAst::Constructor {
                            meta: Default::default(),
                            module: None,
                            args: vec![],
                            name: "Int".to_string(),
                        },
                    },
                    ExternalFnArg {
                        label: None,
                        typ: TypeAst::Constructor {
                            meta: Default::default(),
                            module: None,
                            args: vec![],
                            name: "Int".to_string(),
                        },
                    },
                ],
                name: "add_ints".to_string(),
                fun: "add".to_string(),
                module: "int".to_string(),
                public: false,
                retrn: TypeAst::Constructor {
                    meta: Default::default(),
                    module: None,
                    args: vec![],
                    name: "Int".to_string(),
                },
            },
            Statement::ExternalFn {
                meta: Default::default(),
                args: vec![],
                name: "map".to_string(),
                fun: "new".to_string(),
                module: "maps".to_string(),
                public: true,
                retrn: TypeAst::Constructor {
                    meta: Default::default(),
                    module: None,
                    args: vec![],
                    name: "Map".to_string(),
                },
            },
        ],
    };
    let expected = "-module(magic).
-compile(no_auto_import).

-export([map/0]).

add_ints(A, B) ->
    int:add(A, B).

map() ->
    maps:new().
"
    .to_string();
    assert_eq!(expected, module(m));

    let m = Module {
        type_info: crate::typ::Module {
            name: vec!["term".to_string()],
            types: HashMap::new(),
            values: HashMap::new(),
        },
        name: vec!["term".to_string()],
        statements: vec![
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                public: false,
                args: vec![],
                name: "int".to_string(),
                body: Expr::Int {
                    typ: crate::typ::int(),
                    meta: Default::default(),
                    value: 176,
                },
            },
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                public: false,
                args: vec![],
                name: "float".to_string(),
                body: Expr::Float {
                    meta: Default::default(),
                    typ: crate::typ::float(),
                    value: 11177.324401,
                },
            },
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                public: false,
                args: vec![],
                name: "nil".to_string(),
                body: Expr::Nil {
                    meta: Default::default(),
                    typ: crate::typ::int(),
                },
            },
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                public: false,
                args: vec![],
                name: "string".to_string(),
                body: Expr::String {
                    meta: Default::default(),
                    typ: crate::typ::string(),
                    value: "Hello there!".to_string(),
                },
            },
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                public: false,
                args: vec![],
                name: "seq".to_string(),
                body: Expr::Seq {
                    typ: crate::typ::int(),
                    first: Box::new(Expr::Int {
                        typ: crate::typ::int(),
                        meta: Default::default(),
                        value: 1,
                    }),
                    then: Box::new(Expr::Int {
                        typ: crate::typ::int(),
                        meta: Default::default(),
                        value: 2,
                    }),
                },
            },
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                public: false,
                args: vec![],
                name: "bin_op".to_string(),
                body: Expr::BinOp {
                    meta: Default::default(),
                    typ: crate::typ::int(),
                    name: BinOp::AddInt,
                    left: Box::new(Expr::Int {
                        typ: crate::typ::int(),
                        meta: Default::default(),
                        value: 1,
                    }),
                    right: Box::new(Expr::Int {
                        typ: crate::typ::int(),
                        meta: Default::default(),
                        value: 2,
                    }),
                },
            },
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                public: false,
                args: vec![],
                name: "enum1".to_string(),
                body: Expr::Var {
                    meta: Default::default(),
                    constructor: ValueConstructor {
                        public: true,
                        origin: Default::default(),
                        typ: crate::typ::int(),
                        variant: ValueConstructorVariant::Record {
                            name: "Nil".to_string(),
                            field_map: None,
                            arity: 0,
                        },
                    },
                    name: "Nil".to_string(),
                },
            },
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                public: false,
                args: vec![],
                name: "let".to_string(),
                body: Expr::Let {
                    meta: Default::default(),
                    typ: crate::typ::int(),
                    value: Box::new(Expr::Int {
                        typ: crate::typ::int(),
                        meta: Default::default(),
                        value: 1,
                    }),
                    pattern: Pattern::Var {
                        meta: Default::default(),
                        name: "OneTwo".to_string(),
                    },
                    then: Box::new(Expr::Var {
                        meta: Default::default(),
                        constructor: ValueConstructor {
                            public: true,
                            origin: Default::default(),
                            typ: crate::typ::int(),
                            variant: ValueConstructorVariant::LocalVariable,
                        },
                        name: "one_two".to_string(),
                    }),
                },
            },
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                public: false,
                args: vec![],
                name: "conny".to_string(),
                body: Expr::Cons {
                    meta: Default::default(),
                    typ: crate::typ::int(),
                    head: Box::new(Expr::Int {
                        typ: crate::typ::int(),
                        meta: Default::default(),
                        value: 12,
                    }),
                    tail: Box::new(Expr::Cons {
                        meta: Default::default(),
                        typ: crate::typ::int(),
                        head: Box::new(Expr::Int {
                            typ: crate::typ::int(),
                            meta: Default::default(),
                            value: 34,
                        }),
                        tail: Box::new(Expr::Nil {
                            meta: Default::default(),
                            typ: crate::typ::int(),
                        }),
                    }),
                },
            },
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                public: false,
                args: vec![],
                name: "funny".to_string(),
                body: Expr::Fn {
                    meta: Default::default(),
                    is_capture: false,
                    typ: crate::typ::int(),
                    args: vec![
                        Arg {
                            meta: Meta { start: 0, end: 0 },
                            annotation: None,
                            names: ArgNames::Named {
                                name: "one_really_long_arg_to_cause_wrapping".to_string(),
                            },
                        },
                        Arg {
                            meta: Meta { start: 0, end: 0 },
                            annotation: None,
                            names: ArgNames::Named {
                                name: "also_really_quite_long".to_string(),
                            },
                        },
                    ],
                    body: Box::new(Expr::Int {
                        typ: crate::typ::int(),
                        meta: Default::default(),
                        value: 100000000000,
                    }),
                },
            },
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                public: false,
                args: vec![],
                name: "tup".to_string(),
                body: Expr::Tuple {
                    meta: Default::default(),
                    typ: crate::typ::int(),
                    elems: vec![
                        Expr::Int {
                            typ: crate::typ::int(),
                            meta: Default::default(),
                            value: 1,
                        },
                        Expr::Float {
                            meta: Default::default(),
                            typ: crate::typ::float(),
                            value: 2.0,
                        },
                    ],
                },
            },
        ],
    };
    let expected = "-module(term).
-compile(no_auto_import).

int() ->
    176.

float() ->
    11177.324401.

nil() ->
    [].

string() ->
    <<\"Hello there!\">>.

seq() ->
    1,
    2.

bin_op() ->
    1 + 2.

enum1() ->
    nil.

let() ->
    OneTwo = 1,
    OneTwo.

conny() ->
    [12, 34].

funny() ->
    fun(OneReallyLongArgToCauseWrapping, AlsoReallyQuiteLong) ->
        100000000000
    end.

tup() ->
    {1, 2.0}.
"
    .to_string();
    assert_eq!(expected, module(m));

    let m = Module {
        type_info: crate::typ::Module {
            name: vec!["term".to_string()],
            types: HashMap::new(),
            values: HashMap::new(),
        },
        name: vec!["term".to_string()],
        statements: vec![Statement::Fn {
            return_annotation: None,
            meta: Default::default(),
            public: false,
            name: "some_function".to_string(),
            args: vec![
                Arg {
                    meta: Meta { start: 0, end: 0 },
                    annotation: None,
                    names: ArgNames::Named {
                        name: "arg_one".to_string(),
                    },
                },
                Arg {
                    meta: Meta { start: 0, end: 0 },
                    annotation: None,
                    names: ArgNames::Named {
                        name: "arg_two".to_string(),
                    },
                },
                Arg {
                    meta: Meta { start: 0, end: 0 },
                    annotation: None,
                    names: ArgNames::Named {
                        name: "arg_3".to_string(),
                    },
                },
                Arg {
                    meta: Meta { start: 0, end: 0 },
                    annotation: None,
                    names: ArgNames::Named {
                        name: "arg4".to_string(),
                    },
                },
                Arg {
                    meta: Meta { start: 0, end: 0 },
                    annotation: None,
                    names: ArgNames::Named {
                        name: "arg_four".to_string(),
                    },
                },
                Arg {
                    meta: Meta { start: 0, end: 0 },
                    annotation: None,
                    names: ArgNames::Named {
                        name: "arg__five".to_string(),
                    },
                },
                Arg {
                    meta: Meta { start: 0, end: 0 },
                    annotation: None,
                    names: ArgNames::Named {
                        name: "arg_six".to_string(),
                    },
                },
                Arg {
                    meta: Meta { start: 0, end: 0 },
                    annotation: None,
                    names: ArgNames::Named {
                        name: "arg_that_is_long".to_string(),
                    },
                },
            ],
            body: Expr::Int {
                typ: crate::typ::int(),
                meta: Default::default(),
                value: 1,
            },
        }],
    };
    let expected = "-module(term).
-compile(no_auto_import).

some_function(
    ArgOne,
    ArgTwo,
    Arg3,
    Arg4,
    ArgFour,
    ArgFive,
    ArgSix,
    ArgThatIsLong
) ->
    1.
"
    .to_string();
    assert_eq!(expected, module(m));

    let m = Module {
        type_info: crate::typ::Module {
            name: vec!["ok".to_string()],
            types: HashMap::new(),
            values: HashMap::new(),
        },
        name: vec!["vars".to_string()],
        statements: vec![
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                public: false,
                args: vec![],
                name: "arg".to_string(),
                body: Expr::Var {
                    meta: Default::default(),
                    name: "some_arg".to_string(),
                    constructor: ValueConstructor {
                        public: true,
                        origin: Default::default(),
                        typ: crate::typ::int(),
                        variant: ValueConstructorVariant::LocalVariable,
                    },
                },
            },
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                public: false,
                args: vec![],
                name: "moddy".to_string(),
                body: Expr::ModuleSelect {
                    typ: crate::typ::Type::Fn {
                        args: vec![],
                        retrn: Box::new(crate::typ::int()),
                    },
                    meta: Default::default(),
                    module_alias: "zero".to_string(),
                    module_name: vec!["one".to_string()],
                    label: "two".to_string(),
                    constructor: ModuleValueConstructor::Fn,
                },
            },
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                public: false,
                args: vec![],
                name: "moddy2".to_string(),
                body: Expr::ModuleSelect {
                    typ: crate::typ::Type::Fn {
                        args: vec![crate::typ::int(), crate::typ::int()],
                        retrn: Box::new(crate::typ::int()),
                    },
                    meta: Default::default(),
                    module_alias: "zero".to_string(),
                    module_name: vec!["one".to_string(), "zero".to_string()],
                    label: "two".to_string(),
                    constructor: ModuleValueConstructor::Fn,
                },
            },
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                public: false,
                args: vec![],
                name: "moddy4".to_string(),
                body: Expr::Call {
                    meta: Default::default(),
                    typ: crate::typ::int(),
                    args: vec![CallArg {
                        label: None,
                        meta: Default::default(),
                        value: Expr::Int {
                            meta: Default::default(),
                            typ: crate::typ::int(),
                            value: 1,
                        },
                    }],
                    fun: Box::new(Expr::ModuleSelect {
                        typ: crate::typ::int(),
                        meta: Default::default(),
                        module_alias: "zero".to_string(),
                        module_name: vec!["one".to_string(), "zero".to_string()],
                        label: "two".to_string(),
                        constructor: ModuleValueConstructor::Fn,
                    }),
                },
            },
        ],
    };
    let expected = "-module(vars).
-compile(no_auto_import).

arg() ->
    SomeArg.

moddy() ->
    fun one:two/0.

moddy2() ->
    fun one@zero:two/2.

moddy4() ->
    one@zero:two(1).
"
    .to_string();
    assert_eq!(expected, module(m));

    let m = Module {
        type_info: crate::typ::Module {
            name: vec!["my_mod".to_string()],
            types: HashMap::new(),
            values: HashMap::new(),
        },
        name: vec!["my_mod".to_string()],
        statements: vec![Statement::Fn {
            return_annotation: None,
            meta: Default::default(),
            public: false,
            args: vec![],
            name: "go".to_string(),
            body: Expr::Case {
                meta: Default::default(),
                typ: crate::typ::int(),
                subjects: vec![Expr::Int {
                    typ: crate::typ::int(),
                    meta: Default::default(),
                    value: 1,
                }],
                clauses: vec![
                    Clause {
                        meta: Default::default(),
                        patterns: vec![Pattern::Int {
                            meta: Default::default(),
                            value: 1,
                        }],
                        then: Expr::Int {
                            typ: crate::typ::int(),
                            meta: Default::default(),
                            value: 1,
                        },
                    },
                    Clause {
                        meta: Default::default(),
                        patterns: vec![Pattern::Float {
                            meta: Default::default(),
                            value: 1.0,
                        }],
                        then: Expr::Int {
                            typ: crate::typ::int(),
                            meta: Default::default(),
                            value: 1,
                        },
                    },
                    Clause {
                        meta: Default::default(),
                        patterns: vec![Pattern::String {
                            meta: Default::default(),
                            value: "hello".to_string(),
                        }],
                        then: Expr::Int {
                            typ: crate::typ::int(),
                            meta: Default::default(),
                            value: 1,
                        },
                    },
                    Clause {
                        meta: Default::default(),
                        patterns: vec![Pattern::Nil {
                            meta: Default::default(),
                        }],
                        then: Expr::Int {
                            typ: crate::typ::int(),
                            meta: Default::default(),
                            value: 1,
                        },
                    },
                    Clause {
                        meta: Default::default(),
                        patterns: vec![Pattern::Constructor {
                            meta: Default::default(),
                            module: None,
                            name: "Error".to_string(),
                            args: vec![CallArg {
                                label: None,
                                meta: Default::default(),
                                value: Pattern::Int {
                                    meta: Default::default(),
                                    value: 2,
                                },
                            }],
                            constructor: PatternConstructor::Record {
                                name: "Error".to_string(),
                            },
                        }],
                        then: Expr::Int {
                            typ: crate::typ::int(),
                            meta: Default::default(),
                            value: 1,
                        },
                    },
                    Clause {
                        meta: Default::default(),
                        patterns: vec![Pattern::Tuple {
                            meta: Default::default(),
                            elems: vec![
                                Pattern::Int {
                                    meta: Default::default(),
                                    value: 1,
                                },
                                Pattern::Int {
                                    meta: Default::default(),
                                    value: 2,
                                },
                            ],
                        }],
                        then: Expr::Int {
                            typ: crate::typ::int(),
                            meta: Default::default(),
                            value: 1,
                        },
                    },
                ],
            },
        }],
    };
    let expected = "-module(my_mod).
-compile(no_auto_import).

go() ->
    case 1 of
        1 ->
            1;

        1.0 ->
            1;

        <<\"hello\">> ->
            1;

        [] ->
            1;

        {error, 2} ->
            1;

        {1, 2} ->
            1
    end.
"
    .to_string();
    assert_eq!(expected, module(m));

    let m = Module {
        type_info: crate::typ::Module {
            name: vec!["funny".to_string()],
            types: HashMap::new(),
            values: HashMap::new(),
        },
        name: vec!["funny".to_string()],
        statements: vec![
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                args: vec![],
                name: "one".to_string(),
                public: false,
                body: Expr::Call {
                    meta: Default::default(),
                    typ: crate::typ::int(),
                    args: vec![CallArg {
                        label: None,
                        meta: Default::default(),
                        value: Expr::Int {
                            typ: crate::typ::int(),
                            meta: Default::default(),
                            value: 1,
                        },
                    }],
                    fun: Box::new(Expr::Var {
                        meta: Default::default(),
                        constructor: ValueConstructor {
                            public: true,
                            origin: Default::default(),
                            variant: ValueConstructorVariant::ModuleFn {
                                name: "one_two".to_string(),
                                module: vec!["funny".to_string()],
                                field_map: None,
                                arity: 1,
                            },
                            typ: crate::typ::int(),
                        },
                        name: "one_two".to_string(),
                    }),
                },
            },
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                args: vec![],
                name: "two".to_string(),
                public: false,
                body: Expr::Call {
                    meta: Default::default(),
                    typ: crate::typ::int(),
                    args: vec![CallArg {
                        label: None,
                        meta: Default::default(),
                        value: Expr::Int {
                            typ: crate::typ::int(),
                            meta: Default::default(),
                            value: 1,
                        },
                    }],
                    fun: Box::new(Expr::Var {
                        meta: Default::default(),
                        constructor: ValueConstructor {
                            public: true,
                            origin: Default::default(),
                            variant: ValueConstructorVariant::LocalVariable,
                            typ: crate::typ::int(),
                        },
                        name: "one_two".to_string(),
                    }),
                },
            },
            Statement::Fn {
                return_annotation: None,
                meta: Default::default(),
                args: vec![],
                name: "three".to_string(),
                public: false,
                body: Expr::Call {
                    meta: Default::default(),
                    typ: crate::typ::int(),
                    args: vec![CallArg {
                        label: None,
                        meta: Default::default(),
                        value: Expr::Int {
                            typ: crate::typ::int(),
                            meta: Default::default(),
                            value: 2,
                        },
                    }],
                    fun: Box::new(Expr::Call {
                        meta: Default::default(),
                        typ: crate::typ::int(),
                        args: vec![CallArg {
                            label: None,
                            meta: Default::default(),
                            value: Expr::Int {
                                typ: crate::typ::int(),
                                meta: Default::default(),
                                value: 1,
                            },
                        }],
                        fun: Box::new(Expr::Var {
                            meta: Default::default(),
                            constructor: ValueConstructor {
                                public: true,
                                origin: Default::default(),
                                typ: crate::typ::int(),
                                variant: ValueConstructorVariant::ModuleFn {
                                    name: "one_two_actual".to_string(),
                                    module: vec!["funny".to_string()],
                                    field_map: None,
                                    arity: 2,
                                },
                            },
                            name: "one_two_alias".to_string(),
                        }),
                    }),
                },
            },
        ],
    };
    let expected = "-module(funny).
-compile(no_auto_import).

one() ->
    one_two(1).

two() ->
    OneTwo(1).

three() ->
    (one_two_actual(1))(2).
"
    .to_string();
    assert_eq!(expected, module(m));
}

#[test]
fn integration_test() {
    struct Case {
        src: &'static str,
        erl: &'static str,
    }

    let cases = [
        Case {
            src: r#"fn go() {
let x = tuple(100000000000000000, tuple(2000000000, 3000000000000, 40000000000), 50000, 6000000000)
  x
}"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

go() ->
    X = {100000000000000000,
         {2000000000, 3000000000000, 40000000000},
         50000,
         6000000000},
    X.
"#,
        },
        Case {
            src: r#"fn go() {
  let y = 1
  let y = 2
  y
}"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

go() ->
    Y = 1,
    Y1 = 2,
    Y1.
"#,
        },
        Case {
            src: r#"pub fn t() { True }"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

-export([t/0]).

t() ->
    true.
"#,
        },
        Case {
            src: r#"pub type Money { Pound(Int) }
                    fn pound(x) { Pound(x) }"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

pound(X) ->
    {pound, X}.
"#,
        },
        Case {
            src: r#"fn loop() { loop() }"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

loop() ->
    loop().
"#,
        },
        Case {
            src: r#"external fn run() -> Int = "Elixir.MyApp" "run""#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

run() ->
    'Elixir.MyApp':run().
"#,
        },
        Case {
            src: r#"fn inc(x) { x + 1 }
                    pub fn go() { 1 |> inc |> inc |> inc }"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

-export([go/0]).

inc(X) ->
    X + 1.

go() ->
    inc(inc(inc(1))).
"#,
        },
        Case {
            src: r#"fn add(x, y) { x + y }
                    pub fn go() { 1 |> add(_, 1) |> add(2, _) |> add(_, 3) }"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

-export([go/0]).

add(X, Y) ->
    X + Y.

go() ->
    add(add(2, add(1, 1)), 3).
"#,
        },
        Case {
            src: r#"fn and(x, y) { x && y }
                    fn or(x, y) { x || y }
                    fn modulo(x, y) { x % y }
            "#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

'and'(X, Y) ->
    X andalso Y.

'or'(X, Y) ->
    X orelse Y.

modulo(X, Y) ->
    X rem Y.
"#,
        },
        Case {
            src: r#"fn second(list) { case list { [x, y] -> y z -> 1 } }
                    fn tail(list) { case list { [x | xs] -> xs z -> list } }
            "#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

second(List) ->
    case List of
        [X, Y] ->
            Y;

        Z ->
            1
    end.

tail(List) ->
    case List of
        [X | Xs] ->
            Xs;

        Z ->
            List
    end.
"#,
        },
        Case {
            src: r#"fn x() { let x = 1 let x = x + 1 x }"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

x() ->
    X = 1,
    X1 = X + 1,
    X1.
"#,
        },
        Case {
            src: r#"pub external fn receive() -> Int = "try" "and"
                    pub fn catch(x) { receive() }"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

-export(['receive'/0, 'catch'/1]).

'receive'() ->
    'try':'and'().

'catch'(X) ->
    'receive'().
"#,
        },
        // Translation of Float-specific BinOp into variable-type Erlang term comparison.
        Case {
            src: r#"fn x() { 1. <. 2.3 }"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

x() ->
    1.0 < 2.3.
"#,
        },
        // Custom type creation
        Case {
            src: r#"type Pair(x, y) { Pair(x: x, y: y) } fn x() { Pair(1, 2) Pair(3., 4.) }"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

x() ->
    {pair, 1, 2},
    {pair, 3.0, 4.0}.
"#,
        },
        Case {
            src: r#"type Null { Null } fn x() { Null }"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

x() ->
    null.
"#,
        },
        Case {
            src: r#"type Point { Point(x: Int, y: Int) }
                fn y() { fn() { Point }()(4, 6) }"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

y() ->
    ((fun() -> fun(A, B) -> {point, A, B} end end)())(4, 6).
"#,
        },
        Case {
            src: r#"type Point { Point(x: Int, y: Int) }
                fn x() { Point(x: 4, y: 6) Point(y: 1, x: 9) }"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

x() ->
    {point, 4, 6},
    {point, 9, 1}.
"#,
        },
        Case {
            src: r#"type Point { Point(x: Int, y: Int) } fn x(y) { let Point(a, b) = y a }"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

x(Y) ->
    {point, A, B} = Y,
    A.
"#,
        },
        Case {
            src: r#"external fn go(x: Int, y: Int) -> Int = "m" "f"
                    fn x() { go(x: 1, y: 2) go(y: 3, x: 4) }"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

go(A, B) ->
    m:f(A, B).

x() ->
    go(1, 2),
    go(4, 3).
"#,
        },
        Case {
            src: r#"fn go(x xx, y yy) { xx }
                    fn x() { go(x: 1, y: 2) go(y: 3, x: 4) }"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

go(Xx, Yy) ->
    Xx.

x() ->
    go(1, 2),
    go(4, 3).
"#,
        },
        // https://github.com/lpil/gleam/issues/289
        Case {
            src: r#"
type User { User(id: Int, name: String, age: Int) }
fn create_user(user_id) { User(age: 22, id: user_id, name: "") }
                    "#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

create_user(UserId) ->
    {user, UserId, <<"">>, 22}.
"#,
        },
        Case {
            src: r#"fn run() { case 1, 2 { a, b -> a } }"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

run() ->
    case {1, 2} of
        {A, B} ->
            A
    end.
"#,
        },
        Case {
            src: r#"type X { X(x: Int, y: Float) }
                    fn x() { X(x: 1, y: 2.) X(y: 3., x: 4) }"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

x() ->
    {x, 1, 2.0},
    {x, 4, 3.0}.
"#,
        },
        // https://github.com/gleam-lang/gleam/issues/333
        Case {
            src: r#"
fn go(a) {
  case a {
    99 -> {
      let a = a
      1
    }
    _ -> a
  }
}

                    "#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

go(A) ->
    case A of
        99 ->
            A1 = A,
            1;

        _ ->
            A
    end.
"#,
        },
        Case {
            src: r#"
fn go(a) {
  let a = a + 1
  a
}

                    "#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

go(A) ->
    A1 = A + 1,
    A1.
"#,
        },
        Case {
            src: r#"
fn go(a) {
  let a = 1
  a
}

                    "#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

go(A) ->
    A1 = 1,
    A1.
"#,
        },
        Case {
            src: r#"
fn id(x) {
  x
}

fn main() {
  id(id)
}
"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

id(X) ->
    X.

main() ->
    id(fun id/1).
"#,
        },
        // https://github.com/gleam-lang/gleam/issues/358
        Case {
            src: r#"
pub fn factory(f, i) {
  f(i)
}

pub type Box {
  Box(i: Int)
}

pub fn main() {
  factory(Box, 0)
}
"#,
            erl: r#"-module(the_app).
-compile(no_auto_import).

-export([factory/2, main/0]).

factory(F, I) ->
    F(I).

main() ->
    factory(fun(A) -> {box, A} end, 0).
"#,
        },
    ];

    for Case { src, erl } in cases.into_iter() {
        let mut ast = crate::grammar::ModuleParser::new()
            .parse(src)
            .expect("syntax error");
        ast.name = vec!["the_app".to_string()];
        let ast = crate::typ::infer_module(ast, &std::collections::HashMap::new())
            .expect("should successfully infer");
        let output = module(ast);
        assert_eq!((src, output), (src, erl.to_string()));
    }
}
