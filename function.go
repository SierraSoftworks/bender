package bender

import (
	"net/http"
	"os"

	"github.com/SierraSoftworks/bender/internal/pkg/api"
	"github.com/SierraSoftworks/bender/internal/pkg/loaders"
	"github.com/SierraSoftworks/bender/internal/pkg/providers"
	sentry "github.com/SierraSoftworks/sentry-go"
	log "github.com/sirupsen/logrus"
)

func error500(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(500)
	w.Write([]byte(`{"code": 500, "error": "Server Error", "message": "The server was unable to satisfy the request." }`))
}

var scl = sentry.NewClient(
	sentry.DSN("https://0f9ec16cd0e2473bb994e7108d951b86@sentry.io/1362607"),
	sentry.Environment("cloudfunctions"),
)

var quotes = providers.NewQuoteProvider()

func init() {
	if os.Getenv("QUOTES_FILE") == "" {
		os.Setenv("QUOTES_FILE", "gs://cdn.sierrasoftworks.com/bender/quotes.json")
	}

	loader := loaders.New(os.Getenv("QUOTES_FILE"))
	if loader == nil {
		scl.Capture(
			sentry.Message("Invalid quotes file path"),
			sentry.Context("quotesFile", os.Getenv("QUOTES_FILE")),
			sentry.Level(sentry.Fatal),
		)

		log.
			WithField("quotes", os.Getenv("QUOTES_FILE")).
			Fatal("No quote loader available for quote file path")
	}

	if err := quotes.Load(loader); err != nil {
		scl.Capture(
			sentry.ExceptionForError(err),
			sentry.Context("quotesFile", os.Getenv("QUOTES_FILE")),
			sentry.Level(sentry.Fatal),
		)
		log.WithError(err).Fatal()
	}
}

func Bender(w http.ResponseWriter, r *http.Request) {

	api, err := api.NewAPI(quotes)
	if err != nil {
		log.WithError(err).Error("Failed to initialize the API")
		scl.Capture(
			sentry.ExceptionForError(err),
			sentry.Level(sentry.Fatal),
		)
		error500(w, r)
		return
	}

	api.Handler().ServeHTTP(w, r)

	scl.Capture(sentry.Message("API called"), sentry.Level(sentry.Debug)).Wait()
}
