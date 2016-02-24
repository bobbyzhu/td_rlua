extern crate td_rlua;
use td_rlua::Lua;
use td_rlua::LuaTable;

#[test]
fn basic() {
    let mut lua = Lua::new();
    let val: Option<i32> = lua.exec_string("return 5;");
    assert_eq!(val.unwrap(), 5);
}

#[test]
fn syntax_error() {
    let mut lua = Lua::new();
    let val : Option<()> = lua.exec_string("td_rlua");
    assert!(val.is_none());
}

#[test]
fn execution_error() {
    let mut lua = Lua::new();

   let val : Option<()> = lua.exec_string("return a:hello()");
   assert!(val.is_none());
}

#[test]
fn call_and_read_table() {
    let mut lua = Lua::new();

    let mut val: LuaTable = lua.exec_string("return {1, 2, 3};").unwrap();
    assert_eq!(val.query::<u8, _>(1).unwrap(), 1);
    assert_eq!(val.query::<u8, _>(2).unwrap(), 2);
    assert_eq!(val.query::<u8, _>(3).unwrap(), 3);
}


#[test]
fn simple_function() {
    let mut lua = Lua::new();

    fn ret5() -> i32 { 5 };
    lua.set("ret5", td_rlua::function0(ret5));

    let val: i32 = lua.exec_string("return ret5()").unwrap();
    assert_eq!(val, 5);
}

#[test]
fn one_argument() {
    let mut lua = Lua::new();

    fn plus_one(val: i32) -> i32 { val + 1 };
    lua.set("plus_one", td_rlua::function1(plus_one));

    let val: i32 = lua.exec_string("return plus_one(3)").unwrap();
    assert_eq!(val, 4);
}

#[test]
fn two_arguments() {
    let mut lua = Lua::new();

    fn add(val1: i32, val2: i32) -> i32 { val1 + val2 };
    lua.set("add", td_rlua::function2(add));

    let val: i32 = lua.exec_string("return add(3, 7)").unwrap();
    assert_eq!(val, 10);
}

#[test]
fn wrong_arguments_types() {
    let mut lua = Lua::new();

    fn add(val1: i32, val2: i32) -> i32 { val1 + val2 };
    lua.set("add", td_rlua::function2(add));
    let val : Option<i32> = lua.exec_string("return add(3, \"hello\")");
    match val {
        None => (),
        _ => panic!()
    }
}


#[test]
fn closures() {
    let mut lua = Lua::new();

    lua.set("add", td_rlua::function2(|a:i32, b:i32| a + b));
    lua.set("sub", td_rlua::function2(|a:i32, b:i32| a - b));

    let val1: i32 = lua.exec_string("return add(3, 7)").unwrap();
    assert_eq!(val1, 10);

    let val2: i32 = lua.exec_string("return sub(5, 2)").unwrap();
    assert_eq!(val2, 3);
}

#[test]
fn closures_lifetime() {
    fn t<F>(f: F) where F: Fn(i32, i32) -> i32 {
        let mut lua = Lua::new();

        lua.set("add", td_rlua::function2(f));

        let val1: i32 = lua.exec_string("return add(3, 7)").unwrap();
        assert_eq!(val1, 10);
    }

    t(|a, b| a + b);
}

#[test]
fn closures_extern_access() {
    let mut a = 5;

    {
        let mut lua = Lua::new();

        lua.set("inc", td_rlua::function0(|| a += 1));
        for _ in 0 .. 15 {
            let _: () = lua.exec_string("inc()").unwrap();
        }
    }

    assert_eq!(a, 20)
}
