package bender

import (
	"net/http"
	"os"
	"sync"

	"github.com/pkg/errors"

	"github.com/SierraSoftworks/bender/internal/pkg/api"
	"github.com/SierraSoftworks/bender/internal/pkg/loaders"
	"github.com/SierraSoftworks/bender/internal/pkg/providers"
	sentry "github.com/SierraSoftworks/sentry-go"
	log "github.com/sirupsen/logrus"
)

var scl = sentry.NewClient(
	sentry.DSN("https://0f9ec16cd0e2473bb994e7108d951b86@sentry.io/1362607"),
	sentry.Environment("cloudfunctions"),
)

type FunctionService struct {
	Quotes *providers.QuoteProvider
	API    *api.API

	initialized bool
	m           sync.RWMutex
}

func (s *FunctionService) Initialize() error {
	s.m.RLock()
	if s.initialized {
		s.m.RUnlock()
		return nil
	}

	s.m.RUnlock()
	s.m.Lock()
	defer s.m.Unlock()
	if s.initialized {
		return nil
	}

	quotesFile := os.Getenv("QUOTES_FILE")
	if quotesFile == "" {
		quotesFile = "gs://cdn.sierrasoftworks.com/bender/quotes.json"
	}

	loader := loaders.New(quotesFile)
	if loader == nil {
		scl.Capture(
			sentry.Message("Invalid quotes file path"),
			sentry.Context("quotesFile", quotesFile),
			sentry.Level(sentry.Fatal),
		).Wait()

		log.
			WithField("quotes", os.Getenv("QUOTES_FILE")).
			Error("No quote loader available for quote file path")

		return errors.New("bender: unknown quote file path format")
	}

	s.Quotes = providers.NewQuoteProvider()

	if err := s.Quotes.Load(loader); err != nil {
		scl.Capture(
			sentry.ExceptionForError(err),
			sentry.Context("quotesFile", quotesFile),
			sentry.Level(sentry.Fatal),
		).Wait()
		return errors.Wrap(err, "bender: unable to load quotes")
	}

	apiHost, err := api.NewAPI(s.Quotes)
	if err != nil {
		scl.Capture(
			sentry.ExceptionForError(err),
			sentry.Context("quotesFile", quotesFile),
			sentry.Level(sentry.Fatal),
		).Wait()
		return errors.Wrap(err, "bender: unable to setup API")
	}

	s.API = apiHost

	s.initialized = true

	return nil
}

var service = &FunctionService{}

func Bender(w http.ResponseWriter, r *http.Request) {
	scl := scl.With(
		sentry.HTTPRequest(r),
	)

	if err := service.Initialize(); err != nil {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(500)
		w.Write([]byte(`{"code": 500, "error": "Server Error", "message": "The server was unable to satisfy the request." }`))

		log.WithError(err).Error("Failed to initialize the API")
		scl.Capture(sentry.Message("API was not initialized correctly"), sentry.ExceptionForError(err), sentry.Level(sentry.Fatal)).Wait()
		return
	}

	service.API.Handler().ServeHTTP(w, r)

	scl.Capture(sentry.Message("API called"), sentry.Level(sentry.Debug)).Wait()
}
