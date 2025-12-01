use crate::models::Quote;

use actix_web::{
    HttpRequest, HttpResponse, Responder, body::BoxBody, http::header, http::header::Header,
};
use tracing_batteries::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct QuoteV1 {
    pub quote: String,
    pub who: String,
}

impl From<QuoteV1> for Quote {
    fn from(val: QuoteV1) -> Self {
        Quote {
            quote: val.quote.clone(),
            who: val.who,
        }
    }
}

impl From<Quote> for QuoteV1 {
    fn from(state: Quote) -> Self {
        Self {
            quote: state.quote.clone(),
            who: state.who,
        }
    }
}

impl Responder for QuoteV1 {
    type Body = BoxBody;

    #[tracing::instrument(target="response.render", fields(http.content_type = tracing::field::Empty), skip(self, req))]
    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let content_type = header::Accept::parse(req)
            .map(|header| {
                for a in header.iter() {
                    if a.item.essence_str() == "application/json" {
                        return "application/json";
                    } else if a.item.essence_str() == "text/html" {
                        return "text/html";
                    } else if a.item.essence_str() == "text/plain" {
                        return "text/plain";
                    }
                }

                "application/json"
            })
            .unwrap_or("application/json");

        Span::current().record("http.content_type", display(content_type));

        info!({ http.content_type = %content_type}, "Rendering quote");

        match content_type {
            "text/plain" => HttpResponse::Ok()
                .content_type(content_type)
                .body(format!("{} – {}", self.quote, self.who)),
            "text/html" => HttpResponse::Ok().content_type(content_type).body(format!(
                "
                <html>
                    <head>
                        <style>
                            body {{
                                font-family: Sans-serif;
                            }}
                
                            figure {{
                                margin: 20px;
                            }}
                
                            blockquote {{
                                margin-left: 1em;
                            }}
                
                            figcaption {{
                                margin-left: 2em;
                                font-size: 0.8em;
                                font-weight: bold;
                            }}
                
                            figcaption::before {{
                                display: inline;
                                content: \"\\2013\";
                                padding-right: 0.5em;
                            }}
                        </style>
                        <title>Bender as a Service</title>
                    </head>
                    <body>
                        <figure>
                            <blockquote>{0}</blockquote>
                            <figcaption>{1}</figcaption>
                        </figure>
                    </body>
                </html>",
                self.quote, self.who
            )),
            _ => HttpResponse::Ok().content_type(content_type).json(&self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;

    #[actix_rt::test]
    async fn quote_text() {
        let quote = QuoteV1 {
            quote: "This is a test".to_string(),
            who: "Tester".to_string(),
        };

        let request = TestRequest::default()
            .insert_header((header::ACCEPT, "text/plain; charset=utf-8"))
            .to_http_request();

        let resp = quote.respond_to(&request);
        assert_eq!(resp.headers().get("Content-Type").unwrap(), "text/plain");
    }

    #[actix_rt::test]
    async fn quote_html() {
        let quote = QuoteV1 {
            quote: "This is a test".to_string(),
            who: "Tester".to_string(),
        };

        let request = TestRequest::default()
            .insert_header((header::ACCEPT, "text/html; charset=utf-8"))
            .to_http_request();

        let resp = quote.respond_to(&request);
        assert_eq!(resp.headers().get("Content-Type").unwrap(), "text/html");
    }

    #[actix_rt::test]
    async fn quote_json() {
        let quote = QuoteV1 {
            quote: "This is a test".to_string(),
            who: "Tester".to_string(),
        };

        let request = TestRequest::default()
            .insert_header((header::ACCEPT, "application/json; charset=utf-8"))
            .to_http_request();

        let resp = quote.respond_to(&request);
        assert_eq!(
            resp.headers().get("Content-Type").unwrap(),
            "application/json"
        );
    }

    #[actix_rt::test]
    async fn quote_other() {}
}
