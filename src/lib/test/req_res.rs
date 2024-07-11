#[cfg(test)]
mod req_res {

    use crate::{
        test::util::{from_open_api_test_fn, gen_test, to_open_api_test_fn},
        Generator, Role,
    };

    #[test]
    fn to_open_api_test() {
        to_open_api_test_fn("req-res");
    }

    #[test]
    fn from_open_api_test() {
        from_open_api_test_fn("req-res");
    }

    #[test]
    fn gen_python_client_test() {
        gen_test(
            Generator::Python,
            Role::Client,
            "req-res-trust.yml".to_string(),
        );
    }

    #[test]
    fn gen_python_server_test() {
        gen_test(
            Generator::Python,
            Role::Server,
            "req-res-trust.yml".to_string(),
        );
    }
}
