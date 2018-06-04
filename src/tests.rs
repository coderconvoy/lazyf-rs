
use flag;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn args(){
    let v = ["-a","60"];
    let r = flag::ss_a("-a",&mut v.into_iter().map(|&a|String::from(a)));
    assert_eq!(r,Result::Ok("60".to_string()));
}
