use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
enum Method {
    Get,
    Post,
}

type Handler = fn(Request) -> Response;

#[derive(PartialEq, Eq, Debug)]
struct Request {
    pub method: Method,
    pub uri: String,
    pub headers: HashMap<String, String>,
}

#[derive(PartialEq, Eq, Debug)]
struct Response {
}


#[derive(PartialEq, Eq, Debug)]
struct Route {
    matcher: Matcher,
    handler: Handler,
}
#[derive(PartialEq, Eq, Debug)]
enum Matcher {
    Static(Method, String),
}

// TODO return Optional<Parsed Content>?
fn matches(matcher: Matcher, request: Request) -> bool {
    use Matcher::*;
    match matcher {
        Static(method, uri) => request.method == method && request.uri == uri
    }
}

impl Default for Request {
    fn default() -> Request {
        Request {
            method: Method::Get,
            uri: "/".to_string(),
            headers: Default::default(),
        }
    }
}


#[derive(PartialEq, Eq, Debug)]
struct RouterBuilder {
    routes: Vec<Route>,
}

// TODO split resolve and handle?
impl RouterBuilder {
    fn empty() -> RouterBuilder {
        RouterBuilder { routes: Vec::new() }
    }
    fn build(&self) -> Router {
        Router {}
    }
    fn get(mut self, route: &str, handler: Handler) -> Self {
        self.routes.push(
            Route {
                matcher: Matcher::Static(Method::Get, String::from(route)),
                handler,
            }
        );
        self
    }
}
struct Router {
}
impl Router {
    fn handle(&self, request: Request) -> Option<Response> {
        None
    }
}
fn handler(request: Request) -> Response {
    Response {}
}

#[cfg(test)]
mod tests {
    use ::*;
    #[test]
    fn simple() {
        let mut router = RouterBuilder::empty()
            .get("/", handler)
            .get("/ping", handler)
            .build();
        assert_eq!(
            router.handle(Default::default()),
            Some(Response {}),
        );
    }
}
