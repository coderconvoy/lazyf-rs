
use flag;
use cfg;

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

#[test]
fn reader(){
    let f = cfg::Cfg::load("src/test_data/lztest.lz");

    match f {
        Ok(c)=>{
            assert_eq!( c.len(),2); 
        },
        Err(e)=>assert!(false,"Error {}",e),
    }

    //assert_eq!(r,Result::Err("poo".to_string()));
}
