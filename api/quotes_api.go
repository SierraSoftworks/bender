package api

import (
	"strings"

	"github.com/SierraSoftworks/girder"
	"github.com/SierraSoftworks/girder/errors"
)

func init() {
	addRegistrar(func(a *API) error {
		a.Router().
			Methods("GET").
			Path("/api/v1/quote").
			Name("Get Quote").
			Handler(girder.NewHandler(a.getQuote).
				LogRequests().
				RegisterPreprocessors(SentryRequestLogger))

		a.Router().
			Methods("GET").
			Path("/api/v1/quote/{who}").
			Name("Get Quote By").
			Handler(girder.NewHandler(a.getQuoteBy).
				LogRequests().
				RegisterPreprocessors(SentryRequestLogger))

		return nil
	})
}

func setOutputFormat(c *girder.Context) {
	accept := c.Request.Header.Get("Accept")
	if strings.Contains(accept, "application/json") {
		c.Formatter = &girder.JSONFormatter{}
		c.ResponseHeaders.Set("Content-Type", "application/json; charset=utf8")
	} else if strings.Contains(accept, "text/html") {
		c.Formatter = &HtmlFormatter{}
		c.ResponseHeaders.Set("Content-Type", "text/html; charset=utf8")
	} else if strings.Contains(accept, "text/plain") {
		c.Formatter = &TextFormatter{}
		c.ResponseHeaders.Set("Content-Type", "text/plain; charset=utf8")
	}
}

func (a *API) getQuote(c *girder.Context) (interface{}, error) {
	quote := a.Quotes().GetRandom()
	if quote == nil {
		return nil, errors.NotFound()
	}

	setOutputFormat(c)

	return quote, nil
}

func (a *API) getQuoteBy(c *girder.Context) (interface{}, error) {
	who := c.Vars["who"]
	quote := a.Quotes().GetRandomBy(who)
	if quote == nil {
		return nil, errors.NotFound()
	}

	setOutputFormat(c)

	return quote, nil
}
