use futures::executor::block_on;
use futures::future::lazy;
use ufcs::Pipe;

async fn async_fn(s: String) -> String {
    lazy(|_| format!("a({})", s)).await
}

fn result_fn(s: String) -> Result<String, ()> {
    Ok(format!("r({})", s))
}

#[test]
fn simple() {
    assert_eq!("foo".pipe(Some), Some("foo"));
}

#[test]
fn chaining() {
    let a: Result<String, ()> = block_on(async {
        "foo"
            .to_string()
            .pipe(result_fn)?
            .pipe(|x| format!("c({})", x))
            .pipe(async_fn)
            .await
            .replace("f", "b")
            .pipe(Ok)
    });

    let b: Result<String, ()> = block_on(async {
        Ok(async_fn(format!("c({})", result_fn("foo".to_string())?))
            .await
            .replace("f", "b"))
    });

    assert_eq!(a, b);
    assert_eq!(a, Ok(String::from("a(c(r(boo)))")));
}
