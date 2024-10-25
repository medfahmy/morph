```
main = {} -> {
    tracing_subscriber
        .fmt 
        .fmt {} 
        .with_max_level { tracing.Level.DEBUG } 
        .init {};

    db = self.open { "database" }.try {};
    tracing.info! { "connected to database" };

    cors = middleware.cors.CorsLayer.new {}
        .allow_origin { "http://localhost:8080".parse {}.try {} }
        .allow_methods { [ Method.GET, Method.POST, Method.PATCH, Method.DELETE ] };

    trace = middleware.trace.TraceLayer.new_for_http {}
        .make_span_with { { request } -> 
            tracing.info_span! {
                "request",
                method = ?request.method {},
                uri = %request.uri {},
                request_id = uuid.Uuid.new_v4 {}.to_string {},
            }
        };

    router = http.Router.new {}
        .route { "/", axum.routing.get { health } };

    http.serve { router, "http://localhost:8080" }
        .try {}
}

```
