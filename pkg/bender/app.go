package bender

import (
	"net/http"

	"github.com/SierraSoftworks/bender/internal/pkg/api"
	"github.com/SierraSoftworks/bender/internal/pkg/loaders"
	"github.com/SierraSoftworks/bender/internal/pkg/providers"
	"github.com/pkg/errors"
)

type App struct {
	api    *api.API
	quotes *providers.QuoteProvider
}

func NewApp() (*App, error) {
	app := &App{
		quotes: providers.NewQuoteProvider(),
	}

	apiHost, err := api.NewAPI(app.quotes)
	if err != nil {
		return nil, errors.Wrap(err, "bender: failed to initialize API")
	}

	app.api = apiHost

	return app, nil
}

func (app *App) LoadQuotes(path string) error {
	loader := loaders.New(path)
	if loader == nil {
		return errors.New("bender: unrecognized quotes path format")
	}

	err := app.quotes.Load(loader)
	if err != nil {
		return errors.Wrap(err, "bender: failed to load quotes")
	}

	return nil
}

func (app *App) Handler() http.Handler {
	return app.api.Handler()
}

func (app *App) ListenAndServe(address string) error {
	return app.api.ListenAndServe(address)
}
