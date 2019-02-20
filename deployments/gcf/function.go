package bender_gcf

import (
	"net/http"
	"os"
	"sync"

	"github.com/SierraSoftworks/bender/pkg/bender"
	sentry "github.com/SierraSoftworks/sentry-go"
	log "github.com/sirupsen/logrus"
)

var scl = sentry.NewClient(
	sentry.DSN("https://0f9ec16cd0e2473bb994e7108d951b86@sentry.io/1362607"),
	sentry.Environment("cloudfunctions"),
)

type FunctionService struct {
	App *bender.App

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

	app, err := bender.NewApp()
	if err != nil {
		return err
	}

	s.App = app

	quotesFile := os.Getenv("QUOTES_FILE")
	if quotesFile == "" {
		quotesFile = "gs://cdn.sierrasoftworks.com/bender/quotes.json"
	}

	if err := app.LoadQuotes(quotesFile); err != nil {
		return err
	}

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
		scl.Capture(sentry.ExceptionForError(err), sentry.Level(sentry.Fatal)).Wait()
		return
	}

	service.App.Handler().ServeHTTP(w, r)

	scl.Capture(sentry.Message("API called"), sentry.Level(sentry.Debug)).Wait()
}
